use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::{delete, get, post, post_service, put};
use serde_json::json;
use crate::model::post::{NewPost, Post};
use crate::model::user::NewUser;
use crate::service::post_service::PostService;

pub fn post_controller_router(post_service: PostService)  -> Router{
    Router::new()
        .route("/", get(get_posts))
        .route("/{id}", get(get_post))
        .route("/user/{id}", get(get_posts_of_user))
        .route("/", post(add_post))
        //.route("/", put(update_post))
        .route("/{id}", delete(delete_post))
        .with_state(post_service)
}

async fn get_posts(
    State(post_service): State<PostService>
) -> impl IntoResponse {
    println!("GET /posts");
    match post_service.get_posts().await {
        Ok(data) => Ok((
            StatusCode::OK,
            json!({"success": true, "data": data}).to_string()
        )),

        Err(err) => Err(err)
    }
}

async fn get_posts_of_user(
    State(post_service): State<PostService>,
    Path(id): Path<i32>
) -> impl IntoResponse {
    match post_service.get_posts_of_user_sorted(id).await {
        Ok(data) => Ok((
            StatusCode::OK,
            json!({"success": true, "data": data}).to_string()
        )),
        Err(err) => Err(err)
    }
}

async fn get_post(
    State(post_service): State<PostService>,
    Path(id): Path<i32>
) -> impl IntoResponse {
    match post_service.get_post_by_id(id).await{
        Ok(data) => Ok((
            StatusCode::OK,
            json!({"success": true, "data": data}).to_string()
        )),
        Err(err) => Err(err)
    }
}
async fn add_post(
    State(post_service): State<PostService>,
    Json(body): Json<NewPost>,
) -> impl IntoResponse {
    match post_service.add_post(body).await {
        Ok(data) => Ok((
            StatusCode::OK,
            json!({"success": true, "data": data}).to_string()
        )),
        Err(err) => Err(err)
    }
}

async fn update_post(
    State(post_service): State<PostService>,
    Json(body): Json<Post>,
) -> impl IntoResponse {
    match post_service.update_post(body).await {
        Ok(data) => Ok((
            StatusCode::OK,
            json!({"success": true, "data": data}).to_string()
        )),
        Err(err) => Err(err)
    }
}
async fn delete_post(
    State(post_service): State<PostService>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match post_service.delete_post(id).await {
        Ok(data) => Ok((
            StatusCode::OK,
            json!({"success": true, "data": data}).to_string()
        )),
        Err(err) => Err(err)
    }
}