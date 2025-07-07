// src/handlers.rs
use crate::models::{ApiResponse, CronQuery};
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use chrono::Local;
use croner::describe::lang::swedish::Swedish;
use croner::parser::{CronParser, Seconds, Year};

/// Handles incoming requests to the `/api/test` endpoint for cron evaluation.
pub async fn test_cron_handler(Query(query): Query<CronQuery>) -> impl IntoResponse {
    // 1. Create a CronParser builder and configure it
    let mut builder = CronParser::builder();

    builder = builder.seconds(match query.seconds.as_str() {
        "optional" => Seconds::Optional,
        "required" => Seconds::Required,
        _ => Seconds::Disallowed,
    });

    builder = builder.year(match query.year.as_str() {
        "optional" => Year::Optional,
        "required" => Year::Required,
        _ => Year::Disallowed,
    });

    if query.dom_and_dow {
        builder = builder.dom_and_dow(true);
    }

    if query.alternative_weekdays {
        builder = builder.alternative_weekdays(true);
    }

    let parser = builder.build();

    // 2. Try to parse the expression
    let parsed_cron = match parser.parse(&query.expression) {
        Ok(c) => c,
        Err(e) => {
            let error_message = format!("Invalid cron expression: {}", e);
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse {
                    occurrences: vec![],
                    previous_occurrences: vec![],
                    description: "".to_string(),
                    warnings: vec![error_message],
                }),
            )
                .into_response();
        }
    };

    // 3. Generate occurrences
    let now = Local::now();
    let occurrences: Vec<String> = parsed_cron
        .iter_after(now)
        .take(5)
        .map(|dt| dt.to_string())
        .collect();

    let previous_occurrences: Vec<String> = parsed_cron
        .iter_before(now)
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
            previous_occurrences,
            description,
            warnings: vec![], // For simplicity, warnings are not implemented in this demo
        }),
    )
        .into_response()
}