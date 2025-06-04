use axum::http::StatusCode;
use deadpool::managed::Object;
use diesel::{QueryDsl, SelectableHelper};
use diesel::dsl::{delete, insert_into};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use serde_json::json;
use crate::model::user::User;
use crate::model::user_follow::UserFollow;
use crate::schema::user_follows::dsl::user_follows;
use crate::schema::user_follows::{follower_id, following_user_id};
use crate::schema::users::{email, id};
use diesel::ExpressionMethods;
use crate::schema::users::dsl::users;

#[derive(Clone)]
pub struct UserFollowRepository{
    db:  Pool<AsyncPgConnection>
}

impl UserFollowRepository{
    pub fn new(db: Pool<AsyncPgConnection>) -> Self{ Self{ db } }

    pub async fn get_connection(&self) -> Object<AsyncDieselConnectionManager<AsyncPgConnection>> {
        self.db.get().await.unwrap()
    }

    pub async fn get_users_from_id_list(&self, user_id_list: Vec<i32>) -> Result<Vec<User>, (StatusCode, String)> {
        users
            .filter(id.eq_any(user_id_list))
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

    pub async fn get_followers(&self, user_id: i32) -> Result<Vec<User>, (StatusCode, String)> {
        let follows: Vec<i32> = user_follows
            .filter(following_user_id.eq(user_id))
            .select(follower_id)
            .load(&mut self.get_connection().await)
            .await.unwrap();

        println!("{:?}", follows);

        self.get_users_from_id_list(follows).await
    }

    pub async fn get_following(&self, user_id: i32) -> Result<Vec<User>, (StatusCode, String)> {
        let follows: Vec<i32> =  user_follows
            .filter(follower_id.eq(user_id))
            .select(following_user_id)
            .load(&mut self.get_connection().await)
            .await.unwrap();

        println!("{:?}", follows);

        self.get_users_from_id_list(follows).await
    }

    pub async fn add_follow(&self, user_follow: UserFollow) -> Result<UserFollow, (StatusCode, String)> {
        insert_into(user_follows)
            .values(user_follow)
            .get_result(&mut self.get_connection().await)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({"success": false, "message": e.to_string()}).to_string(),
                )
            })
    }

    pub async fn remove_follow(&self, user_follow: UserFollow) -> Result<UserFollow, (StatusCode, String)> {
        delete(user_follows)
            .filter(follower_id.eq(user_follow.follower_id))
            .filter(following_user_id.eq(user_follow.following_user_id))
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