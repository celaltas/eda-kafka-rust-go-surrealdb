package main

import (
	"context"
	"log"
	"os"
	"os/signal"
	"shipment_service/internal/app"
	"shipment_service/internal/domain"
	"shipment_service/internal/kafka"
	"syscall"
	"time"

	"github.com/joho/godotenv"
)

func main() {
	err := godotenv.Load()
	if err != nil {
		log.Fatalf("Error loading .env file: %v", err)
	}

	kafkaBroker := os.Getenv("KAFKA_BROKER")
	kafkaTopic := os.Getenv("KAFKA_TOPIC")
	kafkaGroupID := os.Getenv("KAFKA_GROUP_ID")

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	stockUpdatedConsumer := kafka.NewKafkaConsumer([]string{kafkaBroker}, kafkaGroupID, kafkaTopic)
	messageChan := stockUpdatedConsumer.Messages()
	stockUpdatedConverter := kafka.NewMessageConverter(messageChan, "stock-updated")
	stockUpdatedApp := app.NewApplication(stockUpdatedConsumer, stockUpdatedConverter)
	stockUpdateHandler := app.NewStockUpdatedHandler()
	stockUpdateEvent := domain.NewStockUpdated()
	stockUpdatedApp.RegisterHandler(&stockUpdateEvent, stockUpdateHandler)

	go stockUpdatedApp.Run(ctx)

	sigs := make(chan os.Signal, 1)
	signal.Notify(sigs, syscall.SIGINT, syscall.SIGTERM)
	<-sigs
	cancel()
	log.Println("Wait 5 second to complete all background processes")
	time.Sleep(5 * time.Second)

}
