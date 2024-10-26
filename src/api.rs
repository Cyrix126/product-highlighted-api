use axum::{extract::State, response::IntoResponse, Json};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use reqwest::StatusCode;

use crate::{
    db::ProductHighlighted,
    error::AppError,
    schema::products_highlighted::{self, enabled, product_id},
    AppState,
};

/// modify list of highlighted products
pub async fn highlight(
    State(state): State<AppState>,
    Json(data): Json<Vec<(u32, u8)>>,
) -> Result<impl IntoResponse, AppError> {
    // check if products exist
    for (id, _) in data.iter() {
        if state.client_product.get_product_from_id(*id).await.is_err() {
            return Ok(StatusCode::NOT_FOUND.into_response());
        }
    }
    // prepare data
    let mut rows = vec![];
    for p in data {
        rows.push(ProductHighlighted {
            id: 0, // incremented by the table
            product_id: p.0 as i32,
            priority: p.1 as i16,
            enabled: true,
        })
    }
    // update database replacing existing.
    let conn = state.pool.get().await?;
    conn.interact(|conn| {
        // empty table
        diesel::delete(products_highlighted::table).execute(conn)?;
        // add data
        diesel::insert_into(products_highlighted::table)
            .values(rows)
            .execute(conn)?;
        Ok::<(), diesel::result::Error>(())
    })
    .await??;

    Ok(().into_response())
}
/// modify list of highlighted products
pub async fn highlight_flip(
    State(state): State<AppState>,
    Json(data): Json<(u32, bool)>,
) -> Result<impl IntoResponse, AppError> {
    // update database replacing existing.
    let conn = state.pool.get().await?;
    conn.interact(move |conn| {
        //
        diesel::update(products_highlighted::table)
            .set(enabled.eq(data.1))
            .filter(product_id.eq(data.0 as i32))
            .execute(conn)?;
        Ok::<(), diesel::result::Error>(())
    })
    .await??;
    Ok(().into_response())
}
pub async fn highlighted_enabled(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    highlighted_bool(State(state), true).await
}
pub async fn highlighted_disabled(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    highlighted_bool(State(state), false).await
}
async fn highlighted_bool(
    State(state): State<AppState>,
    show_enabled: bool,
) -> Result<impl IntoResponse, AppError> {
    // update database replacing existing.
    let conn = state.pool.get().await?;
    let result = conn
        .interact(move |conn| {
            //
            use crate::schema::products_highlighted::dsl::*;
            let products = products_highlighted
                .filter(enabled.eq(show_enabled))
                .select((product_id, priority))
                .load(conn)?;
            Ok::<Vec<(i32, i16)>, diesel::result::Error>(products)
        })
        .await??;

    Ok(Json(result))
}
