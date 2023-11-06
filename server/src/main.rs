use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router, extract::State,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::{instrument, event, Level};
use std::{net::SocketAddr, vec, sync::{Arc}};
use uuid::Uuid;


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

#[derive(Clone)]
struct UserTable {
    data: Arc<Mutex<Vec<User>>>,
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
        .route("/friends/request", post(send_friend_request))
        .with_state(state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!(">> LISTENING on {addr}\n");
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
) -> (StatusCode, Json<Vec<User>>) {
    (StatusCode::OK, Json(state.data.lock().await.to_vec()))
}

async fn create_user(
    State(state): State<UserTable>,
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    let user_index = get_user_index(state.data.clone(), &payload.username).await;
    if user_index.is_some() {
        return Err((StatusCode::BAD_REQUEST, "Username already taken".to_string()));
    } 
    let mut data = state.data.lock().await;
    let user = User {
        id: Uuid::new_v4(),
        username: payload.username,
        friends_list: vec![],
        friend_requests: vec![]
    };
    data.push(user.clone());
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok((StatusCode::CREATED, Json(user)))
}


async fn send_friend_request(
    State(state): State<UserTable>, Json(payload): Json<SendFriendRequest>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let target_user_index = get_user_index(state.data.clone(), &payload.target_username).await;
    let current_user_index = get_user_index(state.data.clone(), &payload.current_username).await;

    if target_user_index.is_none() || current_user_index.is_none() { 
        return Err((StatusCode::BAD_REQUEST, "User not found".to_string()));
    }
    let mut users = state.data.lock().await;
    users.get_mut(target_user_index.unwrap()).unwrap().friend_requests.push(
        FriendRequest {
            status: RequestStatus::PendingResponse,
            rtype: RequestType::Received,
            username: payload.current_username.clone()
        }
    );
    users.get_mut(current_user_index.unwrap()).unwrap().friend_requests.push(
        FriendRequest {
            status: RequestStatus::PendingAccept,
            rtype: RequestType::Sent,
            username: payload.target_username.clone()
        }
    );
    Ok((StatusCode::OK, "Friend request sent".to_string()))
    
}

async fn get_user_index(users: Arc<Mutex<Vec<User>>>, username: &String) -> Option<usize> {
    println!("User {username}");
    let users = users.lock().await;
    let inner: &Vec<User> = &*users;
    inner.into_iter().position(|user| user.username == *username)
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

#[derive(Deserialize)]
// TODO: derive current_username from auth info
struct SendFriendRequest {
    current_username: String,
    target_username: String,
}

// the output to our `create_user` handler
#[derive(Serialize, Clone, Debug)]
struct User {
    id: Uuid,
    username: String,
    friends_list: Vec<User>,
    friend_requests: Vec<FriendRequest>
}

#[derive(Serialize, Clone, Debug)]
struct FriendRequest {
    status: RequestStatus,
    rtype: RequestType,
    username: String

}

#[derive(Serialize, Clone, Debug)]
enum RequestType {
    Sent,
    Received
}

#[derive(Serialize, Clone, Debug)]
enum RequestStatus {
    PendingAccept,
    PendingResponse,
}

// call SendFriendRequest specifying a username
// this will send a FriendRequest -> add a FriendRequest object to target User with
// PENDING_RESPONSE
// also add a FriendRequest object to current User with PENDING_ACCEPT

