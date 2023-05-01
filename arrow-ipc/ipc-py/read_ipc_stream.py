# @file:	read_ipc_stream.py
# @author:	Jacob Xie
# @date:	2023/05/01 14:05:31 Monday
# @brief:

import socket
import pyarrow as pa

listen = "127.0.0.1"
port = 56565

# with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
#     sock.bind((listen, port))
#     sock.listen()
#     print(f"Listening on {listen} on port {port}")
#     conn, _ = sock.accept()
#     with conn:
#         conn_file = conn.makefile(mode="b")  # type: ignore
#         reader = pa.ipc.RecordBatchStreamReader(conn_file)
#         table = reader.read_all()
#         print(table)
#         print(table.to_pandas())

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
    sock.connect((listen, port))
    with sock.makefile(mode="rb") as stream:
        with pa.ipc.open_stream(stream) as reader:
            table = reader.read_all()
            print(table)
