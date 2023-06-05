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

## RabbitMQ channel

多路复用

## Qos 与 Manual Ack

Qos（Quality of Service）即服务质量保证与 Manual Ack 手动消费都是用于更为细致的消息消费方式。

Qos 参数：

- `prefetch_size`：服务器传送最大内容，没有限制则为 0

- `prefetch_count`：服务器每次传递的最大消息数，没有限制则为 0

- `global`：如果 true，则当前设置将会应用于整个频道（channel）

基础的应答方式：

- `basic_ack`：成功消费，消息从队列中删除。参数：

  - `delivery_tag`：服务器端向消费者推送消息，消息会携带一个deliveryTag参数，也可以成此参数为消息的唯一标识，是一个递增的正整数

  - `multiple`：true表示确认所有消息，包括消息唯一标识小于等于deliveryTag的消息，false只确认deliveryTag指定的消息

- `basic_nack`：拒绝消息，requeue=true 消息重新进入队列，反之被删除。参数：

  - `delivery_tag`

  - `multiple`

  - `requeue`：true 表示拒绝的消息应重新入队，反之丢弃

- `basic_reject`：等同于 nack

- `basic_recover`：消息重入队列，requeue=true 发送给新的 consumer，反之发给相同的 consumer

  - `如果为true,消息将会重新入队，可能会被发送给其它的消费者；如果为false,消息将会发送给相同的消费者`

## Topics exchange 的匹配案例

- Routing Key Pattern: It is the routing key pattern that binds a specific Queue with an Exchange.
- Valid Routing Key: The message with this key reaches the linked Queue.
- Invalid Routing Key: The message with this key does not reach the Queue.

|Routing Key Pattern|Valid Routing Key|Invalid Routing Key|
|---|---|---|
|__health.*__ <br/>health as the first word followed by one word.|health.education,<br/>health.sports,<br/>health.anything|health,<br/>health.education.anything,<br/>health.education.sports|
|__#.sports.*__ <br/>Zero or more words, then sports, after that exactly one word.|sports.education,<br/>sports.sports.sports,<br/>sports.sports|sports,<br/>education.sports,<br/>anything.sports.anything.xyz|
|__#.education__ <br/>Zero or more words followed by the word education.|health.education,<br/>education.education,<br/>education|education.health,<br/>anything.education.anything|

## 三种 Consumer 行为

- Consumer cancel: 消费者取消通知

- Consumer prefetch: 消费者消息的预获取，参数如下

  - `prefetch_size`：最大消息的占用大小，默认 `0` 为不设限；

  - `prefetch_count`：最多消息的预获取数量，默认 `0` 为不设限；

  - `global`：是否应用于整个频道。false，仅应用于此消费者；true，应用于整个频道，若是多消费者的情况，
  那么所有消费者预获取的消息数总和的上限则被限制（该种情况下，即使先将单个 consumer 的预获取数值调高，其还是会受限于频道的总数值）。

- Consumer priorities: 消费者优先级，（队列中多消费者的情况下）优先级高的消费者优先获取消息

## DLX (dead letter exchange)

文档：<https://www.rabbitmq.com/dlx.html>

### 设置 dlx 主要的三种方式

1. RabbitMQ Web UI

1. 命令行：

    ```sh
    `rabbitmqctl set_policy DLX ".*" '{"dead-letter-exchange":"dev-dlx"}' --apply-to queues`
    ```

    其中 `dev-dlx` 为 exchange 名称，其应用于所有 queue（`--apply-to`）。
    生产环境中需要显式声明 routing key，例如 `x-dead-letter-routing-key: dl`

1. 代码：

    1. 声明 exchange：`channel.exchange_diclare`，类型：`direct`；

    1. 声明 queue：`channel.queue_declare`，参数：`x-dead-letter-exchange: <exchange_name>`，其中 `<exchange_name>` 为第一步中的名称；channel 绑定该 queue；

### Routing dl 消息的方法

- 显式指定 routing key 为 dlx 的名称；

- 或者 routing key 为原本 publish 的，但消息中设置了例如 `x-dead-letter-routing-key: dl`。

### dl 消息

dl（死信）一条消息将会修改其头信息：

- exchange 名称被 dlx 名称代替；

- routing key 可能会被指定成处理 dl 的名称代替；

- 如果上述发送，那么 `CC` 头同样会被移除，同时，

- `BCC` 头将会被每个 [Sender-selected distribution](https://www.rabbitmq.com/sender-selected.html) 移除
