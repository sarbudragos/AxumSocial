use axum::http::StatusCode;
use chrono::{ Utc};
use deadpool::managed::Object;
use diesel::{delete, QueryDsl, SelectableHelper, update};
use diesel::dsl::insert_into;
use diesel_async::pooled_connection::deadpool::{Pool};
use serde_json::json;
use crate::model::post::{NewPost, Post};
use crate::schema::posts::dsl::posts;
use diesel_async::{RunQueryDsl, AsyncPgConnection,};
use crate::schema::posts::{id, user_id};
use diesel::ExpressionMethods;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;

#[derive(Clone)]
pub struct PostRepository{
    db:  Pool<AsyncPgConnection>
}

impl PostRepository {
    pub fn new(db: Pool<AsyncPgConnection>) -> Self {
        Self { db }
    }

    async fn get_connection(&self) -> Object<AsyncDieselConnectionManager<AsyncPgConnection>> {
        self.db.get().await.unwrap()
    }

    pub async fn get_all(&self)-> Result<Vec<Post>, (StatusCode, String)>{
        posts
            .select(Post::as_select())
            .load(&mut self.get_connection().await)
            .await
            .map_err(
                |e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        json!({"success": false, "message": e.to_string()}).to_string(),
                    )
                }
            )
    }

    pub async fn get_one(&self, other_id: i32) -> Result<Post, (StatusCode, String)>{
        posts
            .select(Post::as_select())
            .filter(id.eq(&other_id))
            .first(&mut self.get_connection().await)
            .await
            .map_err(
                |e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        json!({"success": false, "message": e.to_string()}).to_string(),
                    )
                }
            )
    }

    pub async fn get_posts_of_user(&self, user_id_value: i32) -> Result<Vec<Post>, (StatusCode, String)>{
        posts
            .select(Post::as_select())
            .filter(user_id.eq(&user_id_value))
            .load(&mut self.get_connection().await)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({"success": false, "message": e.to_string()}).to_string(),
                )
            })
    }

    pub async fn add(&self, mut new_post: NewPost) -> Result<Post, (StatusCode, String)>{
        new_post.set_creation_date(Utc::now().naive_utc());

        insert_into(posts)
            .values(new_post)
            .get_result(&mut self.get_connection().await)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({"success": false, "message": e.to_string()}).to_string(),
                )
            })
    }

    pub async fn update(&self, post: Post) -> Result<Post, (StatusCode, String)>{
        update(&post)
            .set(&post)
            .get_result(&mut self.get_connection().await)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({"success": false, "message": e.to_string()}).to_string(),
                )
            })
    }

    pub async fn delete(&self, post_id: i32) -> Result<Post, (StatusCode, String)>{
        delete(posts.filter(id.eq(post_id)))
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