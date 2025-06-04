-- Your SQL goes here
create table user_follows(
    follower_id INT NOT NULL REFERENCES users(id),
    following_user_id INT NOT NULL REFERENCES users(id),
    PRIMARY KEY (follower_id, following_user_id)
)