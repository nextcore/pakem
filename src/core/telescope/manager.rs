use crate::core::telescope::entry::{EntryType, TelescopeEntry};
use serde_json::Value;
use std::collections::VecDeque;
use std::sync::RwLock;

/// Kapasitas maksimum buffer in-memory untuk entri telescope.
const MAX_ENTRIES: usize = 100;

const SENSITIVE_KEYS: &[&str] = &[
    "password",
    "passwd",
    "secret",
    "token",
    "api_key",
    "apikey",
    "authorization",
    "cookie",
    "credit_card",
    "cvv",
    "private_key",
];

fn is_sensitive_key(key: &str) -> bool {
    let lower = key.to_ascii_lowercase();
    SENSITIVE_KEYS.iter().any(|s| lower.contains(s))
}

/// Redaksi rekursif untuk menyembunyikan field sensitif (password, token, secret, dsb.).
pub fn redact_sensitive_json(val: &mut Value) {
    match val {
        Value::Object(map) => {
            for (k, v) in map.iter_mut() {
                if is_sensitive_key(k) {
                    *v = Value::String("[REDACTED]".to_string());
                } else {
                    redact_sensitive_json(v);
                }
            }
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                redact_sensitive_json(item);
            }
        }
        _ => {}
    }
}

pub struct TelescopeManager {
    entries: RwLock<VecDeque<TelescopeEntry>>,
}

impl TelescopeManager {
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(VecDeque::with_capacity(MAX_ENTRIES)),
        }
    }

    /// Menentukan apakah Telescope aktif.
    /// Default mati di production (`APP_ENV=production|prod`) kecuali `TELESCOPE_ENABLED=true`.
    pub fn is_enabled() -> bool {
        if let Ok(flag) = std::env::var("TELESCOPE_ENABLED") {
            return matches!(flag.to_ascii_lowercase().as_str(), "true" | "1" | "yes");
        }
        let env = std::env::var("APP_ENV").unwrap_or_else(|_| "local".to_string());
        !matches!(env.to_ascii_lowercase().as_str(), "production" | "prod")
    }

    /// Mencatat entri baru ke dalam buffer dengan redaksi otomatis field sensitif.
    pub fn record(&self, entry_type: EntryType, mut content: Value) {
        redact_sensitive_json(&mut content);
        if let Ok(mut entries) = self.entries.write() {
            if entries.len() >= MAX_ENTRIES {
                entries.pop_back(); // Hapus entri tertua
            }

            entries.push_front(TelescopeEntry::new(entry_type, content));
        }
    }

    /// Mengambil semua entri.
    pub fn all(&self) -> Vec<TelescopeEntry> {
        self.entries
            .read()
            .map(|entries| entries.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Mengambil entri berdasarkan tipe.
    pub fn get_by_type(&self, entry_type: EntryType) -> Vec<TelescopeEntry> {
        self.entries
            .read()
            .map(|entries| {
                entries
                    .iter()
                    .filter(|e| {
                        std::mem::discriminant(&e.entry_type) == std::mem::discriminant(&entry_type)
                    })
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Mencatat HTTP Request.
    pub fn record_request(&self, content: crate::core::telescope::entry::RequestContent) {
        self.record(
            EntryType::Request,
            serde_json::to_value(content).unwrap_or(Value::Null),
        );
    }

    /// Mencatat Event.
    pub fn record_event(&self, name: &str, payload: &str) {
        let content = crate::core::telescope::entry::EventContent {
            name: name.to_string(),
            payload: payload.to_string(),
        };
        self.record(
            EntryType::Event,
            serde_json::to_value(content).unwrap_or(Value::Null),
        );
    }
}
