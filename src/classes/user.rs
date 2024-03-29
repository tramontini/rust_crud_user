//Coloca para poder fazer o match
use serde::ser::{Serialize, Serializer, SerializeStruct};
// PartialEq, Eq, Clone,  Debug, Copy

#[derive(Serialize, Deserialize)]
pub struct User{
    pub login: String,
    pub password: String,
    pub name: String,
    pub email: String,
    pub age: u32,
    pub active: bool
}
