from commands_gen import write_storage_command
from struct import pack, unpack

signed_int_types = {
    "int8_t": "b",
    "int16_t": "h",
    "int32_t": "l",
}

unsigned_int_types = {
    "uint8_t": "B",
    "uint16_t": "H",
    "uint32_t": "L",
}


def write_storage(socket, address, size, value, var_type):
    if var_type in signed_int_types:
        unsigned_type = "u" + var_type
        value = unpack("<" + unsigned_int_types[unsigned_type], pack("<" + signed_int_types[var_type], value))[0]

    write_storage_command(socket, address, size, value)
