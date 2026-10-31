use crate::contracts_impl::ManagedConnectionPool;
use crate::entities::Department;
use ohkami::{Json, Response, claw::Path, claw::Query, fang::Context};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
#[derive(Deserialize)]
pub struct AddDepartment {
    pub name: String,
}
#[derive(Serialize)]
pub struct DepartmentResponse {
    id: String,
    name: String,
}
#[derive(Deserialize)]
pub struct ListDepartmentsQuery {
    limit: Option<usize>,
}
#[derive(Deserialize)]
pub struct UpdateDepartment {
    name: String,
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

pub async fn add_department(
    Json(payload): Json<AddDepartment>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    println!("Adding department: {}", payload.name);
    let _pool = &context.connection_pool;
    Response::NotImplemented()
}

pub async fn remove_department(Path(id): Path<&str>) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_text("invalid product id");
    };
    println!("Remove department: {id}");
    Response::NotImplemented()
}

pub async fn update_department(
    Path(id): Path<&str>,
    Json(payload): Json<UpdateDepartment>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_text("invalid department id");
    };
    println!("Update {id}: {}", payload.name);
    Response::NotImplemented()
}

pub async fn get_department(Path(id): Path<&str>) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_text("invalid product id");
    };
    println!("Requested product: {id}");
    Response::NotImplemented()
}
pub async fn list_departments(Query(query): Query<ListDepartmentsQuery>) -> Response {
    println!("Limit: {:?}", query.limit);
    Response::NotImplemented()
}

fn created_department_response(department: &Department) -> Response {
    Response::Created().with_json(DepartmentResponse {
        id: department.id.to_string(),
        name: department.name.clone(),
    })
}
