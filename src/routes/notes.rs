use crate::handlers::notes_handlers::{
    create_note, delete_note, get_note_by_id, get_notes, update_note,
};
use actix_web::web;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(get_notes)
        .service(create_note)
        .service(get_note_by_id)
        .service(update_note)
        .service(delete_note);

    conf.service(scope);
}
