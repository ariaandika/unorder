use crate::views::form::Form;
use std::future::{ready, Ready};

use askama::Template;

#[derive(Template)]
#[template(path = "orders.html")]
pub enum Views {
    Show(Vec<Orders>),
    New,
}

impl Views {
    pub fn show() -> Ready<Self> {
        ready(Self::Show(vec![]))
    }

    pub fn new() -> Ready<Self> {
        ready(Self::New)
    }
}

pub struct Orders {
    id: String,
    name: String,
}

