use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router, extract::State,
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use std::{net::SocketAddr, vec, sync::{Arc, Mutex}};


/*
    Routes
        * upload_file
        * get_thumbnail
        * stream_video thing
        * user login stuff - no idea what do here
*/

#[derive(Clone)]
struct UserTable {
    data: Arc<Mutex<Vec<String>>>,
}



#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    
    let state = UserTable { data: Arc::new(Mutex::new(vec![]))};
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/upload", post(upload))
        .route("/listusers", get(list_users))
        .route("/users", post(create_user))
        .with_state(state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World watch is working!"
}

async fn upload() {

}

// take in a filename
async fn get_thumbnail() {

}

async fn list_thumbnails() {

}

async fn list_users(
    State(state): State<UserTable>,
) -> (StatusCode, Json<Vec<String>>) {
    (StatusCode::OK, Json(state.data.lock().expect("mutext was poisened").to_vec()))
}

async fn create_user(
    State(state): State<UserTable>,
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };
    let mut data = state.data.lock().expect("mutext was poisened");
    data.push(user.username.clone());

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct UploadFile {
    // multipart upload stuff - bytes?
}


#[derive(Deserialize)]
struct ListThumbnails {
    username: String
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

