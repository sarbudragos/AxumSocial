mod model;
mod controller;
mod repository;
mod service;

pub mod schema;

use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use crate::controller::post_controller::post_controller_router;
use crate::controller::user_controller::user_controller_router;
use crate::repository::post_repository::PostRepository;
use crate::repository::user_follow_repository::UserFollowRepository;
use crate::repository::user_repository::UserRepository;
use crate::service::post_service::PostService;
use crate::service::user_service::UserService;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access .env file");

    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:8080".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env file");

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&database_url);
    let pool = Pool::builder(config)
        .build()
        .unwrap_or_else(
            |_| panic!("Error connecting to {}", database_url)
        );

    let user_repository = UserRepository::new(pool.clone());
    let user_follow_repository = UserFollowRepository::new(pool.clone());
    let post_repository = PostRepository::new(pool.clone());

    let user_service = UserService::new(user_repository.clone(), user_follow_repository.clone());
    let post_service = PostService::new(post_repository.clone(), user_repository.clone());

    let user_controller = user_controller_router(user_service.clone());
    let post_controller = post_controller_router(post_service.clone());

    let router = Router::new()
        .nest("/api/users", user_controller)
        .nest("/api/posts", post_controller)
        .route("/", get(hello))
        .layer(CorsLayer::permissive());

    let listener = TcpListener::bind(server_address)
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router)
        .await
        .unwrap();
}

async fn hello() -> impl IntoResponse {
    Html("Hello <strong> WORLD !!! </strong>")
}
