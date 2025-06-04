use axum::http::StatusCode;
use diesel_async::RunQueryDsl;
use crate::model::post::{NewPost, Post};
use crate::model::user::User;
use crate::repository::post_repository::PostRepository;
use crate::repository::user_repository::UserRepository;
use crate::schema::posts::dsl::posts;

#[derive(Clone)]
pub struct PostService{
    post_repository:  PostRepository,
    user_repository:  UserRepository,
}

impl PostService{
    pub fn new(post_repository:  PostRepository, user_repository:  UserRepository) -> Self{
        Self{
            post_repository,
            user_repository,
        }
    }

    pub async fn get_posts(&self) -> Result<Vec<Post>, (StatusCode, String)>{
        self.post_repository.get_all().await
    }

    pub async fn get_post_by_id(&self, id: i32) -> Result<Post, (StatusCode, String)>{
        self.post_repository.get_one(id).await
    }

    pub async fn get_user_of_post(&self, post_id: i32) -> Result<User, (StatusCode, String)>{
        let post = self.post_repository.get_one(post_id).await.unwrap();

        self.user_repository.get_one(post.get_user_id()).await
    }

    pub async fn get_posts_of_user_sorted(&self, user_id: i32) -> Result<Vec<Post>, (StatusCode, String)>{
        // self.post_repository.get_posts_of_user(user_id).await.map(
        //     |mut posts| {
        //         let sorted_posts = posts.clone().sort_by(|post1, post2| { post1.get_creation_date().cmp(&post2.get_creation_date()) })
        //         sorted_posts
        //     }
        // )

        // match self.post_repository.get_posts_of_user(user_id).await {
        //     Ok(mut posts) => {
        //         posts.sort_by_key(Post::get_creation_date)
        //     }
        // }

        let mut unsorted_posts = self.post_repository.get_posts_of_user(user_id).await?;

        unsorted_posts.sort_by_key(Post::get_creation_date);

        Ok(unsorted_posts)
    }

    pub async fn add_post(&self, new_post: NewPost) -> Result<Post, (StatusCode, String)>{
        self.post_repository.add(new_post).await
    }

    pub async fn update_post(&self, post: Post) -> Result<Post, (StatusCode, String)>{
        self.post_repository.update(post).await
    }

    pub async fn delete_post(&self, post_id: i32) -> Result<Post, (StatusCode, String)>{
        self.post_repository.delete(post_id).await
    }
}