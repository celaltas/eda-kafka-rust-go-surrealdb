package app

import (
	"context"
	"log"
	"shipment_service/internal/domain"
)

type Application struct {
	consumer  domain.Consumer
	converter domain.Converter
	handlers  map[string]domain.EventHandler
}

func NewApplication(consumer domain.Consumer, converter domain.Converter) *Application {
	return &Application{
		handlers: make(map[string]domain.EventHandler),
		consumer: consumer,
		converter: converter,
	}
}

func (a *Application) RegisterHandler(event domain.Event, handler domain.EventHandler) {
	a.handlers[event.EventName()] = handler
}

func (a *Application) dispatchEvent(ctx context.Context, event domain.Event) {
	if handler, ok := a.handlers[event.EventName()]; ok {
		if err := handler.Handle(ctx, event); err != nil {
			log.Printf("Error handling event: %v", err)
		}
	} else {
		log.Printf("No handler found for event: %v", event.EventName())
	}
}

func (a *Application) Run(ctx context.Context) {
	log.Println("Starting application...")
	go a.consumer.Consume(ctx)
	go a.converter.Start(ctx)

	for {
		select {
		case <-ctx.Done():
			log.Println("Shutting down application...")
			return
		case event := <-a.converter.Events():
			a.dispatchEvent(ctx, event)
		}
	}
}

func (a *Application) Stop() {
	println("Goodbye, world!")
}
