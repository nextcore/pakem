use axum::{
    extract::{State, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use crate::core::application::AppState;
use crate::core::telescope::entry::EntryType;
use crate::core::telescope::TelescopeManager;
use crate::core::blade::BladeContext as Context;
use serde::Deserialize;

pub struct TelescopeController;

#[derive(Deserialize)]
pub struct TelescopeQuery {
    pub tag: Option<String>,
}

impl TelescopeController {
    /// GET /pakem/telescope
    pub async fn index(
        State(state): State<AppState>,
        Query(query): Query<TelescopeQuery>,
    ) -> impl IntoResponse {
        if !TelescopeManager::is_enabled() {
            return (
                StatusCode::NOT_FOUND,
                Html("<h1>404 Not Found</h1><p>Telescope dinonaktifkan di lingkungan production.</p>".to_string()),
            )
                .into_response();
        }
        let mut context = Context::new();

        let tag = query.tag.unwrap_or_else(|| "requests".to_string());
        context.insert("active_tag", &tag);

        let entries = match tag.as_str() {
            "requests" => state.telescope.get_by_type(EntryType::Request),
            "events" => state.telescope.get_by_type(EntryType::Event),
            "jobs" => state.telescope.get_by_type(EntryType::Job),
            "logs" => state.telescope.get_by_type(EntryType::Log),
            "sandbox" => state.telescope.get_by_type(EntryType::Sandbox),
            _ => state.telescope.all(),
        };

        let metrics = state.sandbox.get_metrics();
        context.insert("sandbox_metrics", &metrics);
        context.insert("entries", &entries);

        let rendered = state.view.render("telescope/index.blade.rs", &context);
        Html(rendered).into_response()
    }

    /// GET /pakem/telescope/entries (Partial for HTMX)
    pub async fn entries(
        State(state): State<AppState>,
        Query(query): Query<TelescopeQuery>,
    ) -> impl IntoResponse {
        if !TelescopeManager::is_enabled() {
            return (StatusCode::NOT_FOUND, Html(String::new())).into_response();
        }
        let mut context = Context::new();
        let tag = query.tag.unwrap_or_else(|| "requests".to_string());

        let entries = match tag.as_str() {
            "requests" => state.telescope.get_by_type(EntryType::Request),
            "events" => state.telescope.get_by_type(EntryType::Event),
            "jobs" => state.telescope.get_by_type(EntryType::Job),
            "logs" => state.telescope.get_by_type(EntryType::Log),
            "sandbox" => state.telescope.get_by_type(EntryType::Sandbox),
            _ => state.telescope.all(),
        };

        let metrics = state.sandbox.get_metrics();
        context.insert("sandbox_metrics", &metrics);
        context.insert("entries", &entries);
        context.insert("active_tag", &tag);

        let rendered = state.view.render("telescope/partials/entries.blade.rs", &context);
        Html(rendered).into_response()
    }
}
