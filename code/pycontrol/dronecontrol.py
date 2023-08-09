import socket
import time
import struct

from commands_gen import *
from storage_gen import *
from typing import Callable
from threading import Thread, Lock

COMMAND_INPUT_MAX = 300
COMMAND_INPUT = 100
THRUST_MAX = 1500

ANGLE_INPUT_MAX = 180

THRUST_VARIABLE_NAME = "thrust"
PITCH_VARIABLE_NAME = "pitch"
YAW_VARIABLE_NAME = "yaw"
ROLL_VARIABLE_NAME = "roll"

YAW_KP = 50
YAW_KI = 15
YAW_KD = 100


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

    def reset(self):
        self.last_applied_value = None
        if self.reset_trim is not None:
            self.trim_value = self.reset_trim
        self.input_sign = 0

    def get_cumulative_value(self):
        return min(self.max_value, max(self.min_value, self.trim_value + self.input_sign * self.input_step))

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
    telemetry_buffer: bytes
    telemetry_values: dict
    telemetry_lock: Lock

    def __init__(self):
        self.connection = None
        self.variables = {}
        self.reset_variables()
        self.telemetry_thread = Thread(target=self.receiver_thread_body, daemon=True)
        self.telemetry_thread.start()
        self.telemetry_buffer = b""
        self.telemetry_values = {}
        self.telemetry_lock = Lock()

    def reconnect(self):
        try:
            self.disconnect()
            self.telemetry_values = {}
            print("Connecting")
            self.connection = socket.socket(socket.AF_BLUETOOTH, socket.SOCK_STREAM, socket.BTPROTO_RFCOMM)
            self.connection.connect(('00:18:E4:35:53:8C', 1))
            print("Connection successful, resetting drone")
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
            try:
                self.connection.shutdown(socket.SHUT_RDWR)
                self.connection.close()
                print("Socket closed")
                time.sleep(1)
                self.telemetry_buffer = b""
            except Exception as e:
                print("ERROR: Disconnect failed", e)
            finally:
                self.connection = None

    def initialize_variables(self):
        self.variables = {
            YAW_VARIABLE_NAME: DroneVariable(self, "Yaw", 0, None, 0, 1, -ANGLE_INPUT_MAX,
                                             ANGLE_INPUT_MAX,
                                             storage_write_yaw_input),
            PITCH_VARIABLE_NAME: DroneVariable(self, "Pitch", 0, None, COMMAND_INPUT, 1, -COMMAND_INPUT_MAX,
                                               COMMAND_INPUT_MAX,
                                               storage_write_pitch_input),
            ROLL_VARIABLE_NAME: DroneVariable(self, "Roll", 0, None, COMMAND_INPUT, 1, -COMMAND_INPUT_MAX,
                                              COMMAND_INPUT_MAX,
                                              storage_write_roll_input),
            THRUST_VARIABLE_NAME: DroneVariable(self, "Thrust", 0, 0, 0, 10, 0, THRUST_MAX, storage_write_thrust_input),

            "yaw_kp": DroneVariable(self, "Yaw Kp", YAW_KP, None, 0, 1, 0, 1000, storage_write_yaw_kp),
            "yaw_ki": DroneVariable(self, "Yaw Ki", YAW_KI, None, 0, 1, 0, 1000, storage_write_yaw_ki),
            "yaw_kd": DroneVariable(self, "Yaw Kd", YAW_KD, None, 0, 1, 0, 1000, storage_write_yaw_kd)
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
                var_val = struct.unpack("<l", var_val)[0]
            else:
                var_val = struct.unpack("<f", var_val)[0]
            self.telemetry_values[var_id] = var_val


def dcmain():
    s = socket.socket(socket.AF_BLUETOOTH,
                      socket.SOCK_STREAM, socket.BTPROTO_RFCOMM)
    s.connect(('00:18:E4:35:53:8C', 1))

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
