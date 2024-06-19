package kafka

import "github.com/segmentio/kafka-go"

type KafkaMessage struct {
    msg kafka.Message
}

func (km KafkaMessage) Value() []byte {
    return km.msg.Value
}

func (km KafkaMessage) Key() []byte {
    return km.msg.Key
}