use std::sync::Arc;
use ohkami::{Ohkami, Route, fang::Context};

mod contracts;
mod contracts_impl;
mod entities;
mod http;

fn main() -> smol::io::Result<()> {
    smol::block_on(async {
        Ohkami::new((
            "/add-product".POST(http::add_product),
            "/remove-product".DELETE(http::remove_product),
            "/update-product".PUT(http::update_product),
            "/get-product".GET(http::get_product),
        ))
        .howl("localhost:3000")
        .await;
        Ok(())
    })
}