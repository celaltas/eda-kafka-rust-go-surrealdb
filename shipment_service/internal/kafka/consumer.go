package kafka

import (
	"context"
	"log"
	"shipment_service/internal/domain"
	"github.com/segmentio/kafka-go"
)

type KafkaConsumer struct {
	reader      *kafka.Reader
	messageChan chan domain.Message
}

func NewKafkaConsumer(brokers []string, groupID, topic string) *KafkaConsumer {
	return &KafkaConsumer{
		reader: kafka.NewReader(kafka.ReaderConfig{
			Brokers: brokers,
			GroupID: groupID,
			Topic:   topic,
		}),
		messageChan: make(chan domain.Message, 1),
	}
}

func (c *KafkaConsumer) Consume(ctx context.Context) {
	defer func() {
		defer c.reader.Close()
		defer close(c.messageChan)
	}()
	for {
		select {
		case <-ctx.Done():
			log.Println("Shutting down consumer...")
			return
		default:
			message, err := c.reader.ReadMessage(context.Background())
			if err != nil {
				log.Printf("Error reading message: %v", err)
				continue
			}
			c.messageChan <- KafkaMessage{msg: message}
			log.Printf("Consumed message at offset %d: %s = %s", message.Offset, string(message.Key), string(message.Value))
		}
	}

}

func (c *KafkaConsumer) Messages() <-chan domain.Message {
	return c.messageChan
}
