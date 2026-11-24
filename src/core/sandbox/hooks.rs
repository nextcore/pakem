use serde::{Deserialize, Serialize};

/// Tipe event siklus hidup (lifecycle hook) yang didukung oleh plugin WASM
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PluginHook {
    /// Berjalan saat request HTTP diterima oleh host sebelum controller
    OnRequest,
    /// Berjalan sebelum entitas model disimpan ke database
    BeforeSave,
    /// Berjalan setelah response HTTP dihasilkan sebelum dikirim ke client
    AfterResponse,
    /// Hook kustom dengan nama fungsi tertentu
    Custom(String),
}

impl PluginHook {
    /// Mengembalikan nama fungsi export WebAssembly yang dicari
    pub fn export_name(&self) -> String {
        match self {
            PluginHook::OnRequest => "pakem_on_request".to_string(),
            PluginHook::BeforeSave => "pakem_before_save".to_string(),
            PluginHook::AfterResponse => "pakem_after_response".to_string(),
            PluginHook::Custom(name) => format!("pakem_{}", name),
        }
    }
}
