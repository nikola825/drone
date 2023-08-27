from storage_base import write_storage

def storage_write_storage_start(sock, arg_val):
    write_storage(sock, 0, 4, arg_val, 'uint16_t')

def storage_write_thrust_input(sock, arg_val):
    write_storage(sock, 4, 2, arg_val, 'uint16_t')

def storage_write_yaw_input(sock, arg_val):
    write_storage(sock, 6, 2, arg_val, 'int16_t')

def storage_write_pitch_input(sock, arg_val):
    write_storage(sock, 8, 2, arg_val, 'int16_t')

def storage_write_roll_input(sock, arg_val):
    write_storage(sock, 10, 2, arg_val, 'int16_t')

def storage_write_yaw_kp(sock, arg_val):
    write_storage(sock, 12, 2, arg_val, 'uint16_t')

def storage_write_yaw_ki(sock, arg_val):
    write_storage(sock, 14, 2, arg_val, 'uint16_t')

def storage_write_yaw_kd(sock, arg_val):
    write_storage(sock, 16, 2, arg_val, 'uint16_t')

def storage_write_pitch_kp(sock, arg_val):
    write_storage(sock, 18, 2, arg_val, 'uint16_t')

def storage_write_pitch_ki(sock, arg_val):
    write_storage(sock, 20, 2, arg_val, 'uint16_t')

def storage_write_pitch_kd(sock, arg_val):
    write_storage(sock, 22, 2, arg_val, 'uint16_t')

def storage_write_roll_kp(sock, arg_val):
    write_storage(sock, 24, 2, arg_val, 'uint16_t')

def storage_write_roll_ki(sock, arg_val):
    write_storage(sock, 26, 2, arg_val, 'uint16_t')

def storage_write_roll_kd(sock, arg_val):
    write_storage(sock, 28, 2, arg_val, 'uint16_t')

def storage_write_motor_thrust(sock, arg_val):
    write_storage(sock, 30, 2, arg_val, 'uint16_t')

def storage_write_motor_yaw(sock, arg_val):
    write_storage(sock, 32, 2, arg_val, 'int16_t')

def storage_write_motor_pitch(sock, arg_val):
    write_storage(sock, 34, 2, arg_val, 'int16_t')

def storage_write_motor_roll(sock, arg_val):
    write_storage(sock, 36, 2, arg_val, 'int16_t')
