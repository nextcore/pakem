use bcrypt::{hash, DEFAULT_COST};
use crate::core::security::legacy_adapter::LegacyAuthAdapter;
use crate::core::security::sandi_an::SandiAN;

/// Helper untuk menyelesaikan APP_KEY rahasia aplikasi dari .env
fn resolve_app_key() -> String {
    std::env::var("APP_KEY")
        .or_else(|_| std::env::var("JWT_SECRET"))
        .unwrap_or_else(|_| "pakem-app-secret-pepper-key-2026".to_string())
}

/// Menghasilkan hash kata sandi berstandar Pakem Framework:
/// 1. Layer 0 (Opsional / Obfuscation): Pre-encoding Sandi A-N Nusantara
/// 2. Layer 1 (Kriptografi): HMAC-SHA256 Secret Pepper menggunakan APP_KEY
/// 3. Layer 2 (Kriptografi): Bcrypt Slow Hash (Cost: 12)
pub fn make(password: &str) -> String {
    let app_key = resolve_app_key();
    let sandi = SandiAN::encode_with_key(password, &app_key);
    let pepper = LegacyAuthAdapter::compute_pepper(&sandi, &app_key);
    hash(pepper, DEFAULT_COST).expect("Gagal melakukan hashing password Tri-Layer Pakem")
}

/// Verifikasi apakah kata sandi mentah cocok dengan hash di database.
/// Mendukung kompatibilitas multi-framework (Pakem Tri-Layer, Laravel Bcrypt, CodeIgniter SHA1, dan MD5).
pub fn check(password: &str, hashed: &str) -> bool {
    let app_key = resolve_app_key();
    let (is_valid, _) = LegacyAuthAdapter::verify_and_check_migration(password, hashed, Some(&app_key));
    is_valid
}

/// Verifikasi kata sandi sekaligus memeriksa apakah hash membutuhkan migrasi / re-hash transparan.
/// 
/// Mengembalikan:
/// - `is_valid: bool`: Kata sandi benar/salah.
/// - `needs_rehash: bool`: True jika kata sandi cocok tetapi masih berformat lama (Laravel/CI/MD5).
/// 
/// # Contoh Penggunaan di Auth Controller:
/// ```rust,no_run
/// use pakem::core::auth::hash;
/// 
/// let user_password = "$2y$10$...";
/// let (valid, needs_rehash) = hash::check_with_migration("password123", user_password);
/// if valid {
///     if needs_rehash {
///         let new_hash = hash::make("password123");
///         // Update database secara otomatis: User::query(req.db()).where_eq("id", user.id).update(...);
///     }
///     // Lanjutkan proses login
/// }
/// ```
pub fn check_with_migration(password: &str, hashed: &str) -> (bool, bool) {
    let app_key = resolve_app_key();
    LegacyAuthAdapter::verify_and_check_migration(password, hashed, Some(&app_key))
}
