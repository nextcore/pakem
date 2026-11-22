use std::collections::HashMap;
use std::sync::Arc;

/// ConfigManager menyimpan semua konfigurasi aplikasi yang dimuat dari .env atau file lain.
pub struct ConfigManager {
    settings: HashMap<String, String>,
}

impl ConfigManager {
    /// Inisialisasi ConfigManager dengan memuat data dari environment variables.
    pub fn new() -> Self {
        let mut settings = HashMap::new();

        // Muat semua environment variables ke dalam HashMap
        for (key, value) in std::env::vars() {
            settings.insert(key, value);
        }

        Self { settings }
    }

    /// Set atau override nilai konfigurasi secara programatik.
    pub fn set(&mut self, key: &str, value: &str) {
        self.settings.insert(key.to_string(), value.to_string());
    }

    /// Ambil nilai konfigurasi berdasarkan key.
    /// Contoh: config.get("APP_NAME")
    pub fn get(&self, key: &str) -> Option<String> {
        self.settings.get(key).cloned()
    }

    /// Ambil nilai konfigurasi dengan fallback default.
    pub fn get_or(&self, key: &str, default: &str) -> String {
        self.settings
            .get(key)
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }

    /// Ambil nilai konfigurasi sebagai integer.
    pub fn get_int(&self, key: &str, default: i32) -> i32 {
        self.get(key)
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(default)
    }

    /// Ambil nilai konfigurasi sebagai boolean.
    pub fn get_bool(&self, key: &str, default: bool) -> bool {
        match self.get(key).as_deref() {
            Some("true") | Some("1") | Some("yes") => true,
            Some("false") | Some("0") | Some("no") => false,
            _ => default,
        }
    }
    /// Membangun DATABASE_URL dari variabel DB_* ala Laravel atau mengambil langsung dari DATABASE_URL.
    pub fn get_db_url(&self) -> String {
        // 1. Jika DATABASE_URL sudah ada di .env secara eksplisit dan valid (memiliki skema), gunakan itu (prioritas).
        if let Some(url) = self.get("DATABASE_URL") {
            let trimmed = url.trim();
            if !trimmed.is_empty()
                && (trimmed.contains("://")
                    || trimmed.starts_with("sqlite:")
                    || trimmed.starts_with("jdbc:")
                    || trimmed.to_ascii_lowercase().starts_with("server="))
            {
                return trimmed.to_string();
            }
        }

        // 2. Format dinamis ala Laravel (DB_CONNECTION, DB_HOST, DB_PORT, DB_DATABASE, DB_USERNAME, DB_PASSWORD)
        let connection = self.get_or("DB_CONNECTION", "sqlite").to_lowercase();

        match connection.as_str() {
            "sqlite" | "sqlite3" => {
                let db = self.get_or("DB_DATABASE", "./database.sqlite");
                if db.starts_with("sqlite:") {
                    db
                } else if db.starts_with("./") || db.starts_with("/") || db.contains(":\\") || db.contains(":/") {
                    format!("sqlite:{}", db)
                } else {
                    format!("sqlite:./{}", db)
                }
            }
            "mysql" | "mariadb" => {
                let host = self.get_or("DB_HOST", "127.0.0.1");
                let port = self.get_or("DB_PORT", "3306");
                let user = self.get_or("DB_USERNAME", "root");
                let pass = self.get_or("DB_PASSWORD", "");
                let db = self.get_or("DB_DATABASE", "pakem");
                let auth = if pass.is_empty() { user } else { format!("{}:{}", user, pass) };
                format!("mysql://{}@{}:{}/{}", auth, host, port, db)
            }
            "postgres" | "postgresql" | "pgsql" | "pg" => {
                let host = self.get_or("DB_HOST", "127.0.0.1");
                let port = self.get_or("DB_PORT", "5432");
                let user = self.get_or("DB_USERNAME", "postgres");
                let pass = self.get_or("DB_PASSWORD", "");
                let db = self.get_or("DB_DATABASE", "pakem");
                let auth = if pass.is_empty() { user } else { format!("{}:{}", user, pass) };
                format!("postgres://{}@{}:{}/{}", auth, host, port, db)
            }
            "mssql" | "sqlserver" | "sqlsrv" => {
                let host = self.get_or("DB_HOST", "127.0.0.1");
                let port = self.get_or("DB_PORT", "1433");
                let user = self.get_or("DB_USERNAME", "sa");
                let pass = self.get_or("DB_PASSWORD", "");
                let db = self.get_or("DB_DATABASE", "pakem");
                let trust = self.get_or("DB_TRUST_CERT", "true");
                format!(
                    "jdbc:sqlserver://{}:{};user={};password={};database={};trustServerCertificate={}",
                    host, port, user, pass, db, trust
                )
            }
            _ => self.get_or("DATABASE_URL", "sqlite:./database.sqlite"),
        }
    }
}

/// Helper untuk memudahkan akses config jika menggunakan Arc.
pub type Config = Arc<ConfigManager>;
