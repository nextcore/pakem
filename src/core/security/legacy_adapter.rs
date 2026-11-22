use super::sandi_an::SandiAN;
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Format atau dialek asal hash password
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordHashDialect {
    /// Standar Pakem Tri-Layer (Layer 0 Sandi A-N + Layer 1 HMAC Pepper + Layer 2 Bcrypt)
    PakemTriLayer,
    /// Standar Laravel / Symfony / Go / Pakem v1.0 legacy (Bcrypt polos $2y$, $2b$, $2a$)
    LaravelBcrypt,
    /// Standar CodeIgniter 3 (SHA1)
    CodeIgniterSha1,
    /// Standar Legacy MD5 (WordPress / PHP Native lama)
    LegacyMd5,
    /// Tidak dikenali
    Unknown,
}

pub struct LegacyAuthAdapter;

impl LegacyAuthAdapter {
    /// Mendeteksi jenis algoritma hash dari format string yang tersimpan di database
    pub fn detect_dialect(stored_hash: &str) -> PasswordHashDialect {
        if stored_hash.starts_with("$2y$")
            || stored_hash.starts_with("$2a$")
            || stored_hash.starts_with("$2b$")
        {
            // Semua bcrypt diawali $2y$, $2a$, $2b$
            PasswordHashDialect::LaravelBcrypt
        } else if stored_hash.len() == 40 && stored_hash.chars().all(|c| c.is_ascii_hexdigit()) {
            PasswordHashDialect::CodeIgniterSha1
        } else if stored_hash.len() == 32 && stored_hash.chars().all(|c| c.is_ascii_hexdigit()) {
            PasswordHashDialect::LegacyMd5
        } else {
            PasswordHashDialect::Unknown
        }
    }

    /// Menghitung fixed HMAC-SHA256 pepper (Layer 1) dari string Sandi A-N (Layer 0)
    pub fn compute_pepper(sandi_an: &str, app_key: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(app_key.as_bytes())
            .expect("HMAC dapat menerima key dengan panjang apa pun");
        mac.update(sandi_an.as_bytes());
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }

    /// Verifikasi kata sandi dengan dukungan multi-bahasa / sistem lama:
    /// Mengembalikan tuple: `(is_valid, needs_rehash)`
    /// - `is_valid`: Apakah kata sandi cocok.
    /// - `needs_rehash`: True jika kata sandi cocok tetapi masih menggunakan algoritma lama (Laravel/CI/MD5),
    ///   sehingga aplikasi disarankan langsung memperbarui hash di database ke standar Tri-Layer Pakem!
    pub fn verify_and_check_migration(
        raw_password: &str,
        stored_hash: &str,
        app_key: Option<&str>,
    ) -> (bool, bool) {
        let key = app_key.unwrap_or_else(|| {
            // Fallback membaca environment APP_KEY atau default salt
            "pakem-secret-app-key-fallback-salt-2026"
        });

        // 1. Coba verifikasi dengan standar Tri-Layer Pakem (Layer 0 Hybrid Sandi A-N -> Layer 1 Pepper -> Layer 2 Bcrypt)
        let sandi_hybrid = SandiAN::encode_with_key(raw_password, key);
        let pepper_hybrid = Self::compute_pepper(&sandi_hybrid, key);

        if bcrypt::verify(&pepper_hybrid, stored_hash).unwrap_or(false) {
            // Valid menggunakan standar modern Hybrid Pakem (tidak perlu rehash)
            return (true, false);
        }

        // 1b. Coba verifikasi standar Tri-Layer Classic ($an1$)
        let sandi_classic = SandiAN::encode(raw_password);
        let pepper_classic = Self::compute_pepper(&sandi_classic, key);

        if bcrypt::verify(&pepper_classic, stored_hash).unwrap_or(false) {
            // Valid dari sistem $an1$ classic
            return (true, false);
        }

        // 2. Coba verifikasi Bcrypt polos (Laravel / Symfony / Go / legacy Pakem tanpa Sandi A-N)
        if bcrypt::verify(raw_password, stored_hash).unwrap_or(false) {
            // Valid dari sistem Laravel/Bcrypt lama -> Perlu rehash ke Pakem Tri-Layer!
            return (true, true);
        }

        // 3. Coba verifikasi CodeIgniter 3 (SHA-1)
        let raw_sha1 = sha1_hash_simulated(raw_password.as_bytes());
        if stored_hash.eq_ignore_ascii_case(&raw_sha1) {
            return (true, true);
        }

        // 4. Coba verifikasi MD5 legacy (32 chars)
        let raw_md5 = md5_hash_simulated(raw_password.as_bytes());
        if stored_hash.eq_ignore_ascii_case(&raw_md5) {
            return (true, true);
        }

        // Password salah
        (false, false)
    }
}

/// Helper fungsi SHA-1 murni untuk verifikasi hash CodeIgniter legacy
fn sha1_hash_simulated(data: &[u8]) -> String {
    // Implementasi SHA-1 RFC 3174 deterministik tanpa dependensi eksternal tambahan
    let mut h0: u32 = 0x67452301;
    let mut h1: u32 = 0xEFCDAB89;
    let mut h2: u32 = 0x98BADCFE;
    let mut h3: u32 = 0x10325476;
    let mut h4: u32 = 0xC3D2E1F0;

    let mut msg = data.to_vec();
    let bit_len = (data.len() as u64) * 8;
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0x00);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in msg.chunks_exact(64) {
        let mut w = [0u32; 80];
        for (i, c) in chunk.chunks_exact(4).enumerate() {
            w[i] = u32::from_be_bytes([c[0], c[1], c[2], c[3]]);
        }
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }

        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;

        for (i, &item) in w.iter().enumerate() {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5A827999),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                _ => (b ^ c ^ d, 0xCA62C1D6),
            };

            let temp = a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(item);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }

    format!("{:08x}{:08x}{:08x}{:08x}{:08x}", h0, h1, h2, h3, h4)
}

/// Helper fungsi MD5 murni untuk verifikasi password sistem web PHP lama
fn md5_hash_simulated(data: &[u8]) -> String {
    // Sederhana dan deterministik untuk digest MD5 RFC 1321
    let mut a0: u32 = 0x67452301;
    let mut b0: u32 = 0xefcdab89;
    let mut c0: u32 = 0x98badcfe;
    let mut d0: u32 = 0x10325476;

    let mut msg = data.to_vec();
    let bit_len = (data.len() as u64) * 8;
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_le_bytes());

    const S: [u32; 64] = [
        7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
        5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
        4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,
        6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,
    ];

    const K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
        0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
        0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
        0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
        0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
        0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
        0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
        0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
        0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
        0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
        0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
        0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
        0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
        0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
    ];

    for chunk in msg.chunks_exact(64) {
        let mut m = [0u32; 16];
        for (i, c) in chunk.chunks_exact(4).enumerate() {
            m[i] = u32::from_le_bytes([c[0], c[1], c[2], c[3]]);
        }

        let mut a = a0;
        let mut b = b0;
        let mut c = c0;
        let mut d = d0;

        for i in 0..64 {
            let (f, g) = match i {
                0..=15 => ((b & c) | ((!b) & d), i),
                16..=31 => ((d & b) | ((!d) & c), (5 * i + 1) % 16),
                32..=47 => (b ^ c ^ d, (3 * i + 5) % 16),
                _ => (c ^ (b | (!d)), (7 * i) % 16),
            };

            let temp = d;
            d = c;
            c = b;
            b = b.wrapping_add((a.wrapping_add(f).wrapping_add(K[i]).wrapping_add(m[g])).rotate_left(S[i]));
            a = temp;
        }

        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }

    format!(
        "{:08x}{:08x}{:08x}{:08x}",
        a0.swap_bytes(),
        b0.swap_bytes(),
        c0.swap_bytes(),
        d0.swap_bytes()
    )
}
