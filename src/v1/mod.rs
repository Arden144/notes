mod note;

use actix_web::{web, Scope};

pub fn get_service() -> Scope {
    web::scope("/v1").service(note::add_note)
}
