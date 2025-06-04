use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: i32,
    username: String,
    email: String,
    password: String,
}

impl User {
    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_email(&self) -> String {
        self.email.clone()
    }

    pub fn get_password(&self) -> String {
        self.password.clone()
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }
}

#[derive(Serialize)]
pub struct UserData {
    id: i32,
    username: String,
    email: String,
}

impl UserData {
    pub fn from_user(user: User) -> Self {
        UserData{
            id: user.get_id(),
            username: user.get_username(),
            email: user.get_email(),
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_email(&self) -> String {
        self.email.clone()
    }
}

#[derive(Deserialize,Insertable,)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    username: String,
    email: String,
    password: String,

}

#[derive(Serialize)]
pub struct CreateUserResult {
    id: i32,

}

impl CreateUserResult{
    pub fn from_user(user: User) -> Self{
        Self{id: user.id}
    }
}

#[derive(Deserialize)]
pub struct UpdateUserBody {
    username: String,
    email: String,
    password: String,

}

impl UpdateUserBody{
    pub fn to_user(&self) -> User{
        User{
            id: 0,
            username: self.username.clone(),
            email: self.email.clone(),
            password: self.password.clone(),
        }
    }
}