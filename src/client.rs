use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION};

use crate::error::Result;
use crate::types::*;

const DEFAULT_BASE: &str = "https://manifold.markets/api/v0";

#[derive(Clone, Debug)]
pub enum ManifoldAuthorization {
    ApiKey(String),
    JWT(String),
    NoAuthorization,
}

impl Into<Option<HeaderValue>> for ManifoldAuthorization {
    fn into(self) -> Option<HeaderValue> {
        let st = match self {
            ManifoldAuthorization::ApiKey(key) => format!("Key {}", key),
            ManifoldAuthorization::JWT(token) => format!("Bearer {}", token),
            ManifoldAuthorization::NoAuthorization => return Option::None,
        };

        Some(HeaderValue::from_str(&st).expect("Failure creating authorization header"))
    }
}

#[derive(Clone, Debug)]
pub struct ManifoldClient {
    // pub auth: ManifoldAuthorization,
    pub base: String,

    pub http: reqwest::Client,
}

impl ManifoldClient {
    /// Create a new client using an API key
    ///
    /// This is the most usual way to authenticate with Manifold.
    /// The key can be found in your account settings: https://manifold.markets/profile
    pub fn from_api_key(key: &str) -> Result<ManifoldClient> {
        Self::new(ManifoldAuthorization::ApiKey(key.to_owned()))
    }

    pub fn new(auth: ManifoldAuthorization) -> Result<ManifoldClient> {
        Self::new_custom_base(auth, DEFAULT_BASE)
    }

    pub fn new_custom_base(auth: ManifoldAuthorization, base: &str) -> Result<ManifoldClient> {
        let mut headers = HeaderMap::new();

        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        if let Some(header) = auth.into() {
            headers.insert(AUTHORIZATION, header);
        }

        Ok(ManifoldClient {
            // auth,
            base: base.trim_end_matches("/").to_owned(),
            http: reqwest::Client::builder()
                .user_agent("manifold-markets.rs/0.1.0")
                .default_headers(headers)
                .build()?,
        })
    }

    pub fn http_get(&self, path: &str) -> reqwest::RequestBuilder {
        self.http.get(&format!("{}{}", self.base, path))
    }

    pub fn http_post(&self, path: &str) -> reqwest::RequestBuilder {
        self.http.post(&format!("{}{}", self.base, path))
    }

    /// GET /v0/user/[username]
    /// Gets a user by their username. Remember that usernames may change.
    /// Requires no authorization.
    pub async fn get_user(&self, username: &str) -> Result<User> {
        Ok(self
            .http_get(&format!("/user/{}", username))
            .send()
            .await?
            .json()
            .await?)
    }

    /// GET /v0/user/by-id/[id]
    /// Gets a user by their unique ID. Many other API endpoints return this as the userId.
    /// Requires no authorization.
    pub async fn get_user_by_id(&self, id: &str) -> Result<User> {
        Ok(self
            .http_get(&format!("/user/by-id/{}", id))
            .send()
            .await?
            .json()
            .await?)
    }

    /// GET /v0/me
    /// Gets the currently authenticated user.
    pub async fn get_me(&self) -> Result<User> {
        Ok(self.http_get("/me").send().await?.json().await?)
    }

    /// GET /v0/groups
    /// Gets all groups, in no particular order.
    ///
    /// Parameters:
    ///
    /// availableToUserId: Optional. if specified, only groups that the user can join and groups they've already joined will be returned.
    ///
    /// Requires no authorization.
    pub async fn get_groups(&self, available_to_user_id: Option<&str>) -> Result<Vec<Group>> {
        let mut req = self.http_get("/groups");

        if let Some(id) = available_to_user_id {
            req = req.query(&[("availableToUserId", id)]);
        }

        Ok(req.send().await?.json().await?)
    }

    /// GET /v0/group/[slug]
    /// Gets a group by its slug.
    /// Requires no authorization. Note: group is singular in the URL.
    pub async fn get_group(&self, slug: &str) -> Result<Group> {
        Ok(self
            .http_get(&format!("/group/{}", slug))
            .send()
            .await?
            .json()
            .await?)
    }

    /// GET /v0/group/by-id/[id]
    /// Gets a group by its unique ID.
    /// Requires no authorization. Note: group is singular in the URL.
    pub async fn get_group_by_id(&self, id: &str) -> Result<Group> {
        Ok(self
            .http_get(&format!("/group/by-id/{}", id))
            .send()
            .await?
            .json()
            .await?)
    }

    /// GET /v0/group/by-id/[id]/markets
    /// Gets a group's markets by its unique ID.
    /// Requires no authorization. Note: group is singular in the URL.
    pub async fn get_group_markets(&self, id: &str) -> Result<Vec<Market>> {
        Ok(self
            .http_get(&format!("/group/by-id/{}/markets", id))
            .send()
            .await?
            .json()
            .await?)
    }

    /// GET /v0/markets
    /// Lists all markets, ordered by creation date descending.
    ///
    /// Parameters:
    ///
    /// limit: Optional. How many markets to return. The maximum is 1000 and the default is 500.
    /// before: Optional. The ID of the market before which the list will start. For example, if you ask for the most recent 10 markets, and then perform a second query for 10 more markets with before=[the id of the 10th market], you will get markets 11 through 20.
    ///
    /// Requires no authorization.
    pub async fn get_markets(
        &self,
        limit: Option<u32>,
        before: Option<&str>,
    ) -> Result<Vec<Market>> {
        let mut req = self.http_get("/markets");

        if let Some(limit) = limit {
            req = req.query(&[("limit", limit.to_string())]);
        }

        if let Some(before) = before {
            req = req.query(&[("before", before)]);
        }

        Ok(req.send().await?.json().await?)
    }
}
