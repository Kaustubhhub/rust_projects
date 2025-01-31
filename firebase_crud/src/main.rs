use firebase_rs::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Response {
    name: String,
}

#[tokio::main]
async fn main() {
    let user: User = User {
        name: String::from("Kaustubh Kumbhare"),
        age: 22,
        email: String::from("kaustubhkumbhare02@gmail.com"),
    };

    println!("\n\n{:?}", user);

    let firebase = Firebase::new("https://fir-rustcrud-default-rtdb.firebaseio.com/")
        .expect("unable to connect to firebase database");

    let response: Response = set_user(&firebase_client, &user).await?;
}

async fn set_user(firebase_client: &Firebase, user: &User) -> Response {
    let firebase = firebase_client.at("users");
    let _users = firebase.set::<User>(&user).await;
    return;
}
async fn get_user(firebase_client: &Firebase, user: &User) -> Response {}
async fn update_user(firebase_client: &Firebase, user: &User) -> Response {}
async fn delete_user(firebase_client: &Firebase, user: &User) -> Response {}

fn string_to_reponse(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}

// Convert a string to a User
fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap()
}
