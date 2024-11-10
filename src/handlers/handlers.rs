use crate::{
    models::notes::{CreateNoteSchema, FilterOptions, NotesModel, UpdateNoteSchema},
    AppState,
};
use actix_web::{get, post, web, HttpResponse};
use chrono::prelude::*;
use serde_json::json;

#[get("/notes")]
async fn get_notes(opts: web::Query<FilterOptions>, data: web::Data<AppState>) -> HttpResponse {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        NotesModel,
        "SELECT * FROM notes ORDER BY id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message = "error: something happened while fetching all the records";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": message}));
    }

    let notes = query_result.unwrap();

    let json_response = json!({
        "status": "success",
        "count": notes.len(),
        "notes": notes
    });

    HttpResponse::Ok().json(json_response)
}

#[post("/notes")]
pub async fn create_note(
    body: web::Json<CreateNoteSchema>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let query_result = sqlx::query_as!(
        NotesModel,
        "INSERT INTO notes(title, content, category) VALUES ($1, $2, $3) RETURNING *",
        body.title.to_string(),
        body.content.to_string(),
        body.category.to_owned().unwrap_or("".to_string())
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(note) => {
            let note_response = json!({"status": "success", "data": json!({"notes": note})});
            HttpResponse::Created().json(note_response)
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest()
                    .json(json!({"status": "error", "message": "duplicated title"}));
            }
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": format!("error: {}", e)}));
        }
    }
}
