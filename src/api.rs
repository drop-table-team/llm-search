use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use log::error;
use serde::Deserialize;
use std::sync::Arc;

use crate::llm::Llm;

#[derive(Deserialize)]
struct AskRequest {
    prompt: String,
}

#[post("/ask")]
async fn ask(llm: Data<Arc<Llm>>, prompt: Json<AskRequest>) -> HttpResponse {
    let response = match llm.chat(&prompt.prompt, 0.7).await {
        Ok(r) => r,
        Err(e) => {
            error!("Error while prompting with chat: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().json(response)
}
