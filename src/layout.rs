use askama::Template;
use askama_axum::{IntoResponse, Response};
use axum::extract::FromRequestParts;

/// conditionally apply layout
///
/// this check for `HX-Request` header, if it exists apply root, else just render the body
pub struct RootRenderer {
    is_hx: bool
}

pub struct Root<T> where T: Template {
    is_hx: bool,
    body: T
}

#[derive(askama::Template)]
#[template(path = "layout.html", escape = "none")]
struct RootTemplate<T> where T: Template {
    body: T
}

impl RootRenderer {
    pub fn render<T: Template>(&self, body: T) -> Root<T> {
        Root { is_hx: self.is_hx, body }
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for RootRenderer {
    type Rejection = std::convert::Infallible;
    async fn from_request_parts(parts: &mut axum::http::request::Parts, _: &S) -> Result<Self,Self::Rejection> {
        Ok(Self {
            is_hx: parts.headers.iter()
                .any(|(name,_)|name == "hx-request")
        })
    }
}

impl<T> IntoResponse for Root<T> where T: Template {
    fn into_response(self) -> Response {
        if self.is_hx {
            Template::render(&self.body)
                .expect("template error")
                .into_response()
        } else {
            RootTemplate { body: self.body }.into_response()
        }
    }
}

