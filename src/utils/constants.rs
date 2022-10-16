pub mod storage_key {
    pub const AUTH_STORAGE_KEY: &str = "sneu@AUTH_STORAGE_KEY";
}

pub mod api_url {
    pub const API_ROOT: &str = "http://localhost:8000/api/v1";
    pub fn with_api_root(suffix: &str) -> String {
        format!("{API_ROOT}{suffix}")
    }
}
