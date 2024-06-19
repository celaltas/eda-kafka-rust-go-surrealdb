use crate::config::DatabaseSettings;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Result, Surreal,
};


#[derive(Debug, Clone)]
pub struct SurrealDB {
    pub client: Surreal<Client>,
}

impl SurrealDB {
    pub async fn new(settings: DatabaseSettings) -> Result<SurrealDB> {
        let client = Self::connect_db(settings)
            .await
            .map_err(|_e| surrealdb::error::Api::ConnectionUninitialised)?;
        Ok(SurrealDB { client })
    }

    pub async fn connect_db(settings: DatabaseSettings) -> Result<Surreal<Client>> {
        let db =
            Surreal::new::<Ws>(format!("{}:{}", settings.host, settings.port).as_str()).await?;
        db.signin(Root {
            username: settings.username.as_str(),
            password: settings.password.as_str(),
        })
        .await?;
        db.use_ns(settings.namespace)
            .use_db(settings.dbname)
            .await?;
        Ok(db)
    }
}

#[cfg(test)]
mod tests {
    use crate::{config::DatabaseSettings, db::surrealdb::connection::SurrealDB};

    #[tokio::test]
    async fn test_connect_db() {
        let settings = DatabaseSettings {
            host: "127.0.0.1".to_string(),
            port: 8000,
            username: "root".to_string(),
            password: "root".to_string(),
            dbname: "test".to_string(),
            namespace: "test".to_string(),
        };

        let db = SurrealDB::connect_db(settings).await;
        assert!(db.is_ok());
    }
}
