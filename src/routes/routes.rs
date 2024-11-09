use actix_web::{get, web, HttpResponse};
use chrono::Utc;
use serde_json::json;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/").service(health);
    conf.service(scope);
}

#[get("/health")]
async fn health() -> HttpResponse {
    let now = Utc::now();
    HttpResponse::Ok().json(json!({"status": "Success", "message": "up and running", "time": now}))
}
