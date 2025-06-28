// src/handlers.rs
use crate::models::{ApiResponse, CronQuery};
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use chrono::Local;
use croner::describe::lang::swedish::Swedish;
use croner::Cron;

/// Handles incoming requests to the `/api/test` endpoint for cron evaluation.
pub async fn test_cron_handler(Query(query): Query<CronQuery>) -> impl IntoResponse {
    // 1. Create a Cron instance with the specified options
    let mut cron = Cron::new(&query.expression);

    match query.seconds.as_str() {
        "optional" => cron.with_seconds_optional(),
        "required" => cron.with_seconds_required(),
        _ => &mut cron, // "disabled", do nothing
    };

    if query.dom_and_dow {
        cron.with_dom_and_dow();
    }

    // 2. Try to parse the expression
    let parsed_cron = match cron.parse() {
        Ok(c) => c,
        Err(e) => {
            let error_message = format!("Invalid cron expression: {}", e);
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse {
                    occurrences: vec![],
                    description: "".to_string(),
                    warnings: vec![error_message],
                }),
            )
                .into_response();
        }
    };

    // 3. Generate occurrences
    let occurrences: Vec<String> = parsed_cron
        .iter_from(Local::now())
        .take(5)
        .map(|dt| dt.to_string())
        .collect();

    // 4. Generate the description in the selected language
    let description = match query.lang.as_str() {
        "sv" => parsed_cron.describe_lang(Swedish::default()),
        _ => parsed_cron.describe(), // Default to English
    };

    // 5. Send the response
    (
        StatusCode::OK,
        Json(ApiResponse {
            occurrences,
            description,
            warnings: vec![], // For simplicity, warnings are not implemented in this demo
        }),
    )
        .into_response()
}