use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use api::ask;
use llm::Llm;
use log::{error, info};
use serde::Deserialize;
use std::{env, sync::Arc};

pub mod api;
pub mod llm;
pub mod ollama;

#[derive(Deserialize, Debug)]
struct Config {
    address: String,
    ollama_address: String,
    qdrant_address: String,
    qdrant_collection: String,
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

    let llm = Arc::new(
        Llm::new(
            config.qdrant_address,
            config.ollama_address,
            config.qdrant_collection,
        )
        .unwrap(),
    );

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(ask)
            .app_data(Data::new(llm.clone()))
    })
    .bind(config.address)
    .unwrap()
    .run()
    .await
    .unwrap();
}
