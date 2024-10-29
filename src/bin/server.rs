use axum::{routing::get, Router};
use std::io;
use tokio::net::TcpListener;
use tower_http::services::ServeFile;

use unorder::orders;

#[tokio::main]
async fn main() -> io::Result<()> {
    let tcp = TcpListener::bind("localhost:3000").await?;

    let route = Router::new()
        .nest(
            "/orders",
            Router::new()
                .route("/", get(orders::Views::show))
                .route("/new", get(orders::Views::new)),
        )
        .route_service("/hx.js", ServeFile::new("static/htmx.js"));

    axum::serve(tcp, route).await
}
