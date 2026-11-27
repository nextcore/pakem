# 📊 Pakem Framework — Reproducible Benchmark Suite

Direktori ini menyediakan skrip dan server referensi agar benchmark **Pakem Framework** dapat direproduksi secara transparan, termasuk skenario **DB-bound** (yang mengukur overhead router/extractor/DB pool secara nyata).

---

## 🛠️ Cara Menjalankan

1. Jalankan server benchmark dalam mode `--release`:
   ```bash
   cargo run --release --example bench_server
   ```
2. Di terminal terpisah, jalankan skrip benchmark (membutuhkan [`oha`](https://github.com/hatoo/oha), `wrk`, atau `cargo run --release --example bench_load`):
   ```powershell
   .\benches\run_bench.ps1
   ```
   Atau langsung dengan `oha`:
   ```bash
   oha -z 15s -c 128 http://127.0.0.1:8080/plaintext
   oha -z 15s -c 128 http://127.0.0.1:8080/json
   oha -z 15s -c 128 http://127.0.0.1:8080/db-users
   ```

---

## 📋 Hasil Benchmark Terukur (Uji Ulang Serentak di Mesin Lokal)

Semua 4 framework diuji ulang secara berurutan pada mesin yang sama menggunakan `examples/bench_load.rs` (`reqwest` HTTP/1.1 keep-alive, `64` koneksi konkuren × `5s` per endpoint, `err=0`):

- **Arsitektur / Core / OS**: `AMD64` · `12 logical cores` · `Microsoft Windows NT 10.0.26300.0`
- **Rustc & Build**: `rustc 1.99.1 (58cf8f9c3 2026-10-13)` · `--release`
- **PHP / Octane**: `FrankenPHP 1.14.0` · `PHP 8.6.0` (4 worker threads, OPcache ON, `APP_ENV=production`, `SESSION_DRIVER=array`)
- **Pakem Config**: `APP_ENV=production`, `RATE_LIMIT_MAX=10000000` (full middleware stack aktif)

| Framework | `/plaintext` (RPS) | `/json` (RPS) | `/db-users` (RPS) | p50 Latency | p99 Latency |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **Axum 0.8 (polos)** | **71,700.9 RPS** | **70,605.7 RPS** | **34,463.9 RPS** | 0.86–1.80 ms | 1.62–3.02 ms |
| **Actix-web 4.20.0** | **67,219.5 RPS** | **66,934.5 RPS** | **30,251.6 RPS** | 0.92–2.06 ms | 1.92–3.23 ms |
| **Pakem 1.0.5 (Minimal / `.without_default_middleware()`)** | **61,390.5 RPS** | **60,488.2 RPS** | **34,090.1 RPS** | 1.00–1.85 ms | 2.00–2.32 ms |
| **Pakem 1.0.5 (Full Stack Default)** | **5,207.8 RPS** *(Request)* | **7,573.6 RPS** *(Request)* | **7,364.9 RPS** *(ExtractDb)* | 8.20–9.94 ms | 13.90–35.08 ms |
| **Laravel 13 Octane (FrankenPHP)** | **652.2 RPS** | **662.4 RPS** | **594.1 RPS** | 95.39–106.41 ms | 116.36–147.96 ms |

> [!NOTE]
> **Kesimpulan Jujur dari Data:**
> 1. **Apples-to-Apples (`without_default_middleware()`)**: Saat middleware bawaan dimatikan, **Pakem 1.0.5** mencapai **61,390 RPS** (`/plaintext`), **60,488 RPS** (`/json`), dan **34,090 RPS** (`/db-users`, p99 **2.32 ms**) — setara **98.9%** performa Axum 0.8 polos di endpoint DB dan **melampaui Actix-web** (`30,251 RPS`) di `/db-users`.
> 2. **Full Stack vs Laravel Octane**: Dalam mode default *batteries-included* (Session SQLite, CSRF, rate limiter, i18n, compression, security headers), **Pakem 1.0.5** tetap **8× – 12.4× lebih cepat** daripada Laravel 13 Octane (FrankenPHP) (`7,364.9 RPS` vs `594.1 RPS` di `/db-users`).
