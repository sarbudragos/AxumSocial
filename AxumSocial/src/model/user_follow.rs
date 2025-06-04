use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use diesel::sql_types::Uuid;
use serde::{Deserialize, Serialize};
use crate::model::user::User;

#[derive(Serialize)]
#[derive(Queryable, Selectable, AsChangeset, Deserialize,Insertable,)]
#[diesel(table_name = crate::schema::user_follows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserFollow {
    pub(crate) follower_id: i32,
    pub(crate) following_user_id: i32,
}