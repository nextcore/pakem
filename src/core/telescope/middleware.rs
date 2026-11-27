use crate::core::application::AppState;
use crate::core::telescope::entry::RequestContent;
use crate::core::telescope::TelescopeManager;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use std::time::Instant;

fn redact_uri_query(uri: &str) -> String {
    let Some((path, query)) = uri.split_once('?') else {
        return uri.to_string();
    };
    let redacted_pairs: Vec<String> = query
        .split('&')
        .map(|pair| {
            if let Some((k, _)) = pair.split_once('=') {
                let kl = k.to_ascii_lowercase();
                if kl.contains("token")
                    || kl.contains("password")
                    || kl.contains("secret")
                    || kl.contains("key")
                    || kl.contains("auth")
                {
                    return format!("{}=[REDACTED]", k);
                }
            }
            pair.to_string()
        })
        .collect();
    format!("{}?{}", path, redacted_pairs.join("&"))
}

pub async fn telescope_middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Response {
    if !TelescopeManager::is_enabled() {
        return next.run(req).await;
    }

    // Abaikan request ke dashboard telescope itu sendiri agar tidak rekursif berlebihan
    let path = req.uri().path();
    if path.starts_with("/pakem/telescope") || path.starts_with("/storage") {
        return next.run(req).await;
    }

    let start = Instant::now();
    let method = req.method().to_string();
    let uri = redact_uri_query(&req.uri().to_string());

    // IP detection (simple version)
    let ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("127.0.0.1")
        .to_string();

    let response = next.run(req).await;

    let duration = start.elapsed().as_millis();
    let status = response.status().as_u16();

    let content = RequestContent {
        method,
        uri,
        status,
        duration_ms: duration,
        ip,
    };

    state.telescope.record_request(content);

    response
}
