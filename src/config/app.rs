pub struct AppConfig {
    pub name: String,
    pub env: String,
    pub debug: bool,
}

impl AppConfig {
    pub fn load() -> Self {
        Self {
            name: "Pakem".to_string(),
            env: "development".to_string(),
            debug: true,
        }
    }
}
