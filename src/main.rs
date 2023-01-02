use axum::{
    http::{StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    //build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/html", get(index))
        .route("/user_info", get(user_info))
        .route("/users", post(create_user));

    //run our app with hyper on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn index() -> Html<&'static str> {
    let html = r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>My API</title>
            </head>
            <body>
                <h1>Welcome to My Api</h1>
            </body>
        </html>
    "#;
    axum::response::Html(html)
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Auth {
    pub exp: i64,
    pub iat: i64,
    pub jti: String,
    pub iss: String,
    pub aud: Vec<String>,
    pub sub: String,
    pub typ: String,
    pub azp: String,
    pub session_state: String,
    pub given_name: String,
    pub family_name: String,
    pub preferred_username: String,
    pub email: String,
    pub email_verified: bool,
    pub scope: String,
    pub sid: String,
    pub created_at: i64,
    pub client_id: String,
    pub username: String,
    pub active: bool,
}


async fn user_info() -> Response {
    let params = [("Content-Type", "application/x-www-form-urlencoded")];
    let client = Client::new();
    let response = client.post("https://keycloak")
        .form(&params)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("body content")
        .send()        
        .await.unwrap()
        .json::<Auth>()
        .await.unwrap();

    println!("Body: {:?}", &response);
   Json(response).into_response()
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
