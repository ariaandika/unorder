use askama::Template;


#[derive(Template)]
#[template(path = "components/form.html")]
pub enum Form {
    Field {
        name: &'static str,
    }
}

impl Form {
    pub fn field(name: &'static str) -> Self {
        Self::Field { name }
    }
}

