use reqwest::{ClientBuilder, Response};
use serde::Deserialize;
use serde_json::json;
use std::fmt;

const MODEL_NAME: &str = "gpt-3.5-turbo-16k";  // Define the model name here

#[derive(Deserialize, Debug)]
pub struct GPTResponse {
    id: String,
    object: String,
    created: u64,
    model: String,  // Model is a string
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    message: Message,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    role: String,
    content: String,
}

impl fmt::Display for GPTResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut choices_str = String::new();
        for choice in &self.choices {
            choices_str.push_str(&format!("Role: {}, Content: {}\n", choice.message.role, choice.message.content));
        }
        write!(f, "{}", choices_str)
    }
}

pub struct GPTService {
    client: reqwest::Client,
    openai_api_key: String,
}

impl GPTService {
    pub fn new(openai_key: &str) -> GPTService {
        let client = ClientBuilder::new()
            .user_agent("prisma-gpt")
            .build()
            .unwrap();
        GPTService {
            client,
            openai_api_key: openai_key.to_string(),
        }
    }

    pub async fn get_gpt_completion(
        &self,
        text: &str,
    ) -> Result<GPTResponse, Box<dyn std::error::Error>> {
        let client = &self.client;
        let openai_api_key = &self.openai_api_key;

        let url = "https://api.openai.com/v1/chat/completions"; 
        let body = json!({
            "messages": [{"role": "system", "content": "You are a helpful assistant."}, {"role": "user", "content": text}],
            "max_tokens": 2000,
            "model": MODEL_NAME,  // use the model name constant here
            "top_p": 1,
            "frequency_penalty": 0,
            "presence_penalty": 0,
        });

        let response: Response = client
            .post(url)
            .bearer_auth(openai_api_key)
            .json(&body)
            .send()
            .await
            .unwrap();

        match response.status() {
            reqwest::StatusCode::OK => match response.json::<GPTResponse>().await {
                Ok(gpt_response) => Ok(gpt_response),
                Err(e) => Err(e.into()),
            },
            _ => {
                let error = response.text().await.unwrap();
                Err(error.into())
            }
        }
    }
}
