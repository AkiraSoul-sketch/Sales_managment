use crate::contracts_impl::ManagedConnectionPool;
use crate::entities::Sales;
use ohkami::{Json, Response, claw::Path, claw::Query, fang::Context};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
#[derive(Deserialize)]
pub struct AddSales {
    pub sail_date: String,
    pub sold_items_count: String,
}
#[derive(Serialize)]
pub struct SalesResponse {
    id: String,
}
#[derive(Deserialize)]
pub struct ListSalesQuery {
    limit: Option<usize>,
}
#[derive(Deserialize)]
pub struct UpdateSales {
    sail_date: String,
    sold_items_count: String,
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

pub async fn add_sales(
    Json(payload): Json<AddSales>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    println!(
        "Adding sales: {}/ {}",
        payload.sail_date, payload.sold_items_count
    );
    let _pool = &context.connection_pool;
    Response::NotImplemented()
}

pub async fn remove_sales(Path(id): Path<&str>) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_text("invalid product id");
    };
    println!("Remove sale: {id}");
    Response::NotImplemented()
}

pub async fn update_sales(Path(id): Path<&str>, Json(payload): Json<UpdateSales>) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_text("invalid sale id");
    };
    println!(
        "Update {id}: {} / {}",
        payload.sail_date, payload.sold_items_count
    );
    Response::NotImplemented()
}

pub async fn get_sales(Path(id): Path<&str>) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_text("invalid sale id");
    };
    println!("Requested sale: {id}");
    Response::NotImplemented()
}
pub async fn list_sales(Query(query): Query<ListSalesQuery>) -> Response {
    println!("Limit: {:?}", query.limit);
    Response::NotImplemented()
}

fn created_sales_response(sales: &Sales) -> Response {
    Response::Created().with_json(SalesResponse {
        id: sales.id.to_string(),
    })
}
