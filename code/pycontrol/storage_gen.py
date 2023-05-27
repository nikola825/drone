from commands_gen import write_storage_command

def storage_write_storage_start(sock, arg_val):
    write_storage_command(sock, 0, 4, arg_val)

def storage_write_thrust_input(sock, arg_val):
    write_storage_command(sock, 4, 2, arg_val)

def storage_write_yaw_input(sock, arg_val):
    write_storage_command(sock, 6, 2, arg_val)

def storage_write_pitch_input(sock, arg_val):
    write_storage_command(sock, 8, 2, arg_val)

def storage_write_roll_input(sock, arg_val):
    write_storage_command(sock, 10, 2, arg_val)

def storage_write_motor_thrust(sock, arg_val):
    write_storage_command(sock, 12, 2, arg_val)

def storage_write_motor_yaw(sock, arg_val):
    write_storage_command(sock, 14, 2, arg_val)

def storage_write_motor_pitch(sock, arg_val):
    write_storage_command(sock, 16, 2, arg_val)

def storage_write_motor_roll(sock, arg_val):
    write_storage_command(sock, 18, 2, arg_val)
