use anyhow::bail;
use qdrant_client::qdrant::SearchPoints;
use serde::Serialize;
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

use crate::llm::Llm;

#[derive(Serialize)]
pub struct ChatResponse {
    response: String,
    sources: Vec<(usize, Uuid)>,
}

pub struct Chat {
    uuid: Uuid,
    llm: Arc<Llm>,
}

impl Chat {
    pub fn new(llm: Arc<Llm>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            llm,
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub async fn chat(&self, prompt: &str) -> anyhow::Result<ChatResponse> {
        let embeddings = self.llm.ollama().embeddings(prompt).await?;

        let mut context = String::new();
        let mut sources = Vec::new();

        for embedding in embeddings {
            let results = self
                .llm
                .qdrant()
                .search_points(SearchPoints {
                    collection_name: self.llm.collection_name().to_string(),
                    vector: embedding,
                    limit: 3,
                    ..Default::default()
                })
                .await?
                .result;

            for (idx, point) in results.iter().enumerate() {
                let uuid = match point.payload.get("uuid") {
                    Some(v) => Uuid::from_str(&v.as_str().unwrap()).unwrap(),
                    None => {
                        bail!("Payload doesn't contain text");
                    }
                };

                let text = match point.payload.get("text") {
                    Some(v) => v.as_str().unwrap(),
                    None => {
                        bail!("Payload doesn't contain text");
                    }
                };

                context.push_str(&format!("\nQuelle [{}]: {}\n", idx + 1, text));
                sources.push((idx + 1, uuid));
            }
        }

        let prompt = format!(
            "Basierend auf folgenden Informationen, beantworte bitte diese Frage und verwende die Quellen aus dem Kontext. Benutze die IEEE-Zitierweise wenn du eine Quelle in der Antwort verwendest. Frage: {}\n\nKontext (Quellen):\n{}",
            prompt, context,
        );

        let response = self.llm.ollama().chat(None, &prompt, None, None).await?;

        return Ok(ChatResponse {
            response: response.response,
            sources,
        });
    }
}
