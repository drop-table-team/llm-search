use std::sync::Arc;

use crate::{chat::Chat, ollama::Ollama};
use qdrant_client::Qdrant;

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

    pub fn ollama(&self) -> &Ollama {
        &self.ollama
    }

    pub fn qdrant(&self) -> &Qdrant {
        &self.qdrant
    }

    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }

    pub fn new_chat(self: &Arc<Self>) -> Chat {
        Chat::new(self.clone())
    }
}
