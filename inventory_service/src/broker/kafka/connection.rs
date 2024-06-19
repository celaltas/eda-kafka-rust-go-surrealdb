use crate::config::KafkaSettings;
use rdkafka::{error::KafkaResult, producer::FutureProducer, ClientConfig};



#[derive(Clone)]
pub struct KafkaPublisher {
    pub producer: FutureProducer,
}

impl KafkaPublisher {
    pub async fn new(settings: KafkaSettings) -> KafkaResult<Self> {
        let producer = ClientConfig::new()
            .set("bootstrap.servers", settings.broker)
            .set("message.timeout.ms", "5000")
            .set("allow.auto.create.topics", "true")
            .create()?;

        Ok(KafkaPublisher { producer })
    }
}
