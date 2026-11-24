use pakem::core::security::sandi_an::SandiAN;
use std::time::Instant;

#[test]
fn test_sandi_an_encode_decode_reversibility() {
    let test_cases = vec![
        "admin",
        "secret",
        "password",
        "1234567890",
        "admin123",
        "Rahasia2026!",
        "KopiPekalongan#1",
        "a",
        "ab",
        "abc",
        "abcd",
        "P@ssw0rd_With_Very_Long_String_And_Special_Characters_!#$&*()",
    ];

    for original in test_cases {
        let encoded = SandiAN::encode(original);

        // 1. Pastikan memiliki prefix $an1$
        assert!(
            encoded.starts_with("$an1$"),
            "Encoded string harus ber-prefix $an1$, got: {}",
            encoded
        );

        // 2. Pastikan hasil encode berbeda dari teks asli
        assert_ne!(
            encoded, original,
            "Hasil transformasi harus mengaburkan teks asli!"
        );

        // 3. Pastikan string hasil decode 100% sama dengan teks asli
        let decoded = SandiAN::decode(&encoded).expect("Decode harus berhasil");
        assert_eq!(
            decoded, original,
            "Decode harus mengembalikan teks asli secara presisi untuk '{}'",
            original
        );
    }
}

#[test]
fn test_sandi_an_character_mapping_correctness() {
    // Uji substitusi A-N huruf kecil: 'a' <-> 'n', 'b' <-> 'o', 'm' <-> 'z'
    // Input "ab":
    // S_BOX: 'a' -> 'n', 'b' -> 'o' => ['n', 'o']
    // Interleaving chunk swap (0, 1) => ['o', 'n']
    let enc_ab = SandiAN::encode("ab");
    assert_eq!(enc_ab, "$an1$on");
    assert_eq!(SandiAN::decode(&enc_ab).unwrap(), "ab");

    // Uji substitusi angka 0-5: '0' <-> '5', '1' <-> '6', '2' <-> '7'
    // Input "01":
    // S_BOX: '0' -> '5', '1' -> '6' => ['5', '6']
    // Interleaving swap (0, 1) => ['6', '5']
    let enc_01 = SandiAN::encode("01");
    assert_eq!(enc_01, "$an1$65");
    assert_eq!(SandiAN::decode(&enc_01).unwrap(), "01");
}

#[test]
fn test_sandi_an_hybrid_key_dependent_reversibility_and_isolation() {
    let app_key_alpha = "pakem-secret-app-key-pro-2026";
    let app_key_beta = "different-company-secret-key-999";

    let test_passwords = vec![
        "admin123",
        "Rahasia2026!",
        "KopiPekalongan#1",
        "P@ssw0rd_With_Very_Long_String_And_Special_Characters_!#$&*()",
    ];

    for pwd in test_passwords {
        // 1. Encode dengan app_key_alpha
        let enc_alpha = SandiAN::encode_with_key(pwd, app_key_alpha);
        assert!(enc_alpha.starts_with("$ank$"));

        // 2. Decode dengan app_key_alpha harus 100% cocok
        let dec_alpha = SandiAN::decode_with_key(&enc_alpha, app_key_alpha).expect("Decode Alpha harus sukses");
        assert_eq!(dec_alpha, pwd);

        // 3. Encode dengan app_key_beta harus menghasilkan ciphertext berbeda (Key-Isolation)
        let enc_beta = SandiAN::encode_with_key(pwd, app_key_beta);
        assert_ne!(enc_alpha, enc_beta, "Dua APP_KEY berbeda harus menghasilkan ciphertext berbeda!");

        // 4. Decode enc_alpha menggunakan app_key_beta HARUS GAGAL atau TIDAK menghasilkan kata aslinya!
        if let Ok(dec_wrong) = SandiAN::decode_with_key(&enc_alpha, app_key_beta) {
            assert_ne!(dec_wrong, pwd, "Mendekode dengan kunci salah tidak boleh menghasilkan password asli!");
        }
    }
}

#[test]
fn test_sandi_an_extreme_speed_benchmark() {
    let sample = "KopiPekalonganRahasia#2026!";
    let key = "pakem-production-enterprise-secret-key-2026";
    let iterations = 20_000;

    let start = Instant::now();
    for _ in 0..iterations {
        let enc = SandiAN::encode_with_key(sample, key);
        let _dec = SandiAN::decode_with_key(&enc, key).unwrap();
    }
    let duration = start.elapsed();
    let per_op = duration / (iterations * 2);

    println!(
        "\n⚡ Benchmark Hybrid Sandi A-N (Key-Dependent Masking): {} operasi encode+decode selesai dalam {:?} (~{:?} per operasi)",
        iterations * 2,
        duration,
        per_op
    );

    // Kecepatan per operasi harus sangat cepat (sub-mikrodetik di release, toleransi 50µs di debug build Windows)
    let threshold_micros = if cfg!(debug_assertions) { 50 } else { 10 };
    assert!(
        per_op.as_micros() < threshold_micros,
        "Hybrid Sandi A-N harus beroperasi sangat cepat! (Actual: {:?})", per_op
    );
}
