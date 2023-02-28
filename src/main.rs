mod dynamic_pages;
mod kv;
mod login;
mod settings;
mod static_pages;
mod utils;

use axum::{Extension, Router};
use kv::sled_session_store::SledSessionStore;
use settings::Settings;
use std::{net::SocketAddr};
use utils::common::BlogStore;
use login::auth_user::User;
use login::sled_user_store::SledUserStore;

#[tokio::main]
async fn main() {
    //TODO: Use config file for different environments.
    // TODO: Use right configs sled
    let settings = Settings::new();

    match settings {
        Ok(settings) => {
            let db_path = settings.sledpath.path;
            let db = sled::open(db_path).expect("failed to open db");
            
            let meta = db.open_tree(settings.metastore.keyspace).expect("failed to open meta keyspace");
            let body = db.open_tree(settings.bodystore.keyspace).expect("failed to open body keyspace");
            let blogstore = BlogStore { meta, body };
 
            let user = db.open_tree(settings.userstore.keyspace).expect("failed to open user keyspace");
            let sled_user_store:SledUserStore<User, ()> = SledUserStore::new(user);

            let sled_session_store = SledSessionStore::new(settings.sessionstore.keyspace.as_str());
                
            let app = Router::new()
                .merge(static_pages::routes())
                .merge(dynamic_pages::index_route())
                .merge(dynamic_pages::post_route())
                .layer(Extension(blogstore));

            let address = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 8000));

            // TODO - add graceful shutdown
            axum::Server::bind(&address)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
        Err(err) => {
            panic!("{:#?}", err);
        }
    }
}
