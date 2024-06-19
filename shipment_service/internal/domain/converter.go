package domain

import "context"




type Converter interface {
	Events() <-chan Event
	Start(ctx context.Context)
}