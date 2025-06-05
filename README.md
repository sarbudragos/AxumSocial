# AxumSocial
Framework design project by Dragos Sarbu

## Introduction

This project is an example application on how to create a REST service in Rust using the Axum framework and the Diesel ORM. More specifically, the focus is on highlighting how to use the Diesel ORM as well as best practices when using the Axum framework.

For demonstration purposes, this project also contains a barebones React web-app that utilizes the REST API. This react app is not the focus of the project and doesn't even utilize 100% of the api.

The API itself is a mock-up of a social media platform, hence the project name, Axum Social. There are two main entities, users and posts, with each post belonging to a user. Users can also follow one another.

## Technologies used

For the database: 
- PostgreSQL

For the backend: 
- Rust
- Axum
- Diesel ORM

For the frontend:
- React

## Installation

These installation instructions do not cover:
- How to set-up a PostgreSQL database
- How to install diesel-cli
- How to build and run Rust programs
- How to install node/npm in order to run a React application#[derive(Clone)]

## Database

This application uses a PostgreSQL database. Deploy one however you see fit and create a new empty database. Obtain the connection string. It should look like this:

> postgres://{username}:{password}@{database url or ip address}:{port}/{database name}

If everything is in order you can move on to setting up the backend.

### Backend

Firstly, clone this repository if you haven't already. Then go into the AxumSocial directory, which contains the backend. 

Before doing anything, create a .env file with the following contents.

> DATABASE_URL='YOUR CONNECTION STRING'

> SERVER_ADDRESS= 'THE ADDRESS FOR YOU SERVER'

If SERVER_ADDRESS is not specified it will default to '127.0.0.1:8080'

Now install diesel-cli. After successfully installing, run in the terminal in the AxumSocial directory:

> diesel setup

If you succesfully installed both the Postgres database as well as diesel-cli this command should fully set-up diesel and run all existing migrations.

At this point you should be able to build and run the backend like any other Rust program.

### Frontend

Go into the axum-social-react directory. 

Go to src/consts.ts and change:

> export const BASE_URL = "http://localhost:8080/api"

By replacing localhost:8080 with your server address.

Make sure you are running an up-to-date version of npm (I used 11.3.0). Run the following command:

> npm install

This will download all dependencies. Then to run the frontend use:

> npm run dev

## Walkthrough

The backend has a standard layered architecture. This walkthrough section focuses mainly on the User entity, with the post entity being handled in a very similar way.

Firstly, starting from src/model/user.rs we have:

```rust
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
```

For anyone unfamiliar with Rust, this is how a typical struct declaration looks like. It is important to note that, despite Rust not being an object-oriented language, structs can do almost anything that objects can in other languages (except inheritance - for simmilar results we use traits and composition).

Everything above should be fairly self-explanatory, except for the lines above the User definition. Those are macros. The first 2 macros are derrive macros. Derrive is one of the most useful macros in all of Rust. As the name implies, it lets us derrive traits (somewhat simmilar to interfaces in languages like Java) without having to implement them manually. It's not magic, it usually has certain requirements, such as "every field must already implement the trait". The Serialize trait, for example, defines how the struct is serialized into JSON.

The last 2 macros are related to Diesel and are basically boilerplate. #[diesel(table_name = crate::schema::users)] associates our struct with a SQL table in the schema.

Moving on to the src/repository/user_repository.rs we have:

```rust
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
    //.....
}
```

Here we define UserRepository, which derrives clone and contains our database connection (that we later inject). Here we also have an example of a constructor in Rust. The Rust language doesn't have built in support for constructors, but any 'static method' that returns Self is considered a constructor.

The get_all function is an example of how to use Diesel in order to make queries much easier. We take the users table (imported from schema.rs) and call .select(User::as_select()) which is 'SELECT * FROM', pass the connection with load and await. We use map_err to modify the formatting of potential errors to something more convenient.

In src/service/user_service.rs :

```rust
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
    //......
}
```

Here we have the service definition, constructor and get_users function. There is also another repository for UserFollows(the table for storing follower relations between users) but we wont go in depth on that. Everything here is just more of the same, except for the fact that in the get_users function we map the users to UserData structs, a DTO created in order to hide the password field.

Then we have the controller in src/controller/user_controller.rs :

```rust
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
    //...
}
```

There's a lot to unpack here. First of all, the controller is not a struct, but a function that returns a router. This is because Axum has a very specific way of doing routing and this is the most sensible way of satisfying its requirements. We won't go into the why here, just know this is how you write controllers in Axum. The get_users function, along with the other controllers are very basic. They just call the service and use a match statement for formatting reasons.

Lastly theres main.rs where we initialize everything and do dependency injection:

```rust
#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access .env file");

    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:8080".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env file");

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&database_url);
    let pool = Pool::builder(config)
        .build()
        .unwrap_or_else(
            |_| panic!("Error connecting to {}", database_url)
        );

    let user_repository = UserRepository::new(pool.clone());
    let user_follow_repository = UserFollowRepository::new(pool.clone());
    let post_repository = PostRepository::new(pool.clone());

    let user_service = UserService::new(user_repository.clone(), user_follow_repository.clone());
    let post_service = PostService::new(post_repository.clone(), user_repository.clone());

    let user_controller = user_controller_router(user_service.clone());
    let post_controller = post_controller_router(post_service.clone());

    let router = Router::new()
        .nest("/api/users", user_controller)
        .nest("/api/posts", post_controller)
        .route("/", get(hello))
        .layer(CorsLayer::permissive());

    let listener = TcpListener::bind(server_address)
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router)
        .await
        .unwrap();
}
```

Nothing special is happening here we get DATABASE_URL and SERVER_ADDRESS from our .env, perform some dependency injection and call axum::serve(listener, router).await.unwrap(); to properly start the server.

## Conclusion

And that's it! If you read and understood the walkthrough and proceded to install and read through the rest of the backend, you should have all the tools at your disposal to start building your own REST services in Rust.

I say that, but there's still a lot of things like authentication or web sokets that are outside the scope of this project. You're going to have to figure those out on your own. Good luck!