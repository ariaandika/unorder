use crate::{layout::{Root, RootRenderer}, views::form::Form};
use std::future::{ready, Ready};

use askama::Template;

#[derive(Template)]
#[template(path = "orders.html")]
pub enum Views {
    Show(Vec<Orders>),
    New,
}

impl Views {
    pub fn show(render: RootRenderer) -> Ready<Root<Self>> {
        ready(render.render(Self::Show(vec![])))
    }

    pub fn new(render: RootRenderer) -> Ready<Root<Self>> {
        ready(render.render(Self::New))
    }
}

pub struct Orders {
    id: String,
    name: String,
}

