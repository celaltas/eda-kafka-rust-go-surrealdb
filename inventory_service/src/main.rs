use broker::kafka::connection::KafkaPublisher;
use config::get_config;
use db::surrealdb::connection::SurrealDB;
use futures_util::StreamExt;
use startup::run;
use std::net::TcpListener;
use tokio::task;
use log::{error, info};
use std::io::Write;





mod broker;
mod config;
mod db;
mod route;
mod startup;



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let start = std::time::Instant::now();
    env_logger::Builder::from_default_env()
        .format(move |buf, rec| {
            let t = start.elapsed().as_secs_f32();
            writeln!(buf, "{:.03} [{}] - {}", t, rec.level(), rec.args())
        })
        .init();


    let configuration = get_config().expect("Failed to read configuration.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address).expect("Failed to bind port");

    let db = SurrealDB::new(configuration.database)
        .await
        .expect("Failed to connect database");
    info!("Connected to database");

    let broker = KafkaPublisher::new(configuration.kafka)
        .await
        .expect("Failed to connect kafka");
    info!("Connected to kafka");

    let db_clone = db.clone();
    let broker_clone = broker.clone();
    let topic = "stock_update";

    task::spawn(async move {
        info!("Starting to stream stock changes");
        let mut stream = db_clone.stream_stock_changes().await.unwrap();
        while let Some(result) = stream.next().await {
            match result {
                Ok(result) => {
                    let event = &result.data;
                    let publishing_result = broker_clone.publish(topic, event).await;
                    match publishing_result {
                        Ok(_) => info!("Message published successfully"),
                        Err(e) => info!("Error publishing message: {}", e.to_string()),
                    }
                }
                Err(e) => error!("error: {}", e.to_string()),
            }
        }
    });

    run(listener, db)?.await
}
