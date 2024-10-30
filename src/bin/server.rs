use axum::{routing::get, Router};
use std::io;
use tokio::net::TcpListener;
use tower_http::services::ServeFile;

use unorder::{orders, auth};

#[tokio::main]
async fn main() -> io::Result<()> {
    let tcp = TcpListener::bind("localhost:3000").await?;

    let route = Router::new()
        .route("/login", get(auth::Views::view).post(auth::Views::login))
        .nest(
            "/orders",
            Router::new()
                .route("/", get(orders::Views::show))
                .route("/new", get(orders::Views::new)),
        )
        .route_service("/hx.js", ServeFile::new("target/hx.js"));

    axum::serve(tcp, route).await
}
