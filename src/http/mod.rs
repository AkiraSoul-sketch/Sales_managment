use ohkami::{Json, Response, Status};
use serde::Deserialize;
#[derive(Deserialize)]
pub struct AddProductPayload {
    pub name: String,
    pub price: f32,
}
pub async fn add_product(Json(payload): Json<AddProductPayload>) -> Response {
    println!("{}: {}", payload.name, payload.price);
    let mut response = Response::new(Status::OK);
    response.set_text("added sales");
    response
}

pub async fn remove_product() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("removed sales");
    response
}

pub async fn update_product() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("updated sales");
    response
}

pub async fn get_product() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("got sales");
    response
}