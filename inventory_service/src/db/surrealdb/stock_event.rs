use super::{connection::SurrealDB, schema::StockEvent};
use surrealdb::{engine::remote::ws::Client, method::Stream, Error};

impl SurrealDB {
    pub async fn stream_stock_changes(&self) -> Result<Stream<'_, Client, Vec<StockEvent>>, Error> {
        let stream = self.client.select("inventory_stock_events").live().await;
        stream
    }
}
