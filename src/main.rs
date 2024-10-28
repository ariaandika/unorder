use std::io;
use axum::{routing::get, Json, Router};
use serde::Serialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let tcp = TcpListener::bind("localhost:3000").await?;

    let route = Router::new()
        .route("/", get(route));

    axum::serve(tcp, route).await
}


#[derive(Serialize)]
struct Post {
    name: String,
}

async fn route() -> Json<Post> {
    Post { name: "Adb".into() }.into()
}


