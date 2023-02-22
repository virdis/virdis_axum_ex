mod utils;
mod static_pages;
mod kv;
mod dynamic_pages;

use std::{net::SocketAddr, sync::Arc};

use axum::{Router, Extension};
use sled::Db;
use utils::common::Store;
use axum::ServiceExt;

#[tokio::main]
async fn main() {

    //TODO: Use config file for different environments.
    // TODO: Use right configs sled

    let db = sled::open("/home/virdis/Source/rust/virdis_me/temp/").expect("failed to open db");
    let meta = db.open_tree("meta").expect("failed to open meta keyspace");
    let body = db.open_tree("body").expect("failed to open body keyspace");

    let store = Store{ meta, body };


    let app = Router::new()
        .merge(static_pages::routes())
        .merge(dynamic_pages::index_route())
        .merge(dynamic_pages::post_route())
        .layer(Extension(store));

    let address = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 8000));

// TODO - add graceful shutdown
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await 
        .unwrap();

}
