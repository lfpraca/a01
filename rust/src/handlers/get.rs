use axum::{debug_handler, extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use sqlx::{query_as, types::chrono::NaiveDateTime, FromRow, PgPool};

pub async fn get_handler(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    Ok(Json(
        query_as!(
            Tb01,
            "select id, col_texto, col_dt from tb01 order by col_dt desc limit 10"
        )
        .fetch_all(&pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Erro ao pesquisar no banco de dados",
            )
        })?,
    ))
}

#[derive(FromRow, Serialize)]
struct Tb01 {
    id: i32,
    col_texto: Option<String>,
    col_dt: NaiveDateTime,
}
