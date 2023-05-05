import struct

START_BYTE = b"\x42"
END_BYTE = b"\x24"
PADDING_BYTE = b"\x43"


def send_command(sock, command_id, fmt, *args):
    args_part = struct.pack(fmt, *args)
    buffer = START_BYTE+bytes([command_id]) + args_part
    while len(buffer) < 10:
        buffer += PADDING_BYTE

    checksum = 0
    for b in buffer:
        checksum ^= b
    buffer += bytes([checksum])
    buffer += END_BYTE
    sock.send(buffer)
