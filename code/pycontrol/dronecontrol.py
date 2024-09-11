import socket
import time
import struct
import math

from commands_gen import *
from storage_gen import *
from typing import Callable
from threading import Thread, Lock

# DRONE_ADDR = "00:22:05:00:31:56"
DRONE_ADDR = "00:22:05:00:31:68"

COMMAND_INPUT_MAX = 50
COMMAND_INPUT = 10
THRUST_MAX = 3000

ANGLE_INPUT_MAX = 225

THRUST_VARIABLE_NAME = "thrust"
PITCH_VARIABLE_NAME = "pitch"
YAW_VARIABLE_NAME = "yaw"
ROLL_VARIABLE_NAME = "roll"

ziegler_coefficients = [
    [0.5,  0.0,  0.0],
    [0.45, 0.54, 0.0],
    [0.8,  0.0,  0.1],
    [0.6,  1.2,  0.075],
    [0.7,  1.75, 0.105],
    [1/3,  2/3,  1/9],
    [0.2,  0.4,  3/20]
]


def ziegler_calc(ku, tu, cs):
    return int(round(ziegler_coefficients[cs][0]*ku)), int(round(ziegler_coefficients[cs][1]*ku/tu)), int(round(ziegler_coefficients[cs][2]*ku*tu))


YAW_KU = 6000
YAW_TU = 0.5
YAW_KP, YAW_KI, YAW_KD = ziegler_calc(YAW_KU, YAW_TU, 1)

PITCH_KU = 1000
PITCH_TU = 0.25

PITCH_KP, PITCH_KI, PITCH_KD = ziegler_calc(PITCH_KU, PITCH_TU, 1)

ROLL_KU = 1000
ROLL_TU = 0.25

ROLL_KP, ROLL_KI, ROLL_KD = ziegler_calc(ROLL_KU, ROLL_TU, 1)


def dummy_setter(*args, **kwargs):
    pass


class DroneVariable:
    # owner: Drone
    name: str
    reset_trim: int
    input_step: int
    trim_step: int
    min_value: int
    max_value: int
    setter: Callable[[socket.socket, int], None]
    analog: int

    trim_value: int
    input_sign: int
    last_applied_value: int

    def __init__(self,
                 owner,  #: Drone
                 name: str,
                 initial_trim: int,
                 reset_trim: int,
                 input_step: int,
                 trim_step: int,
                 min_value: int,
                 max_value: int,
                 setter: Callable[[socket.socket, int], None]):
        self.owner = owner
        self.name = name
        self.reset_trim = reset_trim
        self.input_step = input_step
        self.trim_step = trim_step
        self.min_value = min_value
        self.max_value = max_value
        self.setter = setter

        self.trim_value = initial_trim
        self.input_sign = 0
        self.last_applied_value = None
        self.analog = 0

    def trim_increment(self, sign):
        old_value = self.trim_value
        new_trim = self.trim_value + self.trim_step * sign
        new_value = new_trim + self.input_sign * self.input_step
        if self.min_value <= new_value <= self.max_value:
            self.trim_value = new_trim
            self.apply_value()
        return old_value, self.trim_value

    def trim_set(self, new_trim):
        old_value = self.trim_value
        new_value = new_trim + self.input_sign * self.input_step
        if self.min_value <= new_value <= self.max_value:
            self.trim_value = new_trim
            self.apply_value()
        return old_value, self.trim_value

    def set_input(self, sign):
        self.input_sign = sign
        self.apply_value()

    def set_analog(self, analog):
        self.analog = analog
        self.apply_value()

    def reset(self):
        self.last_applied_value = None
        if self.reset_trim is not None:
            self.trim_value = self.reset_trim
        self.input_sign = 0

    def get_cumulative_value(self):
        return min(self.max_value, max(self.min_value, self.trim_value + self.input_sign * self.input_step + self.analog))

    def apply_value(self):
        if self.owner.is_connected():
            new_value = self.get_cumulative_value()
            if self.last_applied_value is None or new_value != self.last_applied_value:
                self.setter(self.owner.connection, new_value)
            self.last_applied_value = new_value


class Drone:
    connection: socket.socket
    variables: dict[str, DroneVariable]

    telemetry_thread: Thread
    heartbeat_thread: Thread
    telemetry_buffer: bytes
    telemetry_values: dict
    telemetry_lock: Lock

    def __init__(self):
        self.connection = None
        self.variables = {}
        self.reset_variables()
        self.telemetry_thread = Thread(target=self.receiver_thread_body, daemon=True)
        self.heartbeat_thread = Thread(target=self.heartbeat_thread_body, daemon=True)
        self.telemetry_thread.start()
        self.heartbeat_thread.start()
        self.telemetry_buffer = b""
        self.telemetry_values = {}
        self.frequency_log = []
        self.telemetry_lock = Lock()
        self.sensor_logging_started = False

    def reconnect(self):
        try:
            self.disconnect()
            self.telemetry_values = {}
            self.frequency_log = []
            print("Connecting")
            connection = socket.socket(socket.AF_BLUETOOTH, socket.SOCK_STREAM, socket.BTPROTO_RFCOMM)
            connection.connect((DRONE_ADDR, 1))
            print("Connection successful, resetting drone")
            self.connection = connection
            self.stop()
            self.reset_variables()
            self.apply_all_variables()
            print("Reset done")
        except Exception as e:
            print("ERROR: Connection failed", e)
            self.connection = None

    def disconnect(self):
        if self.connection is not None:
            print("Disconnecting")
            connection = self.connection
            self.connection = None
            try:
                connection.shutdown(socket.SHUT_RDWR)
                connection.close()
                print("Socket closed")
                time.sleep(1)
                self.telemetry_buffer = b""
            except Exception as e:
                print("ERROR: Disconnect failed", e)
            finally:
                self.connection = None

    def initialize_variables(self):
        self.variables = {
            YAW_VARIABLE_NAME: DroneVariable(self, "Yaw", 0, None, 90, 1, -ANGLE_INPUT_MAX,
                                             ANGLE_INPUT_MAX,
                                             storage_write_yaw_input),
            PITCH_VARIABLE_NAME: DroneVariable(self, "Pitch", 0, None, COMMAND_INPUT, 1, -COMMAND_INPUT_MAX,
                                               COMMAND_INPUT_MAX,
                                               storage_write_pitch_input),
            ROLL_VARIABLE_NAME: DroneVariable(self, "Roll", 0, None, COMMAND_INPUT, 1, -COMMAND_INPUT_MAX,
                                              COMMAND_INPUT_MAX,
                                              storage_write_roll_input),
            THRUST_VARIABLE_NAME: DroneVariable(self, "Thrust", 0, 0, 0, 10, 0, THRUST_MAX, storage_write_thrust_input),

            "yaw_kp": DroneVariable(self, "Yaw Kp", YAW_KP, None, 0, 1, 0, 5000000, storage_write_yaw_kp),
            "yaw_ki": DroneVariable(self, "Yaw Ki", YAW_KI, None, 0, 1, 0, 5000000, storage_write_yaw_ki),
            "yaw_kd": DroneVariable(self, "Yaw Kd", YAW_KD, None, 0, 1, 0, 5000000, storage_write_yaw_kd),
            "pitch_kp": DroneVariable(self, "pitch Kp", PITCH_KP, None, 0, 1, 0, 50000, storage_write_pitch_kp),
            "pitch_ki": DroneVariable(self, "pitch Ki", PITCH_KI, None, 0, 1, 0, 50000, storage_write_pitch_ki),
            "pitch_kd": DroneVariable(self, "pitch Kd", PITCH_KD, None, 0, 1, 0, 50000, storage_write_pitch_kd),
            "roll_kp": DroneVariable(self, "roll Kp", ROLL_KP, None, 0, 1, 0, 50000, storage_write_roll_kp),
            "roll_ki": DroneVariable(self, "roll Ki", ROLL_KI, None, 0, 1, 0, 50000, storage_write_roll_ki),
            "roll_kd": DroneVariable(self, "roll Kd", ROLL_KD, None, 0, 1, 0, 50000, storage_write_roll_kd)
        }

    def reset_variables(self):
        if not self.variables:
            self.initialize_variables()
        else:
            for variable in self.variables.values():
                variable.reset()

    def is_connected(self):
        return self.connection is not None

    def stop(self):
        if self.is_connected():
            stop_command(self.connection)
            self.variables[THRUST_VARIABLE_NAME].reset()
            self.variables[THRUST_VARIABLE_NAME].apply_value()

    def sensor_dump(self):
        if self.is_connected():
            if not self.sensor_logging_started:
                print("Invoking sensor log")
                sensor_log(self.connection)
                self.sensor_logging_started = True
            else:
                print("Invoking sensor dump")
                sensor_dump(self.connection)
                self.sensor_logging_started = False

    def set_bandwidth(self, bandwidth):
        if self.is_connected:
            set_mpu_dlpf(self.connection, bandwidth)

    def set_rate(self, rate):
        if self.is_connected:
            set_mpu_rate(self.connection, rate)

    def start(self):
        if self.is_connected():
            start_command(self.connection)
            self.variables[THRUST_VARIABLE_NAME].reset()
            self.variables[THRUST_VARIABLE_NAME].apply_value()

    def apply_all_variables(self):
        if self.is_connected():
            for variable in self.variables.values():
                variable.apply_value()
                time.sleep(0.05)

    def receiver_thread_body(self):
        print("Receiver thread start")
        while True:
            try:
                if self.connection:
                    recv_data = self.connection.recv(4096)
                    self.telemetry_buffer += recv_data
                    self.telemetry_lock.acquire()
                    try:
                        self.process_recv_buffer()
                    finally:
                        self.telemetry_lock.release()
                else:
                    time.sleep(2)
            except Exception as e:
                print("Receiver thread ERROR:", e)
                time.sleep(2)

    def get_telemetry_snapshot(self):
        self.telemetry_lock.acquire()
        try:
            returned_value = {}
            for value_id in self.telemetry_values:
                returned_value[value_id] = self.telemetry_values[value_id]
            return returned_value
        finally:
            self.telemetry_lock.release()

    def get_frequency_log(self):
        self.telemetry_lock.acquire()
        try:
            log = self.frequency_log
            self.frequency_log = []
            return log
        finally:
            self.telemetry_lock.release()

    def process_recv_buffer(self):
        while b"\x42" in self.telemetry_buffer:
            self.telemetry_buffer = self.telemetry_buffer[self.telemetry_buffer.find(b"\x42"):]
            if len(self.telemetry_buffer) < 8:
                break
            chunk = self.telemetry_buffer[:8]
            self.process_recv_chunk(chunk)

            self.telemetry_buffer = self.telemetry_buffer[8:]

        if len(self.telemetry_buffer) > 128:
            self.telemetry_buffer = self.telemetry_buffer[-128:]
        return

    def process_recv_chunk(self, chunk):
        if chunk[0] == 0x42 and chunk[-1] == 0x24:
            chunk = chunk[1:7]
            var_type = chunk[0]
            var_id = chunk[1]
            var_val = chunk[2:]
            if var_type == 0:
                var_val = struct.unpack("<L", var_val)[0]
            else:
                var_val = struct.unpack("<f", var_val)[0]
            self.telemetry_values[var_id] = var_val

            if var_id == 1:
                print("AAAAAAAAAAa", var_val, var_id, var_type)
                self.frequency_log.append(var_val)

    def heartbeat(self):
        if self.is_connected():
            heartbeat_command(self.connection)

    def heartbeat_thread_body(self):
        print("Heartbeat thread start")
        while True:
            try:
                self.heartbeat()
                time.sleep(0.05)
            except Exception as e:
                print("Heartbeat thread ERROR:", e)
                time.sleep(2)


def dcmain():
    s = socket.socket(socket.AF_BLUETOOTH,
                      socket.SOCK_STREAM, socket.BTPROTO_RFCOMM)
    s.connect((DRONE_ADDR, 1))

    while True:
        sq = input("Thrust: ")
        if sq.strip().lower() == "s":
            print("Stopping")
            stop_command(s)
        elif sq.strip().lower().startswith("r"):
            roll = int(sq[1:])
            print("writing roll", roll)
            storage_write_roll_input(s, roll)
        else:
            v = int(sq)
            print("writing", v)
            storage_write_thrust_input(s, v)

    halt_command(s)
    start_command(s)
    stop_command(s)
    read_storage_command(s, 37)
    write_storage_command(s, 42, 3, 327)

# dcmain()