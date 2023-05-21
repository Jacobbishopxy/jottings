# RBMQ-RS

## RabbitMQ exchange

1. direct exchange:

    通过 routing key 和队列绑定在一起；可以绑定任意数量的队列。若绑定的是多队列，exchange 会根据 routing key 将消息
    分给每个与之匹配的队列，即每个匹配 routing key 的队列都会得到一份全量的消息。

1. topic exchange:

    与 direct exchange 类似，根据 routing key 进行模式匹配（类似于正则表达式匹配）后与队列绑定在一起；其中，`*` 代表
    一个单词，`#` 代表 0 个或多个单词。

1. fanout exchange:

    不使用 routing key，而是将消息路由到所有与其绑定的队列；与 direct exchange 绑定了多个相同 routing key 的队列一样，
    绑定了 fanout exchange 的队列，都会接受到一份全量的消息；与 direct exchange 绑定多个队列的不同之处在于，direct
    exchange 可以根据 routing key 进行消息赛选。

1. headers exchange:

    忽略 routing key，根据消息的某些头信息分发过滤。一个重要参数 `x-match`：当 `x-match` 为 `any` 时，只需匹配
    任意一个 `header` 的属性值即可将 exchange 与 queue 绑定；当 `x-match` 为 `all` 时，所有值都必须匹配，才能将
    exchange 与 queue 绑定。

PS: 多个 consumer 绑定同一个队列的情况下，消息根据 round robin 的方式分配给不同的 consumer；可以设定 qos 参数调整分配
方式（详解 <https://www.rabbitmq.com/amqp-0-9-1-reference.html#basic.qos>）。
