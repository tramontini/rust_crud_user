mod classes;

use crate::classes::user::User;
use crate::classes::user::Field;
use std::io;
use bcrypt::hash;


//TODO Criar struct com o New
//TODO Usar API
//TODO Salvar no Banco
fn hash_password(password_string: &str) -> String{
    let password_string = hash(password_string, 10).expect("Failed to hash password");
    return password_string;
}

fn request_user_value(field: Field) -> String {

    let mut input = String::new();
    match field {
        Field::Login => {
            println!("Digite o login desejado: ");
        }
        Field::Password => {
            println!("Digite a senha desejada: ");
        },
        Field::Name => {
            println!("Digite seu nome completo: ");
        },
        Field::Age => {
            println!("Digite a sua idade: ");
        },
        Field::Email =>{
            println!("Digite o email desejado: ");
        }
    }
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim_end().to_owned()
}

fn main() {

    let mut user = User {
        login: request_user_value(Field::Login),
        password: request_user_value(Field::Password),
        name: request_user_value(Field::Name),
        email: request_user_value(Field::Email),
        age: request_user_value(Field::Age).parse::<u32>().unwrap(),
        active: true
    };

    println!("Login: {}", user.login);
    println!("Name: {}", user.name);
    println!("Email: {}", user.email);
    println!("Age: {}", user.age);
    println!("Active: {}", user.active);
    user.password =  hash_password(&user.password);
    println!("Senha Hasheada: {}", user.password);

}
