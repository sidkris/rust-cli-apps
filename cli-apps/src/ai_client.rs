use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Serialize, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize)]
struct Choice {
    message: ChatMessage,
}

pub async fn ask_gpt(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY")?;
    let client = Client::new();

    let req_body = ChatRequest {
        model: "gpt-4".to_string(),
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
    };

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&req_body)
        .send()
        .await?;

    let status = res.status();
    let text = res.text().await?;

    if !status.is_success() {
        // eprintln!("OpenAI API Error (status {status}): {text}");
        return Err(format!("OpenAI API error: {status}").into());
    }

    // Debug print the raw response
    // println!("Raw response: {text}");

    let body: ChatResponse = serde_json::from_str(&text)?;
    Ok(body.choices.first().unwrap().message.content.clone())
}
