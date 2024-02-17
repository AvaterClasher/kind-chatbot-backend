#[macro_use]
extern crate rocket;

use reqwest::blocking::Client;
use rocket::serde::json::{json, Json};
use rocket::serde::{Deserialize, Serialize};

// Json Structure for User Message
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct ChatRequest {
    user_message: String,
}

// Json Structure for Chatbot Response
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ChatResponse {
    chatbot_response: String,
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

// Rocket Main function
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
