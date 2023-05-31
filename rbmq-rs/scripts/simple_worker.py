# @file:	simple_worker.py
# @author:	Jacob Xie
# @date:	2023/05/29 23:38:54 Monday
# @brief:

import pika
import time
import random

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

channel.exchange_declare("dlx")

channel.queue_declare(
    queue="task_queue",
    arguments={
        "x-message-ttl": 1000,
        "x-dead-letter-exchange": "dlx",
        # "x-dead-letter-routing-key": "dl",
    },
)
channel.queue_bind(exchange="amq.direct", queue="task_queue")
print(" [*] Waiting for messages. To exit press CTRL+C")


def callback(ch, method, properties, body):
    print(" [x] Received %r" % (body,))

    # TODO: properties' members are all None?
    print(properties)

    # ch.basic_ack(delivery_tag=method.delivery_tag)
    ch.basic_nack(delivery_tag=method.delivery_tag)
    # ch.basic_nack(delivery_tag=method.delivery_tag, requeue=False)

    time.sleep(5)

    # if random.random() < 0.5:
    #     ch.basic_ack(delivery_tag=method.delivery_tag)
    #     time.sleep(5)
    #     print(" [x] Done")
    # else:
    #     death = properties.headers("x-death")
    #     retries = properties.headers.get("x-retry-count")
    #     if death == None or retries < 5:
    #         ch.basic_reject(delivery_tag=method.delivery_tag)
    #         print(f" [x] Rejected ({retries})")
    #     else:
    #         ch.basic_nack(delivery_tag=method.delivery_tag, requeue=False)
    #         print(" [x] Retried ends")


channel.basic_qos(prefetch_count=1)
channel.basic_consume(queue="task_queue", on_message_callback=callback)

channel.start_consuming()
