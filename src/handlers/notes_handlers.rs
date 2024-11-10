use crate::{
    models::notes::{CreateNoteSchema, FilterOptions, NotesModel, UpdateNoteSchema},
    AppState,
};
use actix_web::{delete, get, post, put, web, HttpResponse};
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
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": format!("error: {}", e)}))
        }
    }
}

#[get("/notes/{id}")]
pub async fn get_note_by_id(
    path: web::Query<uuid::Uuid>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let note_id = path.into_inner();
    let query_result = sqlx::query_as!(NotesModel, "SELECT * FROM notes WHERE id = $1", note_id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(note) => {
            let note_response = json!({"status": "success", "data": json!({"note": note})});
            HttpResponse::Ok().json(note_response)
        }
        Err(_) => {
            let note_response = format!("error: not found note with id: {}", note_id);
            HttpResponse::NotFound().json(json!({"status": "fail", "message": note_response}))
        }
    }
}

#[put("/note/{id}")]
pub async fn update_note(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateNoteSchema>,
    data: web::Data<AppState>,
) -> HttpResponse {
    // First we check if the id exist
    let note_id = path.into_inner();
    let query_result = sqlx::query_as!(NotesModel, "SELECT * FROM notes WHERE id = $1", note_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let note_response = format!("error: not found note with id: {}", note_id);
        return HttpResponse::NotFound().json(json!({"status": "fail", "message": note_response}));
    }

    let now = chrono::Utc::now();
    let note = query_result.unwrap();

    let query_result = sqlx::query_as!(
        NotesModel,
        "UPDATE notes SET title = $1, content = $2, category = $3, published = $4, updated_at = $5 WHERE id = $6 RETURNING *",
        body.title.to_owned().unwrap_or(note.title),
        body.content.to_owned().unwrap_or(note.content),
        body.category.to_owned().unwrap_or(note.category.unwrap()),
        body.published.to_owned().unwrap_or(note.published.unwrap()),
        now,
        note_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(note) => {
            let note_response = json!({"status": "success", "data": json!({"note": note})});
            HttpResponse::Created().json(note_response)
        }
        Err(e) => {
            let message = format!("error: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": message}))
        }
    }
}

#[delete("/notes/{id}")]
pub async fn delete_note(path: web::Path<uuid::Uuid>, data: web::Data<AppState>,) -> HttpResponse {
    let note_id = path.into_inner();
    let rows_affected = sqlx::query_as!(NotesModel, "DELETE FROM notes WHERE id = $1", note_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let message = format!("error: not found note with id {}", note_id);
        return HttpResponse::NotFound().json(json!({"status": "fail", "message": message}));
    }

    HttpResponse::NoContent().finish()
}
