#[derive(Clone, Debug)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
}

impl ApiConfig {
    pub fn from_env() -> Self {
        let host = std::env::var("ORBIT_API_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
        let port = std::env::var("ORBIT_API_PORT")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(8080);

        Self { host, port }
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
