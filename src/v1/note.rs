use actix_web::{put, web, HttpRequest, HttpResponse, Responder};
use serenity::http::Http;

use crate::{config, types::*};

#[put("/note")]
async fn add_note(
    note: web::Json<Note>,
    bot_http: web::Data<Http>,
    req: HttpRequest,
) -> impl Responder {
    match config::CHANNEL.say(bot_http.get_ref(), &note.message).await {
        Ok(_) => {
            println!("Sent a message to the channel");
            HttpResponse::Ok()
        }
        Err(err) => {
            eprintln!("Failed to send message: {:?}", err);
            HttpResponse::InternalServerError()
        }
    }
}
