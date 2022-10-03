use witwiki_common::dotenv::dotenv;

pub struct Config {
    pub is_production: bool,
    pub post_change_require_change_reason: bool,
    pub rust_log: String,
    pub is_cors_enabled: bool,
    pub server_port: i64,
}

pub fn get_config() -> Config {
    let env_load_msg = dotenv().map_or_else(
        |e| eprintln!("no .env file loaded: {}", e.to_string()),
        |_| (),
    );
    let mut settings_filename = std::env::current_dir().expect("expected working dir");
    settings_filename.push(".witwiki.json");

    let mut settings = config::Config::builder();
    if std::path::Path::exists(&settings_filename) {
        settings = settings.add_source(config::File::with_name(
            settings_filename.as_path().to_str().unwrap(),
        ));
    }
    let raw = settings
        .add_source(config::Environment::with_prefix("WIT"))
        .build()
        .unwrap();

    let post_change_require_change_reason = raw
        .get_bool("post_change_require_change_reason")
        .unwrap_or(true);

    let is_production = match raw.get_string("profile") {
        Ok(v) => v == "production",
        Err(e) => {
            eprintln!(
                "WIT_PROFILE not set. assuming production: {}",
                e.to_string()
            );
            true
        }
    };
    Config {
        is_production,
        post_change_require_change_reason,
        rust_log: std::env::var("RUST_LOG")
            .unwrap_or_else(|_| "witwiki=debug,tower_http=debug".into()),
        is_cors_enabled: raw.get_bool("ENABLE_CORS").unwrap_or(false),
        server_port: raw.get_int("PORT").unwrap_or(9999),
    }
}
