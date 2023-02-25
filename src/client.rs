use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION};

const DEFAULT_BASE: &str = "https://manifold.markets/api";

pub enum ManifoldAuthorization {
    ApiKey(String),
    JWT(String),
}

impl Into<HeaderValue> for ManifoldAuthorization {
    fn into(self) -> HeaderValue {
        let st = match self {
            ManifoldAuthorization::ApiKey(key) => format!("Key {}", key),
            ManifoldAuthorization::JWT(token) => format!("Bearer {}", token),
        };

        HeaderValue::from_str(&st).expect("Failure creating authorization header")
    }
}

pub struct ManifoldClient {
    // pub auth: ManifoldAuthorization,
    pub base: String,

    pub http: reqwest::Client,
}

impl ManifoldClient {
    pub fn new(auth: ManifoldAuthorization) -> reqwest::Result<ManifoldClient> {
        Self::new_custom_base(auth, DEFAULT_BASE)
    }

    pub fn new_custom_base(
        auth: ManifoldAuthorization,
        base: &str,
    ) -> reqwest::Result<ManifoldClient> {
        let mut headers = HeaderMap::new();

        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(AUTHORIZATION, auth.into());

        Ok(ManifoldClient {
            // auth,
            base: base.trim_end_matches("/").to_owned(),
            http: reqwest::Client::builder()
                .user_agent("manifold-markets.rs/0.1.0")
                .default_headers(headers)
                .build()?,
        })
    }
}
