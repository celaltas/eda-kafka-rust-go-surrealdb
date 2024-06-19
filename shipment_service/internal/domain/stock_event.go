package domain

type StockCreated struct{}

func NewStockCreated() StockCreated {
	return StockCreated{}
}

func (s *StockCreated) EventName() string {
	return "stock-created"
}

type StockUpdated struct{}

func NewStockUpdated() StockUpdated {
	return StockUpdated{}
}

func (s *StockUpdated) EventName() string {
	return "stock-updated"
}

type StockDeleted struct{}

func NewStockDeleted() StockDeleted {
	return StockDeleted{}
}

func (s *StockDeleted) EventName() string {
	return "stock-deleted"
}
