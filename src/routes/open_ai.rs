use crate::shared::AppState;
use axum::body::Bytes;

use crate::error::Error;
use crate::extract::Json;
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAiPrompt {
    pub content: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAIPromptResponse {
    pub content: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAITextToSpeechPrompt {
    pub model: String,
    pub voice: String,
    pub input: String,
}
impl OpenAITextToSpeechPrompt {
    pub fn with_input(input: String) -> Self {
        Self {
            input,
            voice: "alloy".to_string(),
            model: "tts-1".to_string(),
        }
    }
}

pub async fn open_ai(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(OpenAiPrompt { content }): Json<OpenAiPrompt>,
) -> Result<Json<OpenAIPromptResponse>, Error> {
    let openai_client = Client::new(state.env.openai_api_key().to_string());
    let req = ChatCompletionRequest::new(
        chat_completion::GPT3_5_TURBO.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content,
            name: None,
            function_call: None,
        }],
    );

    let result = openai_client.chat_completion(req)?;
    let prompt = OpenAIPromptResponse {
        content: result
            .choices
            .first()
            .and_then(|c| c.message.content.to_owned()),
    };

    Ok(Json(prompt))
}

pub async fn open_ai_text_to_speech(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(OpenAiPrompt { content }): Json<OpenAiPrompt>,
) -> Result<Bytes, Error> {
    let client = reqwest::Client::new();
    let bytes = client
        .post(state.env.openai_api_text_to_speech_model_endpoint())
        .bearer_auth(state.env.openai_api_key())
        .json(&OpenAITextToSpeechPrompt::with_input(content))
        .send()
        .await?
        .bytes()
        .await?;

    // let mut file = fs::OpenOptions::new()
    //     .create(true) // To create a new file
    //     .write(true)
    //     .open(format!("test-{}.mp3", chrono::Utc::now().time()))
    //     .await?;
    //
    // file.write_all(&bytes).await?;

    Ok(bytes)
}
