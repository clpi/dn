// dnet experimental service
// other services: io/data sevice, p2p service, crawler
pub mod error;

use axum::{
    extract::FromRequest, prelude::*, 
    response::{Html, IntoResponse, Json},
};
use serde_json::{json, Value};
use std::{io, collections::HashMap};
use serde::{Serialize, Deserialize};

pub async fn init() -> io::Result<()> {
    tracing_subscriber::fmt::init();
    let mut app = route("/", get(index))
        .route("/health", get(health))
        .route("/auth", get(get_auth))
        .route("/group", get(get_groups))
        .route("/users", get(get_users).post(create_user).delete(clear_users))
        .route("/channel", get(health))
        .route("/user/:id", get(get_user_by_id));
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("Listening on {}", addr);
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())    
}

async fn get_auth(tok: ApiToken) -> String {
    format!("Token is {:#?}", &tok)
}
async fn index() -> Html<&'static str> {
    axum::response::Html("<h1>dnet index</h1>")
}

async fn health() -> &'static str {
    "hi there"
}

async fn create_user(
    extract::Json(payload): extract::Json<User>
) -> impl IntoResponse {
    let user = User {
        id: 1337,
        email: payload.email,
        username: payload.username
    };
    (http::StatusCode::CREATED, response::Json(user))
}

async fn get_users(_query: extract::Query<HashMap<String, String>>) -> Json<Value> {
    Json(json!(  { "user": "user", "email": "user@email.com", "id": 1 }))

}

async fn get_user_by_id() -> Result<Json<Value>, http::StatusCode> {
    Ok(Json(json!( {"user": 1})))
}

async fn clear_users() {}

async fn get_groups(_query: Option<extract::Query<GroupQuery>>) {}

#[derive(Serialize, Default, Deserialize)]
struct User {
    #[serde(skip_serializing)]
    id: u64,
    email: String,
    username: String,
}

#[derive(Debug)]
pub struct ApiToken(http::HeaderValue);

impl std::ops::Deref for ApiToken {
    type Target = http::HeaderValue;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AsRef<http::HeaderValue> for ApiToken {
    fn as_ref(&self) -> &http::HeaderValue {
        &self.0
    }
}

#[async_trait::async_trait]
impl<B: Send> FromRequest<B> for ApiToken {
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request(req: &mut extract::RequestParts<B>) -> Result<Self, Self::Rejection> {
        let token = req.headers()
            .and_then(|headers| headers.get("DNET_API_TOKEN"));
        if let Some(token) = token {
            Ok(ApiToken(token.clone()))
        } else {
            Err((http::StatusCode::BAD_REQUEST, "No api token"))
        }
        
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct GroupQuery {
    group_id: usize,
    user_id: usize,
    email: String,
}
