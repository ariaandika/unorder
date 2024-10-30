use std::future::{ready, Ready};

use askama::Template;
use axum::Form;
use serde::Deserialize;

use crate::layout::{Root, RootRenderer};

#[derive(Template)]
#[template(path = "auth.html")]
pub enum Views {
    Login,
    Err(String),
}

impl Views {
    pub fn view(render: RootRenderer) -> Ready<Root<Self>> {
        ready(render.render(Self::Login))
    }

    pub fn login(
        render: RootRenderer,
        Form(form): Form<Login>
    ) -> Ready<Root<Self>> {
        std::thread::sleep(std::time::Duration::from_secs(2));
        println!("Result: {form:?}");
        ready(render.render(Self::Err(form.phone)))
    }
}

#[derive(Debug, Deserialize)]
pub struct Login {
    phone: String,
    password: String,
}

