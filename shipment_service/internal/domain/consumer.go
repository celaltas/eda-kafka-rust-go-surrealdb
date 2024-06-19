package domain

import "context"

type Consumer interface {
	Messages() <-chan Message
	Consume(ctx context.Context)
}

type Message interface {
	Key() []byte
	Value() []byte
}
