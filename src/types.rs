//! Types that are used in Manifold API
//!
//! Since we expect Manifold API to evolve pretty quickly,
//! instead of hard-coding the schema types, we use wrappers
//! around `serde_json::Value` and provide accessors for
//! the usual fields. (This should change in the future.)
//!
//! Those acessors have "expects" in them to avoid Options/Results
//! everywhere – for the *required* fields that we absolutely expect
//! from the API. This might be a questionable decision, feel free
//! to open an issue if you think it should be changed.
//! (When we stop using `serde_json::Value` we won't need those at all)

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct User(pub Value);

impl User {
    pub fn avatar_url(&self) -> Option<&str> {
        self.0["avatarUrl"].as_str()
    }
    pub fn balance(&self) -> f64 {
        self.0["balance"]
            .as_f64()
            .expect("User.balance is not a number")
    }
    pub fn created_time(&self) -> i64 {
        self.0["createdTime"]
            .as_i64()
            .expect("User.createdTime is not a number")
    }
    pub fn id(&self) -> &str {
        self.0["id"].as_str().expect("User.id is not a string")
    }
    pub fn name(&self) -> &str {
        self.0["name"].as_str().expect("User.name is not a string")
    }
    pub fn total_deposits(&self) -> i64 {
        self.0["totalDeposits"]
            .as_i64()
            .expect("User.totalDeposits is not a number")
    }
    pub fn url(&self) -> &str {
        self.0["url"].as_str().expect("User.url is not a string")
    }
    pub fn username(&self) -> &str {
        self.0["username"]
            .as_str()
            .expect("User.username is not a string")
    }
    pub fn profit_cached(&self) -> ProfitCached {
        serde_json::from_value(self.0["profitCached"].clone())
            .expect("User.profitCached is not a ProfitCached")
    }
}

/// Struct from the User API
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProfitCached {
    #[serde(rename = "allTime")]
    all_time: f64,
    daily: f64,
    monthly: f64,
    weekly: f64,
}
