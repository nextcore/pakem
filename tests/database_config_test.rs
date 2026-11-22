use pakem::core::config::ConfigManager;

#[test]
fn test_sqlite_laravel_style_config() {
    let mut config = ConfigManager::new();
    config.set("DB_CONNECTION", "sqlite");
    config.set("DB_DATABASE", "my_app.sqlite");

    let url = config.get_db_url();
    assert_eq!(url, "sqlite:./my_app.sqlite");
}

#[test]
fn test_mysql_laravel_style_config() {
    let mut config = ConfigManager::new();
    config.set("DB_CONNECTION", "mysql");
    config.set("DB_HOST", "127.0.0.1");
    config.set("DB_PORT", "3306");
    config.set("DB_DATABASE", "toko_online");
    config.set("DB_USERNAME", "root");
    config.set("DB_PASSWORD", "secret123");

    let url = config.get_db_url();
    assert_eq!(url, "mysql://root:secret123@127.0.0.1:3306/toko_online");
}

#[test]
fn test_pgsql_laravel_style_config() {
    let mut config = ConfigManager::new();
    config.set("DB_CONNECTION", "pgsql");
    config.set("DB_HOST", "192.168.1.100");
    config.set("DB_PORT", "5432");
    config.set("DB_DATABASE", "enterprise_db");
    config.set("DB_USERNAME", "app_user");
    config.set("DB_PASSWORD", "SuperSecurePassword!");

    let url = config.get_db_url();
    assert_eq!(url, "postgres://app_user:SuperSecurePassword!@192.168.1.100:5432/enterprise_db");
}

#[test]
fn test_mssql_laravel_style_config() {
    let mut config = ConfigManager::new();
    config.set("DB_CONNECTION", "sqlsrv");
    config.set("DB_HOST", "10.0.0.5");
    config.set("DB_PORT", "1433");
    config.set("DB_DATABASE", "SIMRS_DB");
    config.set("DB_USERNAME", "sa");
    config.set("DB_PASSWORD", "MssqlPass#1");

    let url = config.get_db_url();
    assert_eq!(
        url,
        "jdbc:sqlserver://10.0.0.5:1433;user=sa;password=MssqlPass#1;database=SIMRS_DB;trustServerCertificate=true"
    );
}

#[test]
fn test_explicit_database_url_priority() {
    let mut config = ConfigManager::new();
    config.set("DB_CONNECTION", "mysql");
    config.set("DB_HOST", "127.0.0.1");
    // Explicit DATABASE_URL takes priority
    config.set("DATABASE_URL", "postgres://custom_user:pass@remote:5432/db");

    let url = config.get_db_url();
    assert_eq!(url, "postgres://custom_user:pass@remote:5432/db");
}

#[test]
fn test_invalid_database_url_fallback() {
    let mut config = ConfigManager::new();
    config.set("DB_CONNECTION", "mysql");
    config.set("DB_HOST", "127.0.0.1");
    config.set("DB_PORT", "3306");
    config.set("DB_DATABASE", "toko");
    config.set("DB_USERNAME", "root");
    config.set("DB_PASSWORD", "secret");

    // Jika user salah ketik DATABASE_URL=mysql (tanpa scheme ://)
    config.set("DATABASE_URL", "mysql");

    let url = config.get_db_url();
    // Harus otomatis fallback merakit URL dari DB_*
    assert_eq!(url, "mysql://root:secret@127.0.0.1:3306/toko");
}
