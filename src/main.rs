use actix_web::{web::Data, App, HttpServer};
use ahash::AHashMap;
use api::{ask, new_chat};
use chat::Chat;
use llm::Llm;
use log::{error, info};
use module::Module;
use serde::Deserialize;
use std::{env, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

pub mod api;
pub mod chat;
pub mod llm;
pub mod module;
pub mod ollama;

#[derive(Deserialize, Debug)]
struct Config {
    address: String,
    module_name: String,
    backend_address: String,
    ollama_address: String,
}

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }

    env_logger::init();

    let config = match envy::from_env::<Config>() {
        Ok(c) => c,
        Err(e) => {
            error!("Couldn't parse environment variables: {}", e);
            return;
        }
    };

    info!("Loaded config: {:?}", config);

    let module = Module::new(config.module_name.clone());

    let response = match module.register(&config.backend_address).await {
        Ok(r) => r,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    info!(
        "Successfuly registered module '{}' on backend '{}'",
        config.module_name, config.backend_address
    );

    let llm = Arc::new(
        Llm::new(
            response.qdrant_address,
            config.ollama_address,
            "collection".to_string(),
        )
        .unwrap(),
    );

    let chats: &'static RwLock<AHashMap<Uuid, Chat>> =
        Box::leak(Box::new(RwLock::new(AHashMap::new())));

    HttpServer::new(move || {
        App::new()
            .service(new_chat)
            .service(ask)
            .app_data(Data::new(llm.clone()))
            .app_data(Data::new(chats))
    })
    .bind(config.address)
    .unwrap()
    .run()
    .await
    .unwrap();
}
