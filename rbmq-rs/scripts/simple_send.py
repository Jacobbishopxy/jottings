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

credentials = pika.PlainCredentials(username=username, password=password)
connection = pika.BlockingConnection(
    pika.ConnectionParameters(
        host=host, port=port, virtual_host=vhost, credentials=credentials
    )
)
channel = connection.channel()

channel.basic_publish(
    exchange="amq.direct", routing_key="task_queue", body="Hello World!"
)
print(" [x] Sent 'Hello World!'")

connection.close()

# channel.queue_declare("rbmq-rs-que", arguments={"x-message-ttl": 30000})
# channel.queue_bind("rbmq-rs-que", "rbmq-rs-exchange")
