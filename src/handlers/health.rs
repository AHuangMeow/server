use actix_web::{get, HttpResponse};

#[get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "timestamp": time::OffsetDateTime::now_utc().unix_timestamp()
    }))
}
