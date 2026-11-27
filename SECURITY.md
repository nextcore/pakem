# 🔐 Security Policy — Pakem Framework

Keamanan adalah prioritas utama di **Pakem Framework**. Kami menghargai kontribusi peneliti keamanan dan komunitas untuk melaporkan potensi kerentanan secara bertanggung jawab (*responsible disclosure*).

---

## 📦 Versi yang Didukung

| Versi | Status Dukungan |
| :---: | :--- |
| `1.0.x` | ✅ Didukung (Security & Bug Fixes) |
| `< 1.0.0` | ❌ Tidak lagi didukung |

---

## 🚨 Melaporkan Kerentanan Keamanan

**Mohon JANGAN membuka GitHub Issue publik untuk kerentanan keamanan.**

Silakan laporkan kerentanan secara privat melalui:
1. **GitHub Security Advisories**: Gunakan tab **Security → Advisories → Report a vulnerability** di repositori [pakem-rs/framework](https://github.com/pakem-rs/framework/security/advisories).
2. Sertakan informasi berikut agar kami dapat mereproduksi dengan cepat:
   - Deskripsi kerentanan dan potensi dampaknya.
   - Langkah reproduksi (*proof of concept* / payload minimal).
   - Versi `pakem` dan konfigurasi database/fitur yang digunakan.

Kami menargetkan respons awal dalam **48 jam** dan pembaruan status hingga perbaikan dirilis.

---

## 🛡️ Batasan Keamanan & Praktik Terbaik Produksi

1. **Kunci Aplikasi (`APP_KEY` & `JWT_SECRET`)**:
   - Selalu bangkitkan kunci unik per lingkungan menggunakan `canting key:generate --jwt`. Jangan pernah menggunakan kunci default di produksi.
2. **Legacy Hash Opt-Out (`AUTH_ALLOW_LEGACY_WEAK_HASH`)**:
   - Setelah masa migrasi dari Laravel/CodeIgniter/MD5 selesai, set `AUTH_ALLOW_LEGACY_WEAK_HASH=false` di `.env` agar verifikasi SHA-1 dan MD5 dinonaktifkan sepenuhnya.
3. **WASM & Indonesian Script (`@wasm`, `@eval_is`)**:
   - Jangan pernah mengeksekusi sumber skrip atau modul WASM yang berasal dari input pengguna yang tidak tepercaya.
4. **Telescope & Debug Mode**:
   - Pastikan `APP_ENV=production` dan `APP_DEBUG=false` di lingkungan produksi serta lindungi rute observabilitas dengan autentikasi/otorisasi admin.
