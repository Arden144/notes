use std::{error::Error, fs::File, io::BufReader};

use actix_web::{http::header, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
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
        .get(header::AUTHORIZATION)
        .filter(|&value| value == AUTH)
        .is_some()
}

#[post("/notes")]
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
        Err(_) => HttpResponse::InternalServerError(),
    }
}

fn get_tls_config() -> Result<ServerConfig, Box<dyn Error>> {
    let mut cert_file = BufReader::new(File::open(CERT)?);
    let cert_chain: Vec<Certificate> = certs(&mut cert_file)?
        .into_iter()
        .map(Certificate)
        .collect();
    if cert_chain.len() == 0 {
        panic!("Could not get certificate.");
    }
    let mut key_file = BufReader::new(File::open(KEY)?);
    let key_der = pkcs8_private_keys(&mut key_file)?
        .into_iter()
        .map(PrivateKey)
        .next()
        .expect("Could not get private key.");
    let tls_config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key_der)?;
    Ok(tls_config)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut bot = Client::builder(TOKEN).event_handler(Handler).await?;
    let bot_manager = bot.shard_manager.clone();
    let bot_http = web::Data::from(bot.cache_and_http.http.clone());

    let svr = HttpServer::new(move || App::new().app_data(bot_http.clone()).service(add_note))
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
