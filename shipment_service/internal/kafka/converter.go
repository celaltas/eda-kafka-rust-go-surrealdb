package kafka

import (
	"context"
	"log"
	"shipment_service/internal/domain"
)

type MessageConverter struct {
	messageChan <-chan domain.Message
	eventChan   chan domain.Event
	eventType   string
}

func NewMessageConverter(messageChan <-chan domain.Message, eventType string) *MessageConverter {
	return &MessageConverter{
		messageChan: messageChan,
		eventChan:   make(chan domain.Event, 1),
		eventType:   eventType,
	}
}

func (mc *MessageConverter) Events() <-chan domain.Event {
	return mc.eventChan
}

func (mc *MessageConverter) Start(ctx context.Context) {
	for {
		select {
		case <-ctx.Done():
			log.Println("Shutting down message converter...")
			return
		case msg := <-mc.messageChan:
			event := mc.convertMessageToEvent(msg)
			if event != nil {
				log.Println("Message converted to event succesfully")
				mc.eventChan <- event
			}
		}
	}
}
func (mc *MessageConverter) convertMessageToEvent(_ domain.Message) domain.Event {
	switch mc.eventType {
	case "stock-created":
		return &domain.StockCreated{}
	case "stock-updated":
		return &domain.StockUpdated{}
	case "stock-deleted":
		return &domain.StockDeleted{}
	default:
		log.Printf("Unknown event type: %v", mc.eventType)
		return nil
	}
}
