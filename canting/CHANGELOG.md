# 📝 Changelog — `canting`

Semua perubahan penting pada perkakas CLI **Canting** (`canting`) didokumentasikan di dalam berkas ini.

Format pencatatan mengacu pada [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) dan mengikuti standar [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.0.5] - 2026-11-27

### 🦀 Rust 2024 Edition & Axum 0.8 Stack
- Migrasi manifest ke Rust 2024 Edition (`edition = "2024"`).
- Sinkronisasi dependensi dan integrasi runtime web Axum 0.8.
- Pembersihan tag author personal dari manifest Cargo.

---

## [1.0.4] - 2026-11-26

### 🔑 Application Key Generator (`canting key:generate`)
- Menambahkan perintah `canting key:generate` (alias: `pakem key:generate`) untuk membuat `APP_KEY` 256-bit berbasis CSPRNG.
- Flag `--show`, `--force`, `--env <path>`, dan `--jwt` untuk sinkronisasi `JWT_SECRET`.
- Auto-generate `APP_KEY` saat scaffolding proyek baru (`canting new`).
