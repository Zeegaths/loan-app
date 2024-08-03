use askama::Template;

#[derive(Template)]
#[template(path = "login.html")]

pub struct LoginTemplate{
    pub(crate) error: Option<String>,
    pub (crate) message: Option<String>
}

#[derive(Template)]

#[template(path = "resgister.html")]

pub struct RegisterTemplate{
    pub(crate) error: Option<String>
}