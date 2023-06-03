# @file:	simple_recv.py
# @author:	Jacob Xie
# @date:	2023/06/03 22:13:20 Saturday
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

channel.exchange_declare(exchange="my-headers-exchange", exchange_type="headers")

# Queue for messages with header 'h1' and 'c1'
queue1 = channel.queue_declare("", exclusive=True, arguments={"x-message-ttl": 30000})
queue1_name = queue1.method.queue

headers1 = {"x-match": "any", "h1": "1", "c1": "true"}

channel.queue_bind(
    exchange="my-headers-exchange", queue=queue1_name, arguments=headers1
)

# Queue for messages with header 'h2' and 'c1'
queue2 = channel.queue_declare("", exclusive=True, arguments={"x-message-ttl": 30000})
queue2_name = queue2.method.queue

headers2 = {"x-match": "any", "h1": "2", "c1": "true"}

channel.queue_bind(
    exchange="my-headers-exchange", queue=queue2_name, arguments=headers2
)

# Queue for all messages
queue3 = channel.queue_declare("", exclusive=True, arguments={"x-message-ttl": 30000})
queue3_name = queue3.method.queue

headers3 = {"x-match": "all", "h1": "1", "c1": "true"}

channel.queue_bind(
    exchange="my-headers-exchange", queue=queue3_name, arguments=headers3
)

print("Waiting for messages...")


def callback1(ch, method, properties, body):
    print(f"[1] Received message: {body}")


def callback2(ch, method, properties, body):
    print(f"[2] Received message: {body}")


def callback3(ch, method, properties, body):
    print(f"[3] Received message: {body}")


channel.basic_consume(queue=queue1_name, on_message_callback=callback1, auto_ack=True)

channel.basic_consume(queue=queue2_name, on_message_callback=callback2, auto_ack=True)

channel.basic_consume(queue=queue3_name, on_message_callback=callback3, auto_ack=True)

channel.start_consuming()
