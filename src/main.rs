mod dynamic_pages;
mod kv;
mod login;
mod settings;
mod utils;

use crate::login::login_handler::signin_handler;
use crate::utils::common::{shutdown_signal, AppState, HtmlTemplate, MyAuthContext};
use askama::Template;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Extension, Router};
use axum_login::{AuthLayer, RequireAuthorizationLayer};
use axum_sessions::SessionLayer;
use kv::sled_session_store::SledSessionStore;
use login::auth_user::BlogAuthor;
use login::sled_user_store::SledUserStore;
use rand::{random, Rng};
use settings::Settings;
use std::net::SocketAddr;
use std::sync::Arc;
use utils::common::BlogStore;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    title: String,
}

#[tokio::main]
async fn main() {
    //TODO: Use config file for different environments.
    // TODO: Use right configs sled
    let settings = Settings::new();

    match settings {
        Ok(settings) => {
            let db_path: &str = settings.sledpath.path.as_str();
            let db = sled::open(db_path).expect("failed to open db");
            let mut kv_path = settings.sledpath.path.clone();

            kv_path.push_str("store");
            println!("Store using path: {:?}", kv_path);
            let store = Arc::new(db.open_tree(kv_path).unwrap());

            let blogstore = BlogStore {
                store: Arc::clone(&store),
            };

            let sled_user_store: SledUserStore<BlogAuthor> = SledUserStore::new(Arc::clone(&store));

            // Setup the user on application startup.
            // In dev / local environment we use default settings but in production
            // we will production settings
            sled_user_store.setup(settings.auser, settings.salt);

            let mut secret: Vec<u8> = (0..=127).map(|_| rand::random::<u8>()).collect();

            let auth_layer = AuthLayer::new(sled_user_store, &secret);

            let sled_session_store = SledSessionStore::new(Arc::clone(&store))
                .expect("failed to create Sled Session Store");
            let session_layer = SessionLayer::new(sled_session_store, &secret[..]);

            let sled_user_store: SledUserStore<BlogAuthor> = SledUserStore::new(Arc::clone(&store));

            let appstate = AppState {
                blogstore,
                userstore: sled_user_store,
            };

            async fn login_form_handler(mut auth: MyAuthContext) -> impl IntoResponse {
                println!("USER LOGIN----");
                // auth.login(&User::get_rusty_user()).await.unwrap();
                let template = LoginTemplate {
                    title: "App - Login".to_string(),
                };
                crate::utils::common::HtmlTemplate(template)
            }

            async fn logout_handler(mut auth: MyAuthContext) {
                dbg!("Logging out user: {}", &auth.current_user);
                auth.logout().await;
            }

            let app = Router::new()
                .route("/admin", get(dynamic_pages::articles::posts))
                .route_layer(RequireAuthorizationLayer::<BlogAuthor>::login())
                .route("/post", get(dynamic_pages::articles::post))
                .route("/login", get(login_form_handler).post(signin_handler))
                .layer(auth_layer)
                .layer(session_layer)
                .with_state(appstate);

            let address = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 8000));

            // TODO - add graceful shutdown
            axum::Server::bind(&address)
                .serve(app.into_make_service())
                .with_graceful_shutdown(shutdown_signal())
                .await
                .unwrap();
        }
        Err(err) => {
            panic!("{:#?}", err);
        }
    }
}
