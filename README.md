# 🎨 Pakem Framework

<p align="center">
  <img src="https://raw.githubusercontent.com/pakem-rs/pakem/1.x/resources/images/logo.svg" alt="Pakem Framework Logo" width="160">
</p>

<p align="center">
  <strong>A beautiful, ultra-fast, and elegant web framework for Rust.</strong><br>
  <em>Crafted with Canting Precision, Driven by the Raw Speed of Rust.</em>
</p>

> **English Summary:** **Pakem** is a batteries-included Rust web framework built on **Tokio** and **Axum 0.8**, bringing Laravel-inspired developer ergonomics (`PakemModel` ORM, `pakem-blade` templating, `canting` CLI, dual Session/JWT auth, queues, and WebSockets) to Rust 2024. See [pakem.my.id](https://pakem.my.id/) for full documentation and [SECURITY.md](./SECURITY.md) for vulnerability reporting.

<p align="center">
  <a href="https://crates.io/crates/pakem"><img src="https://img.shields.io/crates/v/pakem.svg" alt="Crates.io Version"></a>
  <a href="https://crates.io/crates/canting"><img src="https://img.shields.io/badge/CLI-canting-ec4899.svg" alt="Canting CLI"></a>
  <a href="https://github.com/pakem-rs/framework"><img src="https://img.shields.io/badge/GitHub-pakem--rs%2Fframework-blue.svg" alt="GitHub Repo"></a>
  <a href="https://github.com/pakem-rs/pakem"><img src="https://img.shields.io/badge/Starter%20Kit-pakem--rs%2Fpakem-6366f1.svg" alt="Starter Kit"></a>
  <a href="#-fitur-utama"><img src="https://img.shields.io/badge/Database-PostgreSQL%20|%20MySQL%20|%20SQLite%20|%20MSSQL-06b6d4.svg" alt="Databases"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-amber.svg" alt="License"></a>
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/pakem-rs/pakem/1.x/resources/images/demo-polyglot-clone.gif" alt="Pakem Polyglot System Cloner Demo" width="880">
</p>

---

## 🌟 Mengapa Pakem?

Rust terkenal dengan kecepatan ekstrem dan keamanannya, namun sering kali memiliki kurva belajar yang curam dan membutuhkan banyak kode boilerplate untuk membuat aplikasi web sehari-hari.

**Pakem Framework** dirancang untuk mengubah hal tersebut. Mengusung prinsip **"Convention over Configuration"** (Pakem), framework ini menyediakan arsitektur MVC yang matang, *Dependency Injection* cerdas, *Human ORM*, sistem migrasi multi-database otomatis, serta CLI scaffolding **Canting** instan tanpa mengorbankan performa async berbasis **Tokio** dan **Axum**.

---

## 🚀 Fitur Utama

- 🔄 **Polyglot System Cloning Suite (`canting clone:*`)** — Kloning dan migrasikan aplikasi warisan (*legacy*) ke Pakem Framework dalam sekejap tanpa memulai dari nol:
  - `canting clone:laravel <path>` — Konversi otomatis Eloquent Model, Routes web/api, template Blade, dan Controller Laravel ke Rust.
  - `canting clone:ci <path>` — Modernisasi instan aplikasi CodeIgniter 3 & 4 ke arsitektur MVC Pakem.
  - `canting clone:api <spec>` — Scaffolding otomatis Rust Models & Axum handlers dari OpenAPI / Swagger JSON/YAML.
  - `canting clone:db <dsn>` — Reverse engineering skema database live (PostgreSQL, MySQL, SQLite, MSSQL) menjadi Model Pakem lengkap dengan relasi `has_many` dan `belongs_to`.
- 🔐 **Tri-Layer Legacy Auth & Migration** — Transisi login mulus dengan auto-rehash bertahap dari hash lawas (Laravel Bcrypt, CI3 SHA1, MD5) ke **Bcrypt (Cost 12) + HMAC-SHA256 Pepper (`APP_KEY`)**, dilengkapi pre-encoding opsional Sandi A-N serta flag `AUTH_ALLOW_LEGACY_WEAK_HASH=false` untuk menonaktifkan hash lemah pasca-migrasi.
- 🎨 **Pakem Aurora Starter Kit (`canting aurora:install`)** — Official starter kit autentikasi & dashboard elegan berbasis Pure Native Blade + Tailwind CSS dengan rendering HTML server-side yang ringan.
- 🛡️ **Enterprise Security & Panic Recovery** — Proteksi crash best-effort pada handler async via `catch_unwind` (bukan pengganti `Result`/`AppError`, tidak menangkap `panic=abort`/stack overflow), granular RBAC (`@can`, `@role`, `req.authorize()`), Anti Brute-Force lockout cache (Moka), constant-time comparison, header keamanan OWASP, dan audit log untuk membantu kontrol kepatuhan (seperti ISO 27001).
- 📖 **Portal Dokumentasi Resmi** — Tersedia 25 modul dokumentasi komprehensif di [https://pakem.my.id/](https://pakem.my.id/).
- ⚡ **Supercharged Tokio + Axum Core** — Performa HTTP async tinggi dengan konsumsi resource yang sangat hemat.
- 🏗️ **Arsitektur MVC Lengkap** — Pembagian layer yang rapi: Controllers, Models, Services, Requests, Views, dan Providers.
- 🗄️ **Human ORM & Macro-based Models** — Gunakan `#[derive(PakemModel)]` untuk mendapatkan fungsi CRUD otomatis, pagination (`LengthAwarePaginator`), eager loading, soft deletes, dan query builder lancar (*fluent*).
- 💎 **Zero-Ampersand ACID Transactions & Smart Soft Deletes** — Transaksi database ACID super bersih tanpa tanda ampersand `&tx`, tanpa pipa `|`, dan tanpa generic `<>` (`tx.table("accounts").await?` atau `req.table("accounts").await?`). Dilengkapi auto-rollback on drop, savepoint bertingkat, facade `DB`, dan soft deletes deklaratif anti-crash.
- 🏭 **Model Factory & Faker Engine** — Generator data tiruan (*mock / seeding*) berkecepatan tinggi dengan lokalisasi Indonesia & internasional serta chaining fluen ala Laravel (`User::factory().count(100).create(db).await`).
- 🔄 **Universal Multi-Database Layer** — Dukungan langsung untuk **PostgreSQL**, **MySQL**, **SQLite**, dan **Microsoft SQL Server (MSSQL)** dengan pure-Rust TDS driver (Tiberius), auto-migration runner, serta generator database flag di CLI (`--db`).
- 🪄 **Canting CLI (`canting`)** — Buat project baru dengan preset database, controller, model, factory (`make:factory`), audit keamanan (`audit`), autentikasi instan (`make:auth`), plugin WASM, dan modul CRUD lengkap (`make:crud`).
- 📦 **The Unified `Request` & Typed Extractors** — Gunakan `req: Request` untuk ergonomi penuh, atau extractor ringan idiomatik (`ExtractDb`, `OptionalUser`, `AppSession`) dan trait `RequestLike` untuk unit testing tanpa overhead ekstraksi yang tidak terpakai.
- 🗄️ **Raw `sqlx` Escape Hatch** — Selain `PakemModel` ORM, `DatabasePool` menyediakan akses langsung ke `db.raw_pool()`, `db.pg()`, `db.sqlite()`, dan `db.mysql()` untuk query SQLx native ber-type-safety penuh.
- 🛡️ **Pakem Shield Security Scanner (`canting audit`)** — Audit otomatis postur keamanan aplikasi (kekuatan secret, mode debug di produksi, proteksi CSRF form, storage hardening) dengan security scoreboard dan saran perbaikan.
- 🔐 **Dual Auth Suite** — Autentikasi berbasis **Session** untuk web serta **JWT Bearer Token** untuk API.
- 🎭 **Pure Native Blade Engine** — Template engine independen berkecepatan tinggi tanpa runtime eksternal. Mendukung direktif familiar (`@extends`, `@section`, `@if`, `@foreach`, `@csrf`), pipeline filter (`| rupiah`, `| uppercase`), dan render bebas tanda `&`.
- 🧩 **WASM Native Hybrid Plugin Engine** — Eksekusi plugin pihak ketiga terisolasi dalam sandbox WebAssembly (`wasmi 2.0`) yang aman dari crash memori dan terpantau di Telescope.
- 🇮🇩 **Indonesian Script (.is) Runtime** — Bahasa skrip berorientasi logika bisnis berbasis bahasa Indonesia natural (`fungsi`, `jika`, `selama`) yang dieksekusi langsung oleh backend Pakem.
- 📡 **Pakem Echo (WebSocket Broadcaster)** — Broadcasting event real-time dengan channel authorization (Public, Private, Presence) dan verifikasi JWT.
- ⏳ **Background Queue & Jobs** — Eksekusi antrean tugas asinkron dengan persistensi database.
- ⏰ **Task Scheduler** — Penjadwalan cron-job bawaan yang mudah dikonfigurasi.
- 💾 **High-Speed Cache** — Cache in-memory berbasis Moka dengan dukungan tagging dan TTL.
- 📧 **Mail & Notification** — Fluent email builder dengan template rendering dan driver Log/SMTP.
- 📁 **File Storage & Image Manipulation** — Penanganan upload multipart dan manipulasi gambar otomatis (resize, thumbnail, grayscale).
- 🔭 **Telescope & Health Checks** — Dasbor `/telescope` (Requests, Queries, Logs, Sandbox Metrics), health check `/up` & `/health`, graceful shutdown, dan security headers bawaan.

---

## 🛠️ Instalasi Canting CLI (`canting`)

Canting CLI (`canting`) adalah toolchain resmi Pakem Framework untuk membuat dan mengelola project dengan cepat dan elegan.

### Instalasi Resmi via Cargo (Crates.io):
```bash
cargo install canting
```

### Atau Instalasi Langsung dari GitHub Resmi:
```bash
cargo install --git https://github.com/pakem-rs/canting.git --force
```

> **Catatan:** Crate `canting` akan memasang executable binary `canting` secara global di sistem Anda.

Verifikasi instalasi CLI:
```bash
canting --version
canting --help
```

### Upgrade Toolchain & Update Dependensi Proyek:
```bash
# Memperbarui binary Canting CLI ke rilis terbaru
canting upgrade

# Memperbarui dependensi Cargo proyek (tampilan diff ala Composer)
canting update
canting update --pakem-only
```

---

## ⚡ Quick Start: Membuat Project Baru

### 1. Cara 1: Menggunakan Canting CLI (Rekomendasi)
```bash
# Membuat project baru (otomatis mengambil template starter terbaru dari GitHub)
canting new kontak_manager

# Atau pilih driver database tertentu (--db sqlite / postgres / mysql / mssql / all):
canting new kontak_manager --db postgres
```

### 2. Cara 2: Clone Starter Template Langsung dari GitHub
```bash
git clone https://github.com/pakem-rs/pakem.git kontak_manager
cd kontak_manager
cargo run
```

---

### 3. Konfigurasi Database (.env)
Buka file `.env` di dalam folder project baru Anda, lalu sesuaikan koneksi database.

#### Contoh PostgreSQL (Port Kustom, misal 54322):
```env
DB_CONNECTION=postgres
DB_HOST=127.0.0.1
DB_PORT=54322
DB_DATABASE=pakem_kontak
DB_USERNAME=postgres
DB_PASSWORD=postgress

DATABASE_URL=postgres://postgres:postgress@127.0.0.1:54322/pakem_kontak
```

#### Contoh Microsoft SQL Server (MSSQL):
```env
DB_CONNECTION=mssql
DB_HOST=127.0.0.1
DB_PORT=1433
DB_DATABASE=pakem_kontak
DB_USERNAME=sa
DB_PASSWORD=YourPassword123

DATABASE_URL="Server=tcp:127.0.0.1,1433;Database=pakem_kontak;User Id=sa;Password=YourPassword123;TrustServerCertificate=true;"
```

#### Contoh SQLite (Default, Tanpa Setup):
```env
DB_CONNECTION=sqlite
DATABASE_URL=sqlite:./database.sqlite
```

#### Contoh MySQL:
```env
DB_CONNECTION=mysql
DB_HOST=127.0.0.1
DB_PORT=3306
DB_DATABASE=nama_db
DB_USERNAME=root
DB_PASSWORD=secret
DATABASE_URL=mysql://root:secret@127.0.0.1:3306/nama_db
```

### 4. Menjalankan Server
```bash
cd kontak_manager
canting serve
# atau dengan fitur auto-reload:
canting watch
```
Kunjungi aplikasi di 👉 **http://127.0.0.1:8000** 🎉

---

## 📖 Tutorial: Membuat CRUD API & Service Kontak Manager (PostgreSQL)

Berikut adalah panduan langkah demi langkah implementasi CRUD Kontak Manager menggunakan PostgreSQL di Pakem:

### 1. File Migrasi SQL (`database/migrations/20261016000002_create_contacts_table.sql`)
```sql
CREATE TABLE IF NOT EXISTS contacts (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL,
    phone VARCHAR(50) NOT NULL,
    address TEXT,
    company VARCHAR(100),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_contacts_deleted_at ON contacts(deleted_at);
```

### 2. Model Kontak (`src/app/models/contact.rs`)
```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use pakem::prelude::*;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default, PakemModel)]
#[table("contacts")]
pub struct Contact {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: Option<String>,
    pub company: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}
```

### 3. Request Validation (`src/app/requests/contact_request.rs`)
```rust
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct CreateContactRequest {
    #[validate(length(min = 2, message = "Nama kontak minimal 2 karakter"))]
    pub name: String,

    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,

    #[validate(length(min = 8, message = "Nomor telepon minimal 8 digit"))]
    pub phone: String,

    pub address: Option<String>,
    pub company: Option<String>,
}
```

### 4. Controller Layer yang Cantik & Ergonomis (`src/app/controllers/contact_controller.rs`)

Dengan fitur **Pakem Eloquent ORM** dan **Unified Request Response**, Controller Anda sangat elegan, ekspresif, dan bebas dari boilerplate:

```rust
use pakem::prelude::*;
use crate::app::models::contact::Contact;
use crate::app::requests::contact_request::CreateContactRequest;

pub struct ContactController;

impl ContactController {
    /// GET /api/contacts — Ambil seluruh daftar kontak
    pub async fn api_index(req: Request) -> PakemResult {
        let contacts = Contact::all(req.db()).await?;
        req.ok(contacts)
    }

    /// GET /api/contacts/paged — Paginasi instan (otomatis membaca ?page=X&per_page=Y)
    pub async fn api_paginate(req: Request) -> PakemResult {
        let (page, per_page) = req.pagination();
        let paginator = Contact::paginate(req.db(), page, per_page).await?;
        req.paginate(paginator)
    }

    /// GET /api/contacts/:id — Ambil detail satu kontak
    pub async fn api_show(req: Request) -> PakemResult {
        let id = req.id_or_fail()?;
        match Contact::find(req.db(), id).await? {
            Some(contact) => req.ok(contact),
            None => req.not_found("Kontak tidak ditemukan"),
        }
    }

    /// POST /api/contacts — Simpan kontak baru (Active Record)
    pub async fn api_store(
        req: Request,
        ValidatedJson(payload): ValidatedJson<CreateContactRequest>,
    ) -> PakemResult {
        let mut contact = Contact {
            name: payload.name,
            email: payload.email,
            phone: payload.phone,
            address: payload.address,
            company: payload.company,
            ..Default::default()
        };
        let id = contact.save(req.db()).await?;
        contact.id = id;
        req.created_with(contact, "Kontak berhasil ditambahkan")
    }

    /// PUT /api/contacts/:id — Update kontak
    pub async fn api_update(
        req: Request,
        ValidatedJson(payload): ValidatedJson<CreateContactRequest>,
    ) -> PakemResult {
        let id = req.id_or_fail()?;
        let mut contact = match Contact::find(req.db(), id).await? {
            Some(c) => c,
            None => return req.not_found("Kontak tidak ditemukan"),
        };
        contact.name = payload.name;
        contact.email = payload.email;
        contact.phone = payload.phone;
        contact.address = payload.address;
        contact.company = payload.company;
        contact.save(req.db()).await?;
        req.success(contact, "Kontak berhasil diperbarui")
    }

    /// DELETE /api/contacts/:id — Soft delete kontak
    pub async fn api_destroy(req: Request) -> PakemResult {
        let id = req.id_or_fail()?;
        if Contact::destroy(req.db(), id).await? {
            req.success(serde_json::json!({ "id": id }), "Kontak berhasil dihapus")
        } else {
            req.not_found("Kontak tidak ditemukan")
        }
    }
}
```

### 5. Pendaftaran Routes (`src/routes/api.rs`)
```rust
use pakem::prelude::*;
use crate::app::controllers::contact_controller::ContactController;

pub fn router() -> Router<AppState> {
    Router::new()
        .get("/contacts", ContactController::api_index)
        .post("/contacts", ContactController::api_store)
        .get("/contacts/:id", ContactController::api_show)
        .put("/contacts/:id", ContactController::api_update)
        .delete("/contacts/:id", ContactController::api_destroy)
}
```

---

## 🛠️ Ringkasan Perintah CLI Canting

| Perintah | Deskripsi |
| :--- | :--- |
| `canting new <name> [--db <preset>]` | Membuat project baru (`--db sqlite`, `postgres`, `mysql`, atau `all`) |
| `canting serve` | Menjalankan server aplikasi HTTP |
| `canting watch` | Menjalankan server dengan fitur hot auto-reload (`cargo-watch`) |
| `canting make:crud <Name> [fields]` | Membuat CRUD lengkap instan (Model, Controller, Migrasi, View) |
| `canting make:auth` | Menghasilkan modul autentikasi lengkap (Register, Login, Session, Blade) |
| `canting make:controller <Name>` | Menghasilkan controller baru |
| `canting make:model <Name>` | Menghasilkan model baru beserta file migrasi |
| `canting make:factory <Name>` | Menghasilkan Model Factory baru untuk testing & seeding |
| `canting make:migration <Name>` | Membuat file migrasi database baru |
| `canting migrate` | Menjalankan seluruh file migrasi SQL yang belum diaplikasikan |
| `canting migrate:rollback` | Membatalkan migrasi terakhir |
| `canting db:seed` | Menjalankan database seeder |
| `canting audit` | Menjalankan Pakem Shield untuk audit keamanan & konfigurasi project |
| `canting plugin:new <name>` | Membuat scaffold plugin WebAssembly baru |
| `canting plugin:build <name>` | Membangun binary WASM target `wasm32-wasip1` |
| `canting plugin:list` | Mendaftar semua plugin WASM yang terpasang di `storage/plugins` |
| `canting make:docker` | Menghasilkan `Dockerfile` & `docker-compose.yml` multi-stage produksi |
| `canting make:nginx` | Menghasilkan konfigurasi Nginx reverse proxy produksi |
| `canting make:supervisor` | Menghasilkan konfigurasi Supervisor daemon |
| `canting key:generate` | Menghasilkan `APP_KEY` 256-bit (CSPRNG) dan opsi `--jwt` |
| `canting aurora:install` | Memasang starter kit autentikasi & dashboard **Pakem Aurora** |
| `canting clone:laravel \| ci \| api \| db` | Mengkloning aplikasi Laravel, CI, OpenAPI, atau skema DB live ke Pakem |
| `canting tinker` | Membuka REPL interaktif untuk mencoba query & model |
| `canting update [--pakem-only]` | Memperbarui dependensi proyek dengan ringkasan diff ala Composer |
| `canting upgrade` | Memperbarui binary `canting` CLI ke versi terbaru |

---

## 📁 Struktur Direktori Project Pakem

```
my_app/
├── Cargo.toml                  ← Konfigurasi dependensi project
├── .env                        ← Konfigurasi environment (DB, Port, JWT, dsb.)
├── database/
│   ├── migrations/             ← File migrasi SQL (auto-runner)
│   └── seeders/                ← File seeder data dummy
├── resources/
│   ├── views/                  ← Template tampilan (.blade.rs)
│   ├── js/                     ← Skrip client (Echo broadcaster, dsb.)
│   └── css/                    ← Lembar gaya styling
├── src/
│   ├── main.rs                 ← Entrypoint server & server runner
│   ├── routes/
│   │   ├── web.rs              ← Rute antarmuka Web HTML
│   │   └── api.rs              ← Rute JSON REST API
│   └── app/
│       ├── controllers/        ← Handler HTTP request
│       ├── models/             ← Definisi entitas & ORM database
│       ├── requests/           ← Skema validasi input form/JSON
│       └── services/           ← Layer logika bisnis aplikasi
└── storage/
    ├── app/public/             ← File upload publik
    └── cache/                  ← Cache file lokal
```

---

## 🧩 Status Modul & Feature Flags

| Modul / Crate | Feature Flag | Status | Catatan |
| :--- | :--- | :---: | :--- |
| Core Router, `Request`, Typed Extractors (`ExtractDb`, `OptionalUser`, `AppSession`) | *(Core)* | `Stable` | Di atas Tokio + Axum 0.8 |
| `pakem-blade` & `pakem-macros` | *(Core)* | `Stable` | Engine template & derive ORM |
| Multi-DB Pool (`sqlite`, `postgres`, `mysql`, `mssql`) | `sqlite`, `postgres`, `mysql`, `mssql` | `Stable` | Escape hatch `raw_pool()`, `pg()`, `sqlite()`, `mysql()` |
| Queue, Scheduler, Echo WebSocket | `queue`, `scheduler`, `echo` | `Stable` | Default single-node; driver `redis` opsional |
| WASM Sandbox & Indonesian Script (`.is`) | `wasm`, `is-engine` | `Experimental` | Dibatasi fuel/memori/timeout; jangan untuk input user mentah |

---

## 📚 Dokumentasi Lengkap & Seri Tutorial

Dokumentasi resmi Pakem Framework tersedia secara lengkap dan terstruktur di portal resmi:

👉 **[https://pakem.my.id/](https://pakem.my.id/)**

---

## 🧑‍💻 Cara Mengembangkan Lokal

Untuk berkontribusi atau mengembangkan ekosistem Pakem Framework secara lokal, letakkan seluruh repositori dalam satu folder induk yang sama dengan berkas `.cargo/config.toml` di root induk tersebut:

```text
pakem-workspace/
├── .cargo/
│   └── config.toml      ← [patch.crates-io] mengarah ke folder lokal
├── pakem-macros/        ← Procedural macros (PakemModel, pakem_form)
├── pakem-blade/         ← Pure Native Blade template engine
├── pakem-live/          ← Reactive component & WebSocket engine
├── framework/           ← Core crate (`pakem`)
├── canting/             ← CLI toolchain (`canting`)
└── pakem/               ← Starter kit aplikasi (`pakem-app`)
```

Isi `.cargo/config.toml` di folder induk:

```toml
[patch.crates-io]
pakem        = { path = "framework" }
pakem-macros = { path = "pakem-macros" }
pakem-blade  = { path = "pakem-blade" }
pakem-live   = { path = "pakem-live" }
```

Dengan konfigurasi ini, `cargo check`, `cargo test`, dan `cargo run` di setiap sub-proyek akan otomatis memakai kode sumber lokal tanpa perlu mengubah `Cargo.toml` masing-masing crate.

---

## 🧪 Menjalankan Pengujian (Testing)

Pakem Framework dilengkapi dengan suite testing integrasi lengkap untuk pengujian API CRUD, ORM, middleware, dan broadcasting:

```bash
# Menjalankan seluruh test suite framework
cargo test

# Menjalankan test integrasi CRUD API
cargo test --test integration_test

# Menjalankan test ORM
cargo test --test orm_human_features_test

# Menjalankan test Broadcaster Echo
cargo test --test production_features_test
```

---

## 📜 Dokumen Panduan & Riwayat Rilis

- 🤖 **[AGENTS.md](./AGENTS.md)** — Pedoman arsitektur dan aturan baku penulisan kode (*Laravel-like elegance, Request bundle, dan clean architecture*) untuk AI Agent & developer.
- 🔐 **[SECURITY.md](./SECURITY.md)** — Kebijakan pelaporan kerentanan keamanan (*responsible disclosure*) dan praktik terbaik produksi.
- 📝 **[CHANGELOG.md](./CHANGELOG.md)** — Catatan riwayat rilis dan pembaruan fitur per versi framework.

---

## 📄 License

Pakem Framework dilisensikan di bawah lisensi [MIT](LICENSE).
