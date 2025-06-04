use axum::http::StatusCode;
use deadpool::managed::Object;
use diesel::{delete, insert_into, update, QueryDsl, SelectableHelper};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use serde_json::json;
use crate::model::user::{NewUser, User};
use crate::schema::users::dsl::users;
use crate::schema::users::id;
use diesel::ExpressionMethods;

#[derive(Clone)]
pub struct UserRepository{
    db:  Pool<AsyncPgConnection>
}

impl UserRepository{
    pub fn new(db: Pool<AsyncPgConnection>) -> Self{
        Self{ db }
    }

    async fn get_connection(&self) -> Object<AsyncDieselConnectionManager<AsyncPgConnection>> {
        self.db.get().await.unwrap()
    }

    pub async fn get_all(&self) -> Result<Vec<User>, (StatusCode, String)>{
        users
            .select(User::as_select())
            .load(&mut self.get_connection().await)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({"success": false, "message": e.to_string()}).to_string(),
                )
            })
    }

    pub async fn get_page(&self, page_number: i32, page_size: i32) -> Result<Vec<User>, (StatusCode, String)>{
        users
            .select(User::as_select())
            .limit(page_size as i64)
            .offset((page_number * page_size) as i64)
            .load(&mut self.get_connection().await)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({"success": false, "message": e.to_string()}).to_string(),
                )
            })
    }

    pub async fn get_one(&self, other_id: i32) -> Result<User, (StatusCode, String)>{
        users
            .filter(id.eq(&other_id))
            .select(User::as_select())
            .first(&mut self.get_connection().await)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({"success": false, "message": e.to_string()}).to_string(),
                )
            })
    }

    pub async fn add(&self, user: NewUser) -> Result<User, (StatusCode, String)>{
        insert_into(users)
            .values(user)
            .get_result(&mut self.get_connection().await)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({"success": false, "message": e.to_string()}).to_string(),
                )
            })
    }

    pub async fn update(&self, user: User) -> Result<User, (StatusCode, String)>{
        update(&user)
            .set(&user)
            .get_result(&mut self.get_connection().await)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({"success": false, "message": e.to_string()}).to_string(),
                )
            })
    }

    pub async fn delete(&self, user_id: i32) -> Result<User, (StatusCode, String)> {
        delete(users.filter(id.eq(user_id)))
            .get_result(&mut self.get_connection().await)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({"success": false, "message": e.to_string()}).to_string(),
                )
            })
    }
}