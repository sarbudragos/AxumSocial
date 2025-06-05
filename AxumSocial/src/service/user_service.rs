use axum::http::StatusCode;
use crate::model::user::{NewUser, User, UserData};
use crate::model::user_follow::UserFollow;
use crate::repository::user_follow_repository::UserFollowRepository;
use crate::repository::user_repository::UserRepository;

#[derive(Clone)]
pub struct UserService{
    user_repository: UserRepository,
    user_follow_repository: UserFollowRepository,
}

impl UserService{
    pub fn new(user_repository:  UserRepository, user_follow_repository: UserFollowRepository) -> Self{
        Self{
            user_repository,
            user_follow_repository,
        }
    }
    pub async fn get_users(&self) -> Result<Vec<UserData>, (StatusCode, String)>{
        self.user_repository.get_all().await
            .map(
                |users| users.into_iter().map(UserData::from_user).collect()
            )
    }
    pub async fn get_user_by_id(&self, id:i32) -> Result<UserData, (StatusCode, String)>{
        self.user_repository.get_one(id).await
            .map(UserData::from_user)
    }

    pub async fn add_user(&self,user: NewUser) -> Result<User, (StatusCode, String)>{
        self.user_repository.add(user).await
    }

    pub async fn update_user(&self, user: User) -> Result<User, (StatusCode, String)>{
        self.user_repository.update(user).await
    }

    pub async fn delete_user(&self, id:i32) -> Result<User, (StatusCode, String)>{
        self.user_repository.delete(id).await
    }

    pub async fn get_followers(&self, user_id: i32) -> Result<Vec<UserData>, (StatusCode, String)>{
        self.user_follow_repository.get_followers(user_id).await
            .map(
                |users| users.into_iter().map(UserData::from_user).collect()
            )
    }

    pub async fn get_following_users(&self, user_id: i32) -> Result<Vec<UserData>, (StatusCode, String)>{
        self.user_follow_repository.get_following(user_id).await
            .map(
                |users| users.into_iter().map(UserData::from_user).collect()
            )
    }

    pub async fn add_follow(&self, follow: UserFollow) -> Result<UserFollow, (StatusCode, String)>{
        self.user_follow_repository.add_follow(follow).await
    }

    pub async fn remove_follow(&self, follow: UserFollow) -> Result<UserFollow, (StatusCode, String)> {
        self.user_follow_repository.remove_follow(follow).await
    }
}