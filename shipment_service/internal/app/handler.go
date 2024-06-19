package app

import (
	"context"
	"log"
	"shipment_service/internal/domain"
)

type StockUpdatedHandler struct{}

func NewStockUpdatedHandler() StockUpdatedHandler {
	return StockUpdatedHandler{}
}

func (h StockUpdatedHandler) Handle(ctx context.Context, event domain.Event) error {
	log.Println("Handling StockUpdated event")
	return nil
}

type StockDeletedHandler struct{}

func (h *StockDeletedHandler) Handle(ctx context.Context, event domain.Event) error {
	log.Println("Handling StockDeleted event")
	return nil
}

type StockCreatedHandler struct{}

func (h *StockCreatedHandler) Handle(ctx context.Context, event domain.Event) error {
	log.Println("Handling StockCreated event")
	return nil
}
