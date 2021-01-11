# Code from Real Python website
# at https://realpython.com/python-sockets/
# retrieved at date 01-10-2021

import socket

HOST = '127.0.0.1'
PORT = 30222

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    s.bind((HOST, PORT))
    s.listen()
    conn, addr = s.accept()
    with conn:
        print('Connected by', addr)
        while True:
            data = conn.recv(1024)
            if not data:
                break
            conn.send(data)
    conn.close()
