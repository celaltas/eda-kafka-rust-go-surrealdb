use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Id};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductThing {
    id: Id,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
    pub id: ProductThing,
    pub name: String,
    pub price: i32,
    pub units: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateProductStock {
    pub units: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventThing {
    id: Id,
}

impl EventThing {
    pub fn id(&self) -> &Id {
        &self.id
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StockEvent {
    pub id: EventThing,
    pub time: Datetime,
    pub action: String,
    pub product: ProductThing,
    pub before_update: i32,
    pub after_update: i32,
}
