import socket
import bluetooth
import time

def oldmain():
    import socket

    s=socket.socket(socket.AF_BLUETOOTH, socket.SOCK_STREAM, socket.BTPROTO_RFCOMM)
    s.connect(('00:18:E4:35:53:8C', 1))


    while True:
        q=int(input("T:"))
        if q>40 or q<0:
            break
        buff = b'\xc9' + chr(q).encode("utf-8")*4
        print(buff)
        s.send(buff)

    s.send(b'\xc9'+b'\x00'*4)
    s.close()

def newmain():
    s=socket.socket(socket.AF_BLUETOOTH, socket.SOCK_STREAM, socket.BTPROTO_RFCOMM)
    s.connect(('00:18:E4:35:53:8C', 1))

    while True:
        x=input("INPUT: ")
        s.send(x.encode("utf-8"))

newmain()