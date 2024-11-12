use crate::qr_code::persictence::QrCodeInMemoryDb;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use maud::html;
use maud::Markup;

pub async fn game_view(State(db): State<QrCodeInMemoryDb>) -> Result<Html<String>, StatusCode> {
    let result = db.get_game_state().await;
    let v = game_view_html(result);
    Ok(Html(v.into_string()))
}

pub async fn current(State(db): State<QrCodeInMemoryDb>) -> Result<Html<String>, StatusCode> {
    let result = db.get_game_state().await;

    Ok(Html(result_view(result).into_string()))
}

pub async fn inc(State(db): State<QrCodeInMemoryDb>) -> Result<Html<String>, StatusCode> {
    db.increment().await;
    let result = db.get_game_state().await;

    Ok(Html(result_view(result).into_string()))
}

pub async fn dec(State(db): State<QrCodeInMemoryDb>) -> Result<Html<String>, StatusCode> {
    db.decrement().await;
    let result = db.get_game_state().await;

    Ok(Html(result_view(result).into_string()))
}

fn result_view(result: i64) -> Markup {
    html! {
        div #game_result hx-get="/game/current" hx-trigger="every 1s" {
            p { "Result: " (result) }
        }
    }
}

fn game_view_html(result: i64) -> Markup {
    html! {
        div {
            div #game_result hx-get="/game/current" hx-trigger="every 1s" {
              p { "Result: " (result) }
            }
           button
             hx-post="/game/dec"
             hx-target="#game_result"
             hx-swap="outerHTML"
            {
                ("-")
            }
           button
             hx-post="/game/inc"
             hx-target="#game_result"
             hx-swap="outerHTML"
            {
                ("+")
            }
        }
    }
}
