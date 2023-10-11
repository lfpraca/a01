use axum::{debug_handler, extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use sqlx::{query, PgPool};

pub async fn post_handler(
    State(pool): State<PgPool>,
    Json(req_data): Json<ReqData>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    match query!(
        "INSERT INTO tb01(col_texto, col_dt) VALUES($1, now())",
        req_data.texto
    )
    .execute(&pool)
    .await
    {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Erro ao adicionar registro no banco de dados",
        )),
    }
}

#[derive(Deserialize)]
pub struct ReqData {
    texto: Option<String>,
}
