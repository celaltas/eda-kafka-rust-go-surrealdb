use crate::db::surrealdb::{
    connection::SurrealDB,
    schema::{Product, UpdateProductStock},
};
use actix_web::{
    error::{ErrorBadRequest, InternalError},
    get,
    http::StatusCode,
    patch,
    web::{Data, Json, Path},
    Error, HttpRequest, HttpResponse,
};
use log::{error, info};

#[get("/")]
async fn get_inventory_products(
    db: Data<SurrealDB>,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    let ip = request
        .peer_addr()
        .ok_or(InternalError::new(
            "IP address not found at request",
            StatusCode::BAD_REQUEST,
        ))?
        .ip();
    info!("{} - Request for inventory products", ip);
    let products: Vec<Product> = db.client.select("inventory").await.map_err(|e| {
        error!("{}", e);
        InternalError::new("Database query failed", StatusCode::INTERNAL_SERVER_ERROR)
    })?;
    Ok(HttpResponse::Ok().json(products))
}

#[patch("/{product_id}")]
async fn update_inventory_products(
    path: Path<String>,
    payload: Json<UpdateProductStock>,
    db: Data<SurrealDB>,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    let product_id = path.into_inner();
    let ip = request
        .peer_addr()
        .ok_or(InternalError::new(
            "IP address not found at request",
            StatusCode::BAD_REQUEST,
        ))?
        .ip();
    info!(
        "{} - Request for updating inventory for product: {}",
        ip, product_id
    );
    let payload = payload.into_inner();

    let units_to_reduce = payload.units;
    info!("unit:{}", units_to_reduce);

    let sql = format!("SELECT units FROM inventory:{}", product_id);
    let mut result = db.client.query(sql).await.map_err(|e| {
        error!("{}", e);
        InternalError::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    let current_unit: Option<i32> = result.take((0, "units")).map_err(|e| {
        error!("{}", e);
        InternalError::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    info!("Current unit: {:?}", current_unit);

    if let Some(current_unit) = current_unit {
        if current_unit < units_to_reduce {
            return Err(ErrorBadRequest("Not enough units to reduce"));
        } else {
            let updated_unit = current_unit - units_to_reduce;
            let update_query = format!(
                "UPDATE inventory:{} SET units = {}",
                product_id, updated_unit
            );
            let update_result = db.client.query(update_query).await.map_err(|e| {
                error!("{}", e);
                InternalError::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
            })?;
            info!("update_result:{:#?}", update_result);
            return Ok(HttpResponse::Ok().body("Product updated successfully"));
        }
    } else {
        return Ok(HttpResponse::NotFound().body("Product not found"));
    };
}
