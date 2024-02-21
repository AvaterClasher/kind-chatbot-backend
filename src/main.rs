#[macro_use]
extern crate rocket;

use reqwest::Client;
use rocket::http::Status;
use rocket::serde::json::{json, serde_json::Value, Json};
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
async fn make_gemini_request(prompt: &str) -> Result<String, reqwest::Error> {
    let endpoint = "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key=";
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

    let body = response.text().await?;
    println!("Response body: {}", body);

    // let json: Value = response.json().await?;
    // if let Some(candidates) = json.get("candidates") {
    //     if let Some(candidate) = candidates[0].get("content") {
    //         if let Some(parts) = candidate.get("parts") {
    //             if let Some(text) = parts[0].get("text") {
    //                 return Ok(text.as_str().unwrap().to_string());
    //             }
    //         }
    //     }
    // }

    Ok("Error processing request".to_string())
}

// fn url() -> String {
//     let api_key = "";
//     let url_endpoint =
//         "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key=";
//     format!("{}", url_endpoint)
// }

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
