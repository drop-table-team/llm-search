use std::str::FromStr;

use crate::ollama::Ollama;
use anyhow::bail;
use qdrant_client::{
    qdrant::{with_payload_selector::SelectorOptions, SearchPoints, WithPayloadSelector},
    Qdrant,
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct ChatResponse {
    response: String,
    sources: Vec<Source>,
}

#[derive(Serialize, Debug)]
pub struct Source {
    id: usize,
    uuid: Uuid,
    title: String,
    snippet: String,
}

pub struct Llm {
    qdrant: Qdrant,
    ollama: Ollama,
    collection_name: String,
}

impl Llm {
    pub fn new(
        qdrant_url: String,
        ollama_url: String,
        collection_name: String,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            qdrant: Qdrant::from_url(&qdrant_url).build()?,
            ollama: Ollama::from_url(ollama_url)?,
            collection_name,
        })
    }

    pub async fn chat(&self, prompt: &str, cutoff: f32) -> anyhow::Result<ChatResponse> {
        let embeddings = self.ollama.embeddings(prompt).await?;

        let mut context = String::new();
        let mut sources = Vec::new();

        for embedding in embeddings {
            let results = self
                .qdrant
                .search_points(SearchPoints {
                    collection_name: self.collection_name.to_string(),
                    vector: embedding,
                    limit: 3,
                    with_payload: Some(WithPayloadSelector {
                        selector_options: Some(SelectorOptions::Enable(true)),
                    }),
                    score_threshold: Some(cutoff),
                    ..Default::default()
                })
                .await?
                .result;

            for (idx, point) in results.iter().enumerate() {
                let uuid = match point.payload.get("uuid") {
                    Some(v) => Uuid::from_str(&v.as_str().unwrap()).unwrap(),
                    None => {
                        bail!("Payload doesn't contain an uuid");
                    }
                };

                let text = match point.payload.get("text") {
                    Some(v) => v.as_str().unwrap().to_string(),
                    None => {
                        bail!("Payload doesn't contain text");
                    }
                };

                let title = match point.payload.get("title") {
                    Some(v) => v.as_str().unwrap().to_string(),
                    None => {
                        bail!("Payload doesn't contain title");
                    }
                };

                context.push_str(&format!("\nQuelle [{}]: {}\n", idx + 1, text));
                sources.push(Source {
                    id: idx + 1,
                    uuid,
                    title,
                    snippet: text,
                });
            }
        }

        let prompt = format!(
            "Basierend auf folgenden Informationen, beantworte bitte diese Frage und verwende die Quellen aus dem Kontext. Benutze die IEEE-Zitierweise wenn du eine Quelle in der Antwort verwendest. Frage: {}\n\nKontext (Quellen):\n{}",
            prompt, context,
        );

        let response = self.ollama.chat(None, &prompt, None, None).await?;

        return Ok(ChatResponse {
            response: response.response,
            sources,
        });
    }
}
