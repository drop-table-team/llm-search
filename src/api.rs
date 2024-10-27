use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use ahash::AHashMap;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{chat::Chat, llm::Llm};

#[derive(Serialize)]
struct NewChatResponse {
    uuid: Uuid,
}

#[get("/new_chat")]
async fn new_chat(
    llm: Data<Arc<Llm>>,
    chats: Data<&'static RwLock<AHashMap<Uuid, Chat>>>,
) -> impl Responder {
    let chat = llm.new_chat();

    let uuid = chat.uuid();

    let mut lock = chats.write().await;

    lock.insert(uuid, chat);

    info!("Created new chat '{}'", uuid);

    HttpResponse::Ok().json(NewChatResponse { uuid })
}

#[derive(Debug, Deserialize)]
struct AskRequest {
    prompt: String,
}

#[post("/{uuid}/ask")]
async fn ask(
    uuid: Path<Uuid>,
    prompt: Json<AskRequest>,
    chats: Data<&'static RwLock<AHashMap<Uuid, Chat>>>,
) -> HttpResponse {
    let uuid = uuid.into_inner();

    let lock = chats.read().await;

    let chat = match lock.get(&uuid) {
        Some(c) => c,
        None => {
            error!("Chat with uuid '{}' doesn't exist", uuid);
            return HttpResponse::BadRequest().finish();
        }
    };

    let response = match chat.chat(&prompt.prompt).await {
        Ok(r) => r,
        Err(e) => {
            error!("Error while prompting with chat '{}': {}", uuid, e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().json(response)
}
