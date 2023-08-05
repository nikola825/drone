import socket
from commands_gen import *
from storage_gen import *
from typing import Callable

COMMAND_INPUT_MAX = 300
COMMAND_INPUT = 100
THRUST_MAX = 1500

THRUST_VARIABLE_NAME = "thrust"
PITCH_VARIABLE_NAME = "pitch"
YAW_VARIABLE_NAME = "yaw"
ROLL_VARIABLE_NAME = "roll"


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

    def __init__(self):
        self.connection = None
        self.variables = {}
        self.reset_variables()

    def reconnect(self):
        try:
            self.disconnect()
            self.connection = socket.socket(socket.AF_BLUETOOTH, socket.SOCK_STREAM, socket.BTPROTO_RFCOMM)
            self.connection.connect(('00:18:E4:35:53:8C', 1))
            self.stop()
            self.reset_variables()
            self.apply_all_variables()
        except Exception as e:
            print("ERROR: Connection failed", e)
            self.connection = None

    def disconnect(self):
        if self.connection is not None:
            try:
                self.connection.shutdown(socket.SHUT_RDWR)
                self.connection.close()
            except Exception as e:
                print("ERROR: Disconnect failed", e)
            finally:
                self.connection = None

    def initialize_variables(self):
        self.variables = {
            YAW_VARIABLE_NAME: DroneVariable(self, "Yaw", 0, None, COMMAND_INPUT, 1, -COMMAND_INPUT_MAX,
                                             COMMAND_INPUT_MAX,
                                             storage_write_yaw_input),
            PITCH_VARIABLE_NAME: DroneVariable(self, "Pitch", 0, None, COMMAND_INPUT, 1, -COMMAND_INPUT_MAX,
                                               COMMAND_INPUT_MAX,
                                               storage_write_pitch_input),
            ROLL_VARIABLE_NAME: DroneVariable(self, "Roll", 0, None, COMMAND_INPUT, 1, -COMMAND_INPUT_MAX,
                                              COMMAND_INPUT_MAX,
                                              storage_write_roll_input),
            THRUST_VARIABLE_NAME: DroneVariable(self, "Thrust", 0, 0, 0, 10, 0, THRUST_MAX, storage_write_thrust_input)
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
