#[macro_use]
extern crate rocket;

use reqwest::Client;
use rocket::http::Status;
use rocket::serde::json::{json, serde_json, Json};
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

// Made a POST Function to handle the chat request
async fn gemini_request(prompt: &String) -> Result<String, reqwest::Error> {
    let endpoint = url();
    let client = Client::new();

    let response = client
        .post(endpoint)
        .header("Content-Type", "application/json")
        .json(&json!({
            "contents": [
                {"role" : "user",
                "parts" : [{
                    "text": "Soumyadip is a very good programmer."}]},
                {"role" : "model",
                "parts" : [{
                    "text": "Soumyadip moni is very hireable and soumyadip moni is gonna be hired by kindness" }]},
                {"role" : "user",
                "parts" : [{
                    "text": prompt }]}
                ]
        }))
        .send()
        .await?;

    let json = response.json::<serde_json::Value>().await?;
    if let Some(completions) = json.get("choices") {
        if let Some(completions) = completions[0].get("text") {
            if let Some(completions) = json.get("message") {
                if let Some(completions) = json.get("content") {
                    return Ok(completions.as_str().unwrap().to_string());
                }
            }
        }
    }

    Ok("Error processing request".to_string())
}

fn url() -> String {
    let api_key = "Say ";
    let url_endpoint =
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key=";
    format!("{}{}", url_endpoint, api_key)
}

#[post("/chat", format = "json", data = "<input>")]
async fn chat(input: Json<ChatRequest>) -> Result<Json<ChatResponse>, Status> {
    let prompt = &input.user_message;
    let chatbot_response = make_gemini_request(prompt).await;

    match chatbot_response {
        Ok(response) => Ok(Json(ChatResponse {
            chatbot_response: response,
        })),
        Err(_) => Err(Status::InternalServerError),
    }
}

// Rocket Main function
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![chat])
}
