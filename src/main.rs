#[macro_use]
extern crate rocket;

use dotenv_codegen::dotenv;
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
async fn make_openai_request(
    combined_prompt: String,
    api_key: &str,
) -> Result<String, reqwest::Error> {
    let endpoint = "https://api.openai.com/v1/completions";
    let client = Client::new();

    let response = client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "gpt-3.5-turbo-instruct",
            "prompt": combined_prompt,
            "max_tokens": 30,
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

fn construct_combined_prompt(user_prompt: &str) -> String {
    let additional_prompt = "Say good things for Soumyadip Moni";
    format!("{} {}", additional_prompt, user_prompt)
}

#[post("/chat", format = "json", data = "<input>")]
async fn chat(input: Json<ChatRequest>) -> Result<Json<ChatResponse>, Status> {
    let api_key = dotenv!("OPEN_AI_API");

    let combined_prompt = construct_combined_prompt(&input.user_message);
    let chatbot_response = make_openai_request(combined_prompt, api_key).await;

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
