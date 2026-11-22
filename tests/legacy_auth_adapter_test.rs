use pakem::prelude::*;

#[test]
fn test_tri_layer_hash_make_and_check() {
    let raw_pass = "RahasiaNusantara2026!";

    // 1. Buat hash dengan Tri-Layer Pakem (Layer 0 Sandi A-N -> Layer 1 Pepper -> Layer 2 Bcrypt)
    let hashed = Hash::make(raw_pass);

    assert!(hashed.starts_with("$2"), "Hash akhir harus berformat Bcrypt valid ($2b$ atau $2a$)");

    // 2. Verifikasi kata sandi benar
    assert!(Hash::check(raw_pass, &hashed), "Kata sandi benar harus lolos verifikasi");

    // 3. Verifikasi kata sandi salah
    assert!(!Hash::check("SalahPassword!", &hashed), "Kata sandi salah harus ditolak");

    // 4. Verifikasi migrasi: karena sudah format baru, needs_rehash harus FALSE
    let (is_valid, needs_rehash) = Hash::check_with_migration(raw_pass, &hashed);
    assert!(is_valid);
    assert!(!needs_rehash, "Format Tri-Layer modern tidak memerlukan re-hash");
}

#[test]
fn test_legacy_laravel_bcrypt_migration_detection() {
    let raw_pass = "LaravelLegacyPassword123";

    // Simulasi hash lama dari Laravel (Bcrypt murni tanpa Sandi A-N dan Pepper)
    let laravel_hash = bcrypt::hash(raw_pass, 10).expect("Bcrypt hash");

    // 1. User lama login di Pakem -> Harus tetap berhasil login!
    assert!(
        Hash::check(raw_pass, &laravel_hash),
        "User dari sistem Laravel lama harus bisa login langsung tanpa reset password!"
    );

    // 2. Sistem harus mendeteksi bahwa hash ini perlu di-rehash ke format Tri-Layer Pakem!
    let (is_valid, needs_rehash) = Hash::check_with_migration(raw_pass, &laravel_hash);
    assert!(is_valid);
    assert!(
        needs_rehash,
        "Hash legacy Laravel harus terdeteksi memerlukan migrasi/rehash transparan!"
    );
}

#[test]
fn test_legacy_codeigniter_sha1_migration_detection() {
    let raw_pass = "ci3_user_password";

    // Hash asli CodeIgniter 3: sha1("ci3_user_password")
    let ci_hash = "47f1b4f5e8d82df38e96bcf63db5ea7a02074ad2";

    // 1. User lama login di Pakem
    assert!(
        Hash::check(raw_pass, ci_hash),
        "User dari CodeIgniter lama harus bisa login langsung!"
    );

    // 2. Terdeteksi butuh rehash
    let (is_valid, needs_rehash) = Hash::check_with_migration(raw_pass, ci_hash);
    assert!(is_valid);
    assert!(needs_rehash);
}

#[test]
fn test_legacy_md5_migration_detection() {
    let raw_pass = "php_native_old_password";

    // Hash asli MD5 PHP lama: md5("php_native_old_password")
    let md5_hash = "016aab8a459bf4c982087e548d9faadd";

    // 1. User lama login di Pakem
    assert!(
        Hash::check(raw_pass, md5_hash),
        "User dari PHP Native / MD5 lama harus bisa login langsung!"
    );

    // 2. Terdeteksi butuh rehash
    let (is_valid, needs_rehash) = Hash::check_with_migration(raw_pass, md5_hash);
    assert!(is_valid);
    assert!(needs_rehash);
}
