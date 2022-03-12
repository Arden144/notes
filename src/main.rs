use std::{error::Error, fs::File, io::BufReader, sync::Arc};

use actix_web::{http, put, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use serde::Deserialize;
use serenity::{async_trait, http::Http, model::prelude::*, prelude::*};
use tokio::signal;

const TOKEN: &str = "***REMOVED***";
const CERT: &str = "***REMOVED***.pem";
const KEY: &str = "***REMOVED***.key";
const ADDR: &str = "0.0.0.0:443";
const AUTH: &str = "Basic ***REMOVED***";
const CHANNEL: serenity::model::id::ChannelId = ChannelId(***REMOVED***u64);

#[derive(Deserialize)]
struct Note {
    message: String,
}

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

fn valid_auth_token(req: HttpRequest) -> bool {
    req.headers()
        .get(http::header::AUTHORIZATION)
        .filter(|&value| value == AUTH)
        .is_some()
}

#[put("/note")]
async fn add_note(
    note: web::Json<Note>,
    bot_http: web::Data<Http>,
    req: HttpRequest,
) -> impl Responder {
    if !valid_auth_token(req) {
        return HttpResponse::Unauthorized();
    }
    match CHANNEL.say(bot_http.get_ref(), &note.message).await {
        Ok(_) => HttpResponse::Ok(),
        Err(err) => {
            eprintln!("Failed to send message: {:?}", err);
            HttpResponse::InternalServerError()
        }
    }
}

fn get_tls_config() -> Result<ServerConfig, Box<dyn Error>> {
    let mut cert_file = BufReader::new(File::open(CERT)?);
    let mut key_file = BufReader::new(File::open(KEY)?);

    let cert_chain: Vec<Certificate> = certs(&mut cert_file)?
        .into_iter()
        .map(Certificate)
        .collect();
    assert!(!cert_chain.is_empty(), "Could not get certificate.");

    let key_der = pkcs8_private_keys(&mut key_file)?
        .into_iter()
        .map(PrivateKey)
        .next()
        .expect("Could not get private key.");

    Ok(ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key_der)?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut bot = Client::builder(TOKEN).event_handler(Handler).await?;
    let bot_manager = Arc::clone(&bot.shard_manager);
    let bot_http = Arc::clone(&bot.cache_and_http.http);

    let svr = HttpServer::new(move || {
        let v1 = web::scope("/api/v1").service(add_note);

        App::new()
            .app_data(web::Data::from(Arc::clone(&bot_http)))
            .service(v1)
    })
    .disable_signals()
    .bind_rustls(ADDR, get_tls_config()?)?
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
