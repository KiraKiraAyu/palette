use async_trait::async_trait;
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
}

#[derive(Debug, Deserialize)]
struct ChoiceMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct Choice {
    index: usize,
    message: ChoiceMessage,
}

#[derive(Debug, Deserialize)]
struct ChatResponsePayload {
    choices: Vec<Choice>,
}

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn chat(&self, provider: &ProviderModel, model_id: &str, messages: Vec<ChatMessagePayload>) -> Result<String>;
}

#[derive(Clone)]
pub struct DefaultLlmClient {
    http: reqwest::Client,
}

impl Default for DefaultLlmClient {
    fn default() -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(20))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self { http }
    }
}

#[async_trait]
impl LlmClient for DefaultLlmClient {
    async fn chat(&self, provider: &ProviderModel, model_id: &str, messages: Vec<ChatMessagePayload>) -> Result<String> {
        let base = provider.url.trim_end_matches('/');
        if !base.starts_with("https://") {
            return Err(AppError::BadRequest("Provider URL must be https".to_string()));
        }
        let url = format!("{}/v1/chat/completions", base);

        let payload = ChatRequestPayload { model: model_id.to_string(), messages };
        let mut req = self.http.post(url).json(&payload);
        if let Some(k) = provider.key.clone() { req = req.bearer_auth(k); }

        let resp = req.send().await.map_err(|e| AppError::Internal(format!("调用模型接口失败: {}", e)))?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(AppError::BadRequest(format!("API error: {} {}", status, body)));
        }
        let parsed: ChatResponsePayload = resp.json().await
            .map_err(|e| AppError::Internal(format!("解析模型响应失败: {}", e)))?;
        let content = parsed.choices.get(0)
            .map(|c| c.message.content.clone())
            .ok_or_else(|| AppError::Internal("模型未返回内容".to_string()))?;
        Ok(content)
    }
}