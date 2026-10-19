use ohkami::{Json, Response, Status, fang::Context};
use serde::Deserialize;
use std::sync::Arc;
#[derive(Deserialize)]
pub struct AddProductPayload {
    pub name: String,
    pub price: f64,
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

pub async fn remove_product() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("removed product");
    response
}

pub async fn update_product() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("updated product");
    response
}

pub async fn get_product() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("got product");
    response
}