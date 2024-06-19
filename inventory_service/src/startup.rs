use crate::db::surrealdb::connection::SurrealDB;
use crate::route::product::{get_inventory_products, update_inventory_products};
use actix_web::web::{self, Data};
use actix_web::{dev::Server, App, HttpServer};
use log::info;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db: SurrealDB) -> Result<Server, std::io::Error> {
    info!("Starting server on {}", listener.local_addr()?);
    let db = Data::new(db);
    let server = HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/inventory")
                    .service(get_inventory_products)
                    .service(update_inventory_products),
            )
            .app_data(db.clone())
    })
    .listen(listener)?
    .run();
    info!("Server started successfully");
    Ok(server)
}
