use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool, Pool, Postgres, Row,
};
use std::{net::SocketAddr};

/*
    Routes
        * upload_file
        * get_thumbnail
        * stream_video thing
        * user login stuff - no idea what do here
    TODO
        * User Login
            * CreateUser API called and user is registered
            * Various user-specific requests require login such as
            * SendFriendRequest
            * RemoveFriend
            * ListFriends
*/

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect("postgres://chance:chance@localhost:5432/cleepdb")
        .await
        .unwrap();
    let app = Router::new()
        .route("/", get(welcome))
        .route("/users", get(list_users).post(create_user))
        .with_state(pool);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!(">> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn welcome(
    TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>
) ->Result<(StatusCode, String), (StatusCode, String)> {
    Ok((StatusCode::OK, format!("Welcome {}!", auth_header.token())))
}

async fn upload() {}

// take in a filename
async fn get_thumbnail() {}



async fn list_thumbnails(
    pool: State<PgPool>,
    TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>
) -> Result<(StatusCode, Json<Vec<User>>), (StatusCode, String)> {
    println!("{}", auth_header.token());
    Ok((StatusCode::OK, Json(vec![])))

}

// TODO (crebh): impl pagination
async fn list_users(
    pool: State<PgPool>,
) -> Result<(StatusCode, Json<Vec<User>>), (StatusCode, String)> {
    let users = sqlx::query("SELECT * FROM Users")
        .map(|row: PgRow| User {
            id: row.get("userid"),
            username: row.get("username"),
        })
        .fetch_all(&*pool)
        .await
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(users)))
}

async fn create_user(
    pool: State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    let user = sqlx::query("INSERT INTO Users (username) VALUES ($1) RETURNING *")
        .bind(payload.username)
        .map(|row: PgRow| User {
            id: row.get("userid"),
            username: row.get("username"),
        })
        .fetch_one(&*pool)
        .await
        .map_err(
            |err| err.into_database_error().map_or(
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went horribly wrong".to_string()),
                |err| {
                    if err.is_unique_violation() {
                        return (StatusCode::BAD_REQUEST, "Username already taken".to_string())
                    }
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                }))?;

    Ok((StatusCode::CREATED, Json(user)))
}

async fn create_friend_request(
    pool: State<PgPool>,
    Json(payload): Json<SendFriendRequest>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    Ok((StatusCode::OK, "Friend request sent".to_string()))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[derive(Deserialize)]
struct UploadFile {
    // multipart upload stuff - bytes?
}

#[derive(Deserialize)]
struct ListThumbnails {
    username: String,
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Deserialize)]
// TODO: derive current_username from auth info
struct SendFriendRequest {
    current_username: String,
    target_username: String,
}

// the output to our `create_user` handler
#[derive(Serialize, Clone, Debug, sqlx::FromRow)]
struct User {
    id: i32,
    username: String,
}

#[derive(Serialize, Clone, Debug)]
struct FriendRequest {
    status: RequestStatus,
    rtype: RequestType,
    username: String,
}

#[derive(Serialize, Clone, Debug)]
enum RequestType {
    Sent,
    Received,
}

#[derive(Serialize, Clone, Debug)]
enum RequestStatus {
    PendingAccept,
    PendingResponse,
}
