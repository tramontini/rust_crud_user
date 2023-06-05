//Coloca para poder fazer o match
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Field {
    Login,
    Password,
    Name,
    Email,
    Age
}

pub struct User{
    pub login: String,
    pub password: String,
    pub name: String,
    pub email: String,
    pub age: u32,
    pub active: bool
}