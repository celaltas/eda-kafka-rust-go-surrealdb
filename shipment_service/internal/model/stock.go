
package model



type IDStringWrapper struct {
	String string `json:"String"`
}

type IDWrapper struct {
	ID IDStringWrapper `json:"id"`
}

type StockUpdateMessage struct {
	ID           IDWrapper `json:"id"`
	Time         string    `json:"time"`
	Action       string    `json:"action"`
	Product      IDWrapper `json:"product"`
	BeforeUpdate int       `json:"before_update"`
	AfterUpdate  int       `json:"after_update"`
}

type StockUpdateEvent struct {
	ID           string `json:"id"`
	ProductID    string `json:"productId"`
	BeforeUpdate int    `json:"before_update"`
	AfterUpdate  int    `json:"after_update"`
	Action       string `json:"action"`
	Time         string `json:"time"`
}