use axum::{
    body::Body,
    extract::State,
    http::{HeaderValue, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::core::application::AppState;
use serde::Serialize;

/// Konfigurasi Rate Limiter yang bisa diatur via .env
#[derive(Clone, Debug)]
pub struct RateLimitConfig {
    /// Jumlah maksimum request per window
    pub max_requests: u32,
    /// Durasi window dalam detik
    pub window_seconds: u64,
}

impl RateLimitConfig {
    pub fn new(max_requests: u32, window_seconds: u64) -> Self {
        Self {
            max_requests,
            window_seconds,
        }
    }

    /// Baca konfigurasi dari environment variables:
    /// - `RATE_LIMIT_MAX` (default: 60)
    /// - `RATE_LIMIT_WINDOW` (default: 60 detik)
    pub fn from_env() -> Self {
        let max = std::env::var("RATE_LIMIT_MAX")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60u32);
        let window = std::env::var("RATE_LIMIT_WINDOW")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60u64);
        Self::new(max, window)
    }

    /// Preset ketat untuk endpoint sensitif seperti login, register, forgot password
    pub fn strict() -> Self {
        Self::new(10, 60) // 10 request per menit
    }

    /// Preset longgar untuk API umum
    pub fn api() -> Self {
        Self::new(120, 60) // 120 request per menit
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self::from_env()
    }
}

/// Data yang disimpan per-IP di CacheManager
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
struct RateLimitEntry {
    count: u32,
    window_start_secs: u64, // Unix timestamp saat window dimulai
}

#[derive(Serialize)]
struct RateLimitError {
    error: &'static str,
    message: String,
    retry_after: u64,
}

/// Middleware Rate Limiter berbasis IP menggunakan CacheManager (moka) yang sudah ada.
///
/// Cara penggunaan sebagai global middleware:
/// ```rust,ignore
/// // Di application.rs atau route builder:
/// use pakem::core::rate_limit::{RateLimitConfig, rate_limit_middleware};
/// use axum::middleware;
///
/// let config = RateLimitConfig::from_env();
/// router.layer(middleware::from_fn_with_state(state, rate_limit_middleware))
/// ```
pub async fn rate_limit_middleware(
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let config = RateLimitConfig::from_env();
    rate_limit_with_config(state, config, req, next).await
}

/// Middleware Rate Limiter dengan konfigurasi kustom (berguna untuk per-route).
pub async fn rate_limit_with_config(
    state: AppState,
    config: RateLimitConfig,
    req: Request<Body>,
    next: Next,
) -> Response {
    // Ekstrak IP dari header X-Forwarded-For atau X-Real-IP (untuk behind proxy)
    // Fallback ke "unknown" jika tidak ada
    let client_ip = req
        .headers()
        .get("X-Forwarded-For")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .or_else(|| req.headers().get("X-Real-IP").and_then(|v| v.to_str().ok()))
        .unwrap_or("unknown")
        .trim()
        .to_string();

    let cache_key = format!("rate_limit:{}", client_ip);
    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let cache = state.cache();

    // Ambil entry yang ada dari cache
    let mut entry: RateLimitEntry =
        cache
            .get::<RateLimitEntry>(&cache_key)
            .await
            .unwrap_or(RateLimitEntry {
                count: 0,
                window_start_secs: now_secs,
            });

    // Cek apakah window sudah expired (reset counter)
    let window_expired = now_secs.saturating_sub(entry.window_start_secs) >= config.window_seconds;
    if window_expired {
        entry = RateLimitEntry {
            count: 0,
            window_start_secs: now_secs,
        };
    }

    // Hitung sisa waktu window (untuk header Retry-After)
    let window_remaining = config
        .window_seconds
        .saturating_sub(now_secs.saturating_sub(entry.window_start_secs));
    let requests_remaining = config.max_requests.saturating_sub(entry.count);

    // Cek apakah limit terlampaui
    if entry.count >= config.max_requests {
        tracing::warn!(
            ip = %client_ip,
            limit = config.max_requests,
            window_secs = config.window_seconds,
            "Rate limit exceeded"
        );

        let path = req.uri().path();
        let accept_html = req
            .headers()
            .get("Accept")
            .and_then(|v| v.to_str().ok())
            .map(|a| a.contains("text/html"))
            .unwrap_or(false);
        let is_xhr = req
            .headers()
            .get("X-Requested-With")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.eq_ignore_ascii_case("XMLHttpRequest"))
            .unwrap_or(false);
        let wants_html = accept_html && !is_xhr && !path.starts_with("/api");

        let mut response = if wants_html {
            let secs = window_remaining.max(1);
            let html = format!(
                r#"<!DOCTYPE html>
<html lang="id">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Mohon Tunggu Sebentar — The Gege</title>
<style>
  * {{ box-sizing: border-box; margin: 0; padding: 0; }}
  body {{
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #F6F2EB;
    color: #1C1917;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    padding: 20px;
  }}
  .card {{
    max-width: 440px;
    width: 100%;
    background: #FFFFFF;
    border: 1px solid #E2DAD0;
    border-radius: 20px;
    padding: 36px 28px;
    text-align: center;
    box-shadow: 0 20px 50px rgba(28, 25, 23, 0.08);
  }}
  .ring-wrap {{
    position: relative;
    width: 108px;
    height: 108px;
    margin: 0 auto 22px;
  }}
  .ring-wrap svg {{
    transform: rotate(-90deg);
    width: 108px;
    height: 108px;
  }}
  .ring-bg {{ fill: none; stroke: #EFE9DF; stroke-width: 7; }}
  .ring-fg {{
    fill: none;
    stroke: #B85D3B;
    stroke-width: 7;
    stroke-linecap: round;
    stroke-dasharray: 289;
    stroke-dashoffset: 0;
    transition: stroke-dashoffset 0.95s linear;
  }}
  .countdown {{
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }}
  .countdown-num {{
    font-size: 30px;
    font-weight: 800;
    color: #1C1917;
    line-height: 1;
  }}
  .countdown-unit {{
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.14em;
    color: #78716C;
    margin-top: 4px;
  }}
  .badge {{
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 5px 12px;
    border-radius: 999px;
    background: #EFE9DF;
    color: #B85D3B;
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.12em;
    text-transform: uppercase;
    margin-bottom: 14px;
  }}
  .dot {{
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: #B85D3B;
    animation: pulse 1.2s ease-in-out infinite;
  }}
  @keyframes pulse {{
    0%, 100% {{ opacity: 1; transform: scale(1); }}
    50% {{ opacity: 0.35; transform: scale(0.75); }}
  }}
  h1 {{ font-size: 20px; font-weight: 700; margin-bottom: 10px; color: #1C1917; }}
  p {{ font-size: 14px; line-height: 1.6; color: #57534E; margin-bottom: 22px; }}
  .btn {{
    display: inline-block;
    padding: 11px 22px;
    border-radius: 10px;
    background: #B85D3B;
    color: #FFFFFF;
    font-size: 12px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    text-decoration: none;
    border: none;
    cursor: pointer;
    opacity: 0.55;
    pointer-events: none;
    transition: all 0.2s;
  }}
  .btn.ready {{
    opacity: 1;
    pointer-events: auto;
  }}
  .btn.ready:hover {{ background: #9E4B2B; }}
</style>
</head>
<body>
  <div class="card">
    <div class="badge"><span class="dot"></span> Antrean Request</div>
    <div class="ring-wrap">
      <svg viewBox="0 0 100 100">
        <circle class="ring-bg" cx="50" cy="50" r="46"></circle>
        <circle class="ring-fg" id="ringFg" cx="50" cy="50" r="46"></circle>
      </svg>
      <div class="countdown">
        <span class="countdown-num" id="secNum">{secs}</span>
        <span class="countdown-unit">detik</span>
      </div>
    </div>
    <h1>Request Anda Terlalu Banyak</h1>
    <p>Silakan tunggu sebentar lagi. Halaman akan dimuat ulang secara otomatis dalam <strong id="secText">{secs}</strong> detik.</p>
    <button type="button" id="retryBtn" class="btn" onclick="window.location.reload()">Muat Ulang Halaman</button>
  </div>
  <script>
    (function() {{
      var total = {secs};
      var left = total;
      var numEl = document.getElementById('secNum');
      var txtEl = document.getElementById('secText');
      var ring = document.getElementById('ringFg');
      var btn = document.getElementById('retryBtn');
      var circ = 289;
      var timer = setInterval(function() {{
        left = Math.max(0, left - 1);
        if (numEl) numEl.textContent = left;
        if (txtEl) txtEl.textContent = left;
        if (ring) ring.style.strokeDashoffset = String(circ * (1 - (left / total)));
        if (left <= 0) {{
          clearInterval(timer);
          if (btn) btn.classList.add('ready');
          window.location.reload();
        }}
      }}, 1000);
    }})();
  </script>
</body>
</html>"#,
                secs = secs
            );
            (StatusCode::TOO_MANY_REQUESTS, axum::response::Html(html)).into_response()
        } else {
            let error_body = serde_json::json!(RateLimitError {
                error: "Too Many Requests",
                message: format!(
                    "Terlalu banyak request. Silakan coba lagi dalam {} detik.",
                    window_remaining
                ),
                retry_after: window_remaining,
            });
            (StatusCode::TOO_MANY_REQUESTS, axum::Json(error_body)).into_response()
        };

        // Tambahkan rate limit headers
        let headers = response.headers_mut();
        headers.insert(
            "X-RateLimit-Limit",
            HeaderValue::from_str(&config.max_requests.to_string()).unwrap(),
        );
        headers.insert("X-RateLimit-Remaining", HeaderValue::from_static("0"));
        headers.insert(
            "Retry-After",
            HeaderValue::from_str(&window_remaining.to_string()).unwrap(),
        );

        return response;
    }

    // Increment counter dan simpan kembali
    entry.count += 1;
    cache
        .put(&cache_key, &entry, config.window_seconds + 5)
        .await;

    // Lanjutkan ke handler berikutnya
    let mut response = next.run(req).await;

    // Inject X-RateLimit-* headers ke response
    let headers = response.headers_mut();
    headers.insert(
        "X-RateLimit-Limit",
        HeaderValue::from_str(&config.max_requests.to_string())
            .unwrap_or(HeaderValue::from_static("60")),
    );
    headers.insert(
        "X-RateLimit-Remaining",
        HeaderValue::from_str(&requests_remaining.saturating_sub(1).to_string())
            .unwrap_or(HeaderValue::from_static("0")),
    );

    response
}
