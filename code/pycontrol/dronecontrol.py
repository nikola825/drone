import socket
import time
from commands_gen import *

COMMAND_INPUT_MAX = 200
THRUST_MAX = 500

class Drone:
    def __init__(self):
        pass

    def roll_left(self):
        pass

    def roll_right(self):
        pass

    def pitch_forward(self):
        pass

    def pitch_back(self):
        pass

    def yaw_clock(self):
        pass
    
    def yaw_counterclock(self):
        pass

    def thrust_up(self):
        pass

    def thrust_down(self):
        pass

def main():
    s = socket.socket(socket.AF_BLUETOOTH,
                      socket.SOCK_STREAM, socket.BTPROTO_RFCOMM)
    s.connect(('00:18:E4:35:53:8C', 1))

    halt_command(s)
    start_command(s)
    stop_command(s)
    read_storage_command(s, 37)
    write_storage_command(s, 42, 3, 327)

