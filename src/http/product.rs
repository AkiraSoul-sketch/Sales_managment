use crate::ManagedConnectionPool;
use crate::entities::Product;
use ohkami::{Json, Response, claw::Path, claw::Query, fang::Context};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
#[derive(Deserialize)]
pub struct AddProductPayload {
    pub name: String,
    pub price: f32,
}
#[derive(Serialize)]
pub struct ProductResponse {
    id: String,
    name: String,
    price: f32,
}
#[derive(Deserialize)]
pub struct ListProductsQuery {
    limit: Option<usize>,
}
#[derive(Deserialize)]
pub struct UpdateProductPayload {
    name: String,
    price: f32,
}
#[derive(Clone)]
pub struct RequestContext {
    connection_pool: Arc<ManagedConnectionPool>,
}

impl RequestContext {
    pub fn new(connection_pool: Arc<ManagedConnectionPool>) -> Self {
        Self { connection_pool }
    }
}

pub async fn add_product(
    Json(payload): Json<AddProductPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    println!("Adding product: {}", payload.name);
    let _pool = &context.connection_pool;
    Response::NotImplemented()
}

pub async fn remove_product(Path(id): Path<&str>) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_text("invalid product id");
    };
    println!("Remove product: {id}");
    Response::NotImplemented()
}

pub async fn update_product(
    Path(id): Path<&str>,
    Json(payload): Json<UpdateProductPayload>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_text("invalid product id");
    };
    println!("Update {id}: {} / {}", payload.name, payload.price);
    Response::NotImplemented()
}

pub async fn get_product(Path(id): Path<&str>) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_text("invalid product id");
    };
    println!("Requested product: {id}");
    Response::NotImplemented()
}
pub async fn list_products(Query(query): Query<ListProductsQuery>) -> Response {
    println!("Limit: {:?}", query.limit);
    Response::NotImplemented()
}

fn created_product_response(product: &Product) -> Response {
    Response::Created().with_json(ProductResponse {
        id: product.item_number.to_string(),
        name: product.name.clone(),
        price: product.price,
    })
}
