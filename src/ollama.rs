use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokenizers::Tokenizer;

#[derive(Deserialize)]
struct EmbeddingResponse {
    embedding: Vec<f32>,
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    system: Option<&'a str>,
    template: Option<&'a str>,
    context: Option<&'a [i64]>,
    stream: bool,
}

#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    pub response: String,
    pub context: Vec<i64>,
}

pub struct Ollama {
    client: Client,
    address: String,
    tokenizer: Tokenizer,
}

impl Ollama {
    pub fn from_url(address: String) -> anyhow::Result<Self> {
        let tokenizer =
            Tokenizer::from_pretrained("mixedbread-ai/mxbai-embed-large-v1", None).unwrap();

        Ok(Self {
            client: Client::new(),
            address,
            tokenizer,
        })
    }

    pub async fn chat(
        &self,
        system: Option<&str>,
        prompt: &str,
        template: Option<&str>,
        context: Option<&[i64]>,
    ) -> anyhow::Result<ChatResponse> {
        let url = format!("{}/api/generate", self.address);

        let request = ChatRequest {
            model: "llama3.2",
            prompt: prompt,
            system: system,
            template: template,
            context: context,
            stream: false,
        };

        let response = self
            .client
            .post(url)
            .body(serde_json::to_vec(&request).unwrap())
            .send()
            .await?
            .json::<ChatResponse>()
            .await?;

        Ok(response)
    }

    pub async fn embeddings(&self, text: &str) -> anyhow::Result<Vec<Vec<f32>>> {
        let mut embeddings = Vec::new();

        let chunks = self.chunks(text, 512, 56)?;

        for chunk in chunks {
            let embedding = self.generate_embedding(&chunk).await?;

            embeddings.push(embedding);
        }

        Ok(embeddings)
    }

    fn chunks(
        &self,
        text: &str,
        chunk_size: usize,
        chunk_overlap: usize,
    ) -> anyhow::Result<Vec<String>> {
        let mut chunks = Vec::new();

        let tokens = self.tokenizer.encode(text, true).unwrap();

        if tokens.get_ids().len() <= chunk_size {
            chunks.push(text.to_string());
            return Ok(chunks);
        }

        let tokens = tokens.get_tokens();

        let mut start = 0usize;
        while start < tokens.len() {
            let end = (start + chunk_size).min(tokens.len());

            chunks.push(tokens[start..end].concat());

            start += chunk_size - chunk_overlap
        }

        dbg!("HERE");

        Ok(chunks)
    }

    async fn generate_embedding(&self, text: &str) -> anyhow::Result<Vec<f32>> {
        let url = format!("{}/api/embeddings", self.address);

        let model = "mxbai-embed-large";

        let mut embedding = self
            .client
            .post(url)
            .body(format!(
                "{{\"model\":\"{}\",\"prompt\":\"{}\"}}",
                model, text
            ))
            .send()
            .await?
            .json::<EmbeddingResponse>()
            .await?
            .embedding;

        embedding.shrink_to_fit();

        Ok(embedding)
    }
}
