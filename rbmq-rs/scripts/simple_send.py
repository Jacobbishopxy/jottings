# @file:	simple_send.py
# @author:	Jacob Xie
# @date:	2023/05/29 23:33:22 Monday
# @brief:

import pika

host = "localhost"
port = 5672
username = "dev"
password = "devpass"
vhost = "devhost"

exchange = "my-headers-exchange"

credentials = pika.PlainCredentials(username=username, password=password)
connection = pika.BlockingConnection(
    pika.ConnectionParameters(
        host=host, port=port, virtual_host=vhost, credentials=credentials
    )
)
channel = connection.channel()

channel.basic_publish(
    exchange=exchange,
    routing_key="",
    body="Hello World! --- 1",
    properties=pika.BasicProperties(headers={"h1": "1"}),
)
print(" [x] Sent 'Hello World!'")

channel.basic_publish(
    exchange=exchange,
    routing_key="",
    body="Hello World! --- 2",
    properties=pika.BasicProperties(headers={"h1": "1", "c1": "true"}),
)
print(" [x] Sent 'Hello World!'")


connection.close()
