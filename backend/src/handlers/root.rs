use axum::Json;

pub async fn home() -> Json<&'static str> {
    Json("Hello! World")
}
