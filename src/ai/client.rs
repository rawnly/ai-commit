use reqwest::{
    Client,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue},
};

use super::models::{ChatRequest, ChatResponse, Message};

#[derive(Debug, Clone)]
pub struct GroqClient {
    client: Client,
    api_key: String,
    base_url: Option<String>,
}

impl GroqClient {
    pub fn new(api_key: &str) -> Self {
        GroqClient {
            client: Client::new(),
            api_key: api_key.to_string(),
            base_url: None,
        }
    }

    fn get_base_url(self) -> String {
        self.base_url.unwrap_or("https://api.groq.com".to_string())
    }

    fn default_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        let authorization = format!("Bearer {}", self.api_key);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(authorization.as_str()).unwrap(),
        );

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        headers
    }
}

impl GroqClient {
    pub async fn create_chat_completion(
        self,
        model: String,
        messages: Vec<Message>,
    ) -> anyhow::Result<ChatResponse> {
        let base_url = self.clone().get_base_url();
        let client = self.clone().client;

        let payload = ChatRequest { model, messages };
        let url = format!("{}/openai/v1/chat/completions", base_url);

        let response = client
            .post(&url)
            .json(&payload)
            .headers(self.clone().default_headers())
            .send()
            .await?;

        response.json::<ChatResponse>().await.map_err(|e| e.into())
    }
}
