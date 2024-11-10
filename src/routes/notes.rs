use actix_web::web;
use crate::handlers::handlers::{get_notes, create_note};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
    .service(get_notes)
    .service(create_note);

    conf.service(scope);
}