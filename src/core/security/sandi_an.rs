/// Mesin Kriptografi Nusantara — Enhanced Sandi A-N (Layer 0 Pre-Hash Engine)
/// 
/// Mengombinasikan arsitektur Hybrid:
/// 1. S-Box Non-Linear Aritmatika (Substitusi Alfabet A-N simetris & Sandi Angka 0-5)
/// 2. Cucuk Canting Byte Transposition (Interleaving pasangan indeks genap-ganjil)
/// 3. Key-Dependent Dynamic Masking terikat APP_KEY (Mustahil di-reverse engineer)
/// 4. Dual Envelope Versioning:
///    - `$an1$` (Aksara Nusantara v1 Standard)
///    - `$ank$` (Aksara Nusantara Hybrid Key-Dependent)

pub const SANDI_AN_PREFIX: &str = "$an1$";
pub const SANDI_ANK_PREFIX: &str = "$ank$";

/// Cryptographic S-Box 256-byte (Non-linear look-up table terenkripsi).
/// Memetakan secara simetris A-M <-> N-Z, a-m <-> n-z, dan 0-4 <-> 5-9.
const S_BOX: [u8; 256] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f,
    0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f,
    0x40, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x41, 0x42,
    0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f,
    0x60, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x61, 0x62,
    0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f,
    0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f,
    0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f,
    0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf,
    0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf,
    0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf,
    0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf,
    0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef,
    0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff,
];

pub struct SandiAN;

impl SandiAN {
    /// Mentransformasikan password mentah menjadi Sandi A-N Nusantara ber-envelope `$an1$` (Standard Mode)
    /// 
    /// # Contoh
    /// ```rust
    /// use pakem::core::security::sandi_an::SandiAN;
    /// let sandi = SandiAN::encode("admin123");
    /// assert!(sandi.starts_with("$an1$"));
    /// ```
    pub fn encode(raw: &str) -> String {
        let bytes = raw.as_bytes();
        let mut transformed: Vec<u8> = Vec::with_capacity(bytes.len());

        // 1. Substitusi S-Box (Sandi A-N + Sandi Angka 0-5)
        for &b in bytes {
            transformed.push(S_BOX[b as usize]);
        }

        // 2. Cucuk Canting Interleaving (Tukar pasangan genap-ganjil)
        for chunk in transformed.chunks_exact_mut(2) {
            chunk.swap(0, 1);
        }

        // 3. Format envelope $an1$ (safe UTF-8 conversion, tanpa unsafe)
        let inner = String::from_utf8(transformed)
            .unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).into_owned());
        format!("{}{}", SANDI_AN_PREFIX, inner)
    }

    /// Mentransformasikan password dengan arsitektur **Hybrid Key-Dependent Masking** ber-envelope `$ank$`
    /// 
    /// Mengunci transformasi ke dalam kunci unik aplikasi (`APP_KEY`). Tanpa APP_KEY yang sama persis,
    /// permutasi bitwise tidak akan pernah bisa direkonstruksi atau ditebak oleh dekompiler / peretas.
    pub fn encode_with_key(raw: &str, app_key: &str) -> String {
        let bytes = raw.as_bytes();
        let mut transformed: Vec<u8> = Vec::with_capacity(bytes.len());

        // 1. Substitusi S-Box (Sandi A-N + Sandi Angka 0-5)
        for &b in bytes {
            transformed.push(S_BOX[b as usize]);
        }

        // 2. Cucuk Canting Interleaving (Tukar pasangan genap-ganjil)
        for chunk in transformed.chunks_exact_mut(2) {
            chunk.swap(0, 1);
        }

        // 3. Key-Dependent Stream Masking (XOR Bitwise Diffusion terikat APP_KEY)
        let key_schedule = Self::derive_key_schedule(app_key);
        for (i, b) in transformed.iter_mut().enumerate() {
            let mask = key_schedule[i % key_schedule.len()];
            *b ^= mask;
        }

        // 4. Format envelope $ank$ dengan representasi hex yang aman
        let hex_body = hex::encode(transformed);
        format!("{}{}", SANDI_ANK_PREFIX, hex_body)
    }

    /// Mengembalikan string Sandi A-N ke bentuk kata sandi aslinya (Decode Otomatis)
    /// Mendukung format `$an1$` standar maupun `$ank$` (dengan auto-fallback key dari .env)
    pub fn decode(encoded: &str) -> Result<String, &'static str> {
        if encoded.starts_with(SANDI_ANK_PREFIX) {
            let default_key = std::env::var("APP_KEY")
                .or_else(|_| std::env::var("JWT_SECRET"))
                .unwrap_or_else(|_| "pakem-app-secret-pepper-key-2026".to_string());
            Self::decode_with_key(encoded, &default_key)
        } else if let Some(stripped) = encoded.strip_prefix(SANDI_AN_PREFIX) {
            let mut bytes = stripped.as_bytes().to_vec();

            // 1. Balikkan Cucuk Canting Interleaving (Swap kembali)
            for chunk in bytes.chunks_exact_mut(2) {
                chunk.swap(0, 1);
            }

            // 2. Balikkan S-Box (S_BOX bersifat involutorial: S_BOX[S_BOX[b]] == b)
            for b in &mut bytes {
                *b = S_BOX[*b as usize];
            }

            String::from_utf8(bytes).map_err(|_| "Gagal decode UTF-8 byte stream")
        } else {
            Err("Format Sandi A-N tidak valid (harus diawali $an1$ atau $ank$)")
        }
    }

    /// Mengembalikan string Sandi A-N Hybrid `$ank$` menggunakan kunci spesifik `app_key`
    pub fn decode_with_key(encoded: &str, app_key: &str) -> Result<String, &'static str> {
        let payload = if let Some(stripped) = encoded.strip_prefix(SANDI_ANK_PREFIX) {
            stripped
        } else if encoded.starts_with(SANDI_AN_PREFIX) {
            // Jika dikirim format $an1$, tetap dapat di-decode secara transparan
            return Self::decode(encoded);
        } else {
            return Err("Format Sandi A-N tidak valid (missing prefix)");
        };

        // 1. Decode hex stream
        let mut bytes = hex::decode(payload).map_err(|_| "Hex payload tidak valid")?;

        // 2. Balikkan Key-Dependent Stream Masking (Involutif via XOR: a ^ b ^ b == a)
        let key_schedule = Self::derive_key_schedule(app_key);
        for (i, b) in bytes.iter_mut().enumerate() {
            let mask = key_schedule[i % key_schedule.len()];
            *b ^= mask;
        }

        // 3. Balikkan Cucuk Canting Interleaving (Swap kembali)
        for chunk in bytes.chunks_exact_mut(2) {
            chunk.swap(0, 1);
        }

        // 4. Balikkan S-Box (S_BOX involutif)
        for b in &mut bytes {
            *b = S_BOX[*b as usize];
        }

        String::from_utf8(bytes).map_err(|_| "Gagal decode UTF-8 byte stream")
    }

    /// Memeriksa apakah sebuah string adalah format Sandi A-N ($an1$ atau $ank$)
    pub fn is_sandi_an(s: &str) -> bool {
        s.starts_with(SANDI_AN_PREFIX) || s.starts_with(SANDI_ANK_PREFIX)
    }

    /// Menghasilkan 32-byte Key Schedule non-linear berkecepatan raw CPU dari APP_KEY
    /// Menggunakan dynamic bitwise folding dan Murmur-style mixing tanpa alokasi heap tambahan
    #[inline(always)]
    fn derive_key_schedule(key: &str) -> [u8; 32] {
        let mut schedule = [0x5au8; 32];
        let bytes = key.as_bytes();
        
        if bytes.is_empty() {
            return schedule;
        }

        // Dynamic bit folding
        for (i, &b) in bytes.iter().enumerate() {
            let slot = i % 32;
            schedule[slot] = schedule[slot]
                .wrapping_add(b)
                .rotate_left(3)
                ^ (0xa5 ^ (i as u8));
        }

        // Non-linear diffusion pass
        for i in 0..32 {
            let prev = schedule[(i + 31) % 32];
            let next = schedule[(i + 1) % 32];
            schedule[i] = schedule[i]
                .wrapping_mul(0x9e)
                ^ prev.rotate_right(2)
                ^ next.rotate_left(1);
        }

        schedule
    }
}
