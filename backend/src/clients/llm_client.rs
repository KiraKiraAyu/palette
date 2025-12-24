use async_trait::async_trait;
use eventsource_stream::Eventsource;
use futures::stream::{BoxStream, StreamExt};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{
    error::{AppError, Result},
    models::user_provider::Model as ProviderModel,
};

#[derive(Debug, Clone, Serialize)]
pub struct ChatMessagePayload {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatRequestPayload {
    pub model: String,
    pub messages: Vec<ChatMessagePayload>,
    pub stream: bool,
}

#[derive(Debug, Deserialize)]
struct ChoiceDelta {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    index: usize,
    delta: ChoiceDelta,
}

#[derive(Debug, Deserialize)]
struct ChatResponsePayload {
    choices: Vec<Choice>,
}

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn chat(
        &self,
        provider: &ProviderModel,
        model_id: &str,
        messages: Vec<ChatMessagePayload>,
    ) -> Result<BoxStream<'static, Result<String>>>;
}

#[derive(Clone)]
pub struct DefaultLlmClient {
    http: reqwest::Client,
}

impl Default for DefaultLlmClient {
    fn default() -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self { http }
    }
}

#[async_trait]
impl LlmClient for DefaultLlmClient {
    async fn chat(
        &self,
        provider: &ProviderModel,
        model_id: &str,
        messages: Vec<ChatMessagePayload>,
    ) -> Result<BoxStream<'static, Result<String>>> {
        let base = provider.url.trim_end_matches('/');
        
        let url = if base.ends_with("/v1") {
            format!("{}/chat/completions", base)
        } else {
            format!("{}/v1/chat/completions", base)
        };

        let payload = ChatRequestPayload {
            model: model_id.to_string(),
            messages,
            stream: true,
        };

        let mut req = self.http.post(url).json(&payload);
        if let Some(k) = provider.key.clone() {
            req = req.bearer_auth(k);
        }

        let resp = req
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to call model API: {}", e)))?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(AppError::BadRequest(format!(
                "API error: {} {}",
                status, body
            )));
        }

        let stream = resp
            .bytes_stream()
            .eventsource()
            .map(|event| {
                match event {
                    Ok(event) => {
                        if event.data == "[DONE]" {
                            None
                        } else {
                            match serde_json::from_str::<ChatResponsePayload>(&event.data) {
                                Ok(parsed) => {
                                    if let Some(choice) = parsed.choices.first() {
                                        if let Some(content) = choice.delta.content.clone() {
                                            if !content.is_empty() {
                                                return Some(Ok(content));
                                            }
                                        }
                                    }
                                    // Empty content or no choices, skip
                                    Some(Ok("".to_string()))
                                }
                                Err(e) => Some(Err(AppError::Internal(format!(
                                    "Failed to parse SSE data: {} | Data: {}",
                                    e, event.data
                                )))),
                            }
                        }
                    }
                    Err(e) => Some(Err(AppError::Internal(format!("Stream error: {}", e)))),
                }
            })
            // Filter out None (DONE) and empty strings to clean up the stream
            .take_while(|x| futures::future::ready(x.is_some()))
            .map(|x| x.unwrap())
            .filter(|x| {
                futures::future::ready(match x {
                    Ok(s) => !s.is_empty(),
                    Err(_) => true,
                })
            });

        Ok(Box::pin(stream))
    }
}
