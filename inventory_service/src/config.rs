use std::error::Error;

use config::{Config, File, FileFormat};
use serde::Deserialize;


type ConfigResult<T> = Result<T, Box<dyn Error>>;


#[derive(Deserialize, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub kafka: KafkaSettings,
}

#[derive(Deserialize, Debug)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
    pub storage: String,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub dbname: String,
    pub namespace: String,
}

#[derive(Deserialize, Debug)]
pub struct KafkaSettings {
    pub broker: String,
    pub topic: String,
}

pub fn get_config() -> ConfigResult<Settings> {
    let base_path = std::env::current_dir()?;
    let conf_path = base_path.join("src/configuration/base");
    let builder =
        Config::builder().add_source(File::new(conf_path.to_str().unwrap(), FileFormat::Yaml));
    let config = builder.build()?;
    let config = config.try_deserialize().map_err(|e| e.into());
    config
}

#[cfg(test)]
mod tests {
    use crate::config::{
        get_config, ApplicationSettings, DatabaseSettings, KafkaSettings, Settings,
    };

    #[test]
    fn test_get_config() {
        let res = get_config();
        assert!(res.is_ok());
        let expected = Settings {
            application: ApplicationSettings {
                host: "127.0.0.1".to_string(),
                port: 8080,
                storage: "uploads".to_string(),
            },
            database: DatabaseSettings {
                host: "127.0.0.1".to_string(),
                port: 8000,
                username: "root".to_string(),
                password: "root".to_string(),
                dbname: "test".to_string(),
                namespace: "test".to_string(),
            },
            kafka: KafkaSettings {
                broker: "localhost:9092".to_string(),
                topic: "stock_update".to_string(),
            },
        };
        let res = res.unwrap();
        assert_eq!(res.kafka.broker, expected.kafka.broker);
    }
}
