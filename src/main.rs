use std::{convert::Infallible, io};
use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{FromRequestParts, Query}, http::request, response::Response, routing::get, Router
};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::services::ServeFile;

#[tokio::main]
async fn main() -> io::Result<()> {
    let tcp = TcpListener::bind("localhost:3000").await?;

    let route = Router::new()
        .route("/", get(home))
        .route_service("/hx.js", ServeFile::new("static/htmx.js"));

    axum::serve(tcp, route).await
}



#[derive(Template)]
#[template(path = "layout.html", escape = "none")]
struct Document<T> where T: Template {
    body: T,
}

#[derive(Template, Default, Deserialize)]
#[template(path = "home.html")]
struct Home {
    count: i32
}

async fn home(qs: Option<Query<Home>>, render: RootLayout) -> Response {
    let body = if let Some(c) = qs { Home { count: c.count + 1 } } else { Home::default() };

    render.render(body)
}

struct RootLayout {
    is_hx: bool,
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for RootLayout {
    type Rejection = Infallible;
    async fn from_request_parts(parts: &mut request::Parts, _: &S) -> Result<Self,Self::Rejection> {
        Ok(Self { is_hx: parts.headers.get("hx-request").is_some() })
    }
}

impl RootLayout {
    fn render<T>(&self, body: T) -> Response where T: Template {
        if self.is_hx {
            askama_axum::into_response(&body)
        } else {
            Document { body }.into_response()
        }
    }
}

