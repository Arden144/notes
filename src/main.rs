mod auth;
mod config;
mod tls;
mod types;
mod v1;

use tls::*;

use std::{error::Error, sync::Arc};

use actix_web::{web, App, HttpServer};
use serenity::{async_trait, model::prelude::*, prelude::*};
use tokio::signal;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!(
            "[Notes] Logged in as {}#{}",
            ready.user.name, ready.user.discriminator
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut bot = Client::builder(config::TOKEN)
        .event_handler(Handler)
        .await?;
    let bot_manager = Arc::clone(&bot.shard_manager);
    let bot_http = Arc::clone(&bot.cache_and_http.http);

    let svr = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(Arc::clone(&bot_http)))
            .service(v1::get_service())
    })
    .disable_signals()
    .bind_rustls(config::ADDR, get_tls_config()?)?
    .run();
    let svr_handle = svr.handle();

    println!("[Notes] Starting");
    tokio::spawn(async move { bot.start().await });
    tokio::spawn(svr);

    signal::ctrl_c().await?;
    println!("[Notes] Stopping");
    svr_handle.stop(true).await;
    bot_manager.lock().await.shutdown_all().await;
    println!("[Notes] Done");
    Ok(())
}
