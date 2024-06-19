use crate::db::surrealdb::schema::StockEvent;
use rdkafka::{
    error::{KafkaError, KafkaResult},
    message::{Header, OwnedHeaders},
    producer::FutureRecord,
};
use std::time::Duration;

use super::connection::KafkaPublisher;

impl KafkaPublisher {
    pub async fn publish(&self, topic: &str, event: &StockEvent) -> KafkaResult<()> {
        let queue_timeout = Duration::from_secs(0);
        let payload = format!("Message {}", serde_json::to_string(&event).unwrap());

        let key = event.id.id().to_string();
        let headers = OwnedHeaders::new().insert(Header {
            key: "stock_event",
            value: Some("stock_event"),
        });
        let record = FutureRecord::to(topic)
            .payload(&payload)
            .key(&key)
            .headers(headers);
        let result: Result<(i32, i64), (KafkaError, rdkafka::message::OwnedMessage)> =
            self.producer.send(record, queue_timeout).await;
        match result {
            Ok(_) => Ok(()),
            Err((kafka_error, _owned_message)) => Err(kafka_error),
        }
    }
}
