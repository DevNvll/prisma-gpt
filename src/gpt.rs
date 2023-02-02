use reqwest::{ClientBuilder, Response};
use serde::Deserialize;
use serde_json::json;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct GPTResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    text: String,
}

impl fmt::Display for GPTResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut choices_str = String::new();
        for choice in &self.choices {
            choices_str.push_str(&format!("{}\n", choice.text));
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

        let url = "https://api.openai.com/v1/completions";
        let body = json!({
            "prompt": text,
            "temperature": 0.7,
            "max_tokens": 2000,
            "model": "text-davinci-003",
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
