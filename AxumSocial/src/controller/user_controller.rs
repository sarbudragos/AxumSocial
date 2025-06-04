use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::{delete, get, post, put};
use serde_json::json;
use crate::model::user::{CreateUserResult, NewUser, UpdateUserBody};
use crate::model::user_follow::UserFollow;
use crate::service::user_service::UserService;

pub fn user_controller_router(user_service: UserService) -> Router{
    Router::new()
        .route("/", get(get_users))
        .route("/{id}", get(get_user))
        .route("/", post(add_user))
        .route("/{id}", put(update_user))
        .route("/{id}", delete(delete_user))
        .route("/{id}/followers", get(get_followers))
        .route("/{id}/following", get(get_following))
        .route("/follow", post(add_follow))
        .route("/follow", delete(delete_follow))
        .with_state(user_service)
}

async fn get_users(
    State(user_service): State<UserService>

) -> impl IntoResponse {
    println!("GET /users");
    match user_service.get_users().await {
        Ok(data) => Ok((
            StatusCode::OK,
            json!({"success": true, "data": data}).to_string()
        )),

        Err(err) => Err(err)
    }
}

async fn get_user(
    State(user_service) : State<UserService>,
    Path(id): Path<i32>
) -> impl IntoResponse{
    match user_service.get_user_by_id(id).await{
        Ok(data) => Ok((
            StatusCode::OK,
            json!({"success": true, "data": data}).to_string()
        )),
        Err(err) => Err(err)
    }
}

async fn add_user(
    State(user_service): State<UserService>,
    Json(body): Json<NewUser>,
) -> impl IntoResponse {
    println!("POST /users");
    match user_service.add_user(body).await{
        Ok(data) => Ok((
            StatusCode::OK,
            json!(CreateUserResult::from_user(data)).to_string()
        )),
        Err(err) => Err(err)
    }
}

async fn update_user(
    State(user_service): State<UserService>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateUserBody>,
) -> impl IntoResponse {
    println!("PUT /users");
    let mut user = body.to_user();
    user.set_id(id);
    match user_service.update_user(user).await{
        Ok(data) => Ok((
            StatusCode::OK,
            json!(CreateUserResult::from_user(data)).to_string()
        )),
        Err(err) => Err(err)
    }
}

async fn delete_user(
    State(user_service): State<UserService>,
    Path(id): Path<i32>
) -> impl IntoResponse {
    println!("DELETE /users");
    match user_service.delete_user(id).await{
        Ok(_) => Ok((
            StatusCode::OK,
            json!({"success": true}).to_string()
        )),
        Err(err) => Err(err)
    }
}

async fn get_followers(
    State(user_service): State<UserService>,
    Path(id): Path<i32>
) -> impl IntoResponse{
    println!("GET /users/followers");

    match user_service.get_followers(id).await{
        Ok(data) => Ok((
            StatusCode::OK,
            json!(data).to_string()
        )),
        Err(err) => Err(err)
    }
}

async fn get_following(
    State(user_service): State<UserService>,
    Path(id): Path<i32>
) -> impl IntoResponse{
    println!("GET /users/following");

    match user_service.get_following_users(id).await {
        Ok(data) => Ok((
            StatusCode::OK,
            json!(data).to_string()
        )),
        Err(err) => Err(err)
    }
}

async fn add_follow(
    State(user_service): State<UserService>,
    Json(body): Json<UserFollow>,
) -> impl IntoResponse{
    println!("POST /follow");

    match user_service.add_follow(body).await{
        Ok(data) => Ok((
            StatusCode::OK,
            json!(data).to_string()
        )),
        Err(err) => Err(err)
    }
}

async fn delete_follow(
    State(user_service): State<UserService>,
    Json(body): Json<UserFollow>
) -> impl IntoResponse{
    println!("DELETE /follow");

    match user_service.remove_follow(body).await {
        Ok(_) => Ok((
            StatusCode::OK,
            json!({"success": true}).to_string()
        )),
        Err(err) => Err(err)
    }
}