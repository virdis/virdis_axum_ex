mod dynamic_pages;
mod kv;
mod login;
mod settings;
mod static_pages;
mod utils;

use axum::{Extension, Router};
use axum_sessions::SessionLayer;
use kv::sled_session_store::SledSessionStore;
use rand::seq::SliceRandom;
use rand::{Rng, random};
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
            let sled_user_store: SledUserStore<User, ()> = SledUserStore::new(user);

            
            let mut rng = rand::thread_rng();

            let mut nums: Vec<u8> = (0..128).collect();
            nums.shuffle(&mut rng);
            

            let sled_session_store = SledSessionStore::new(settings
                .sessionstore.keyspace.as_str()).expect("failed to create Sled Session Store");
            let session_layer = SessionLayer::new(sled_session_store, &nums[..]);
            

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
