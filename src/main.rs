use contracts_impl::ManagedConnectionPool;
use http::*;
use ohkami::{Ohkami, Route, fang::Context};
use std::sync::Arc;

mod contracts;
mod contracts_impl;
mod entities;
mod http;

fn main() {
    smol::block_on(async {
        let pool = ManagedConnectionPool::initialize("database.db".into(), 10).await;
        let request_context = RequestContext::new(Arc::new(pool));
        Ohkami::new((
            Context::new(request_context),
            "/add-product".POST(http::add_product),
            "/product/:id"
                .GET(http::get_product)
                .PUT(http::update_product)
                .DELETE(http::remove_product),
        ))
        .howl("localhost:3000")
        .await;
    })
}
