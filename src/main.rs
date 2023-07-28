mod classes;

use crate::classes::user::User;
// use std::io;
use bcrypt::hash;
use axum::{
    routing::{get, post},
    Router,
    response::Json,
};

use axum_macros::{
    debug_handler
};

//TODO Criar struct com o New
//TODO Usar API
//TODO Salvar no Banco
fn hash_password(password_string: &str) -> String{
    let password_string = hash(password_string, 10).expect("Failed to hash password");
    return password_string;
}


// async fn save_user(Json(save_user)::Json<User>) {
//     let user = Experiment::from_request(create_request);
//
// }

#[axum_macros::debug_handler]
async fn get_user() -> Json<User> {

    let mut user = User {
        login: String::from("tramontini"),
        password: hash_password(&String::from("teste")),
        name: String::from("Matheus"),
        email: String::from("Matheus Tramontini"),
        age: 27,
        active: true
    };

    return Json(user);
}

#[tokio::main]
async fn main() {

    print!("criando rota");
    //TODO Usar API

    let app = Router::new()
        .route("/teste", get(|| async { "Hello, World!" }))
        .route("/get_user", get(get_user));
        //.route("/save_user", get(get_user));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}
