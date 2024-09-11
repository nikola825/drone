from commands_base import send_command

def halt_command(sock, ):
    send_command(sock, 0, '<', )

def start_command(sock, ):
    send_command(sock, 1, '<', )

def stop_command(sock, ):
    send_command(sock, 2, '<', )

def heartbeat_command(sock, ):
    send_command(sock, 3, '<', )

def read_storage_command(sock, address):
    send_command(sock, 4, '<H', address)

def write_storage_command(sock, address, length, value):
    send_command(sock, 5, '<HBL', address, length, value)

def set_mpu_rate(sock, rate):
    send_command(sock, 6, '<B', rate)

def set_mpu_dlpf(sock, dlpf):
    send_command(sock, 7, '<B', dlpf)
