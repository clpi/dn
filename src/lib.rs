use axum::{
    prelude::*, response::IntoResponse,
};
use serde::{Serialize, Deserialize};
use std::io;

pub async fn init() -> io::Result<()> {
    tracing_subscriber::fmt::init();
    let app = route("/", get(root))
        .route("/users", post(create_user));
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("Listening on {}", addr);
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())    
}


async fn root() -> &'static str {
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

#[derive(Serialize, Default, Deserialize)]
struct User {
    #[serde(skip_serializing)]
    id: u64,
    email: String,
    username: String,
}
