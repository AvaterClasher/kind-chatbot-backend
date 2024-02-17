#[macro_use]
extern crate rocket;

use dotenv_codegen::dotenv;
use reqwest::blocking::Client;
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

// Added a post function for the api request
#[post("/chat", format = "json", data = "<input>")]
fn chat(input: Json<ChatRequest>) -> Result<Json<ChatResponse>, Status> {
    let api_key = dotenv!("OPEN_AI_API");
    let endpoint = "https://api.openai.com/v1/engines/davinci-codex/completions";
    let client = Client::new();
    let prompt = "How are you ?";

    let response = client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "prompt": prompt,
            "max_tokens": 150,
        }))
        .send();

    match response {
        Ok(res) => {
            if let Ok(json) = res.json::<serde_json::Value>() {
                if let Some(completions) = json.get("choices") {
                    if let Some(completion) = completions[0].get("text") {
                        let chatbot_response = completion.as_str().unwrap().to_string();
                        return Ok(Json(ChatResponse { chatbot_response }));
                    }
                }
            }
        }
        Err(_) => return Err(Status::InternalServerError),
    }
    Err(Status::InternalServerError)
}

// Rocket Main function
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![chat])
}
