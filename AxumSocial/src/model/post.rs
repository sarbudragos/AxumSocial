use chrono::NaiveDateTime;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use crate::model::user::User;

#[derive(Serialize)]
#[derive(Queryable, Selectable, Identifiable, AsChangeset, Associations,)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    id: i32,
    content: String,
    creation_date: NaiveDateTime,
    user_id: i32,
}

impl Post {
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_content(&self) -> String {
        self.content.clone()
    }
    pub fn get_creation_date(&self) -> NaiveDateTime {
        self.creation_date
    }
    pub fn set_creation_date(&mut self, creation_date: NaiveDateTime) {
        self.creation_date = creation_date;
    }
    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }
}

#[derive(Deserialize,Insertable, Associations,)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(belongs_to(User))]
pub struct NewPost {
    content: String,
    creation_date: NaiveDateTime,
    user_id: i32,
}

impl NewPost {
    pub fn set_creation_date(&mut self, creation_date: NaiveDateTime) {
        self.creation_date = creation_date;
    }
}