// src/models.rs
use serde::{Deserialize, Serialize};

/// Defines the query parameters the API accepts from the frontend.
#[derive(Deserialize)]
pub struct CronQuery {
    pub expression: String,
    pub seconds: String, // "required", "optional", or "disabled"
    pub dom_and_dow: bool,
    pub lang: String, // "en" or "sv"
}

/// Defines the JSON response structure the API sends back.
#[derive(Serialize)]
pub struct ApiResponse {
    pub occurrences: Vec<String>,
    pub description: String,
    pub warnings: Vec<String>,
}