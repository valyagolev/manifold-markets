//! Types that are used in Manifold API
//!
//! Since we expect Manifold API to evolve pretty quickly,
//! instead of hard-coding the schema types, we use wrappers
//! around `serde_json::Value` and provide accessors for
//! the usual fields; it'd be better for now than hard-coding a schema
//! and missing out on the new fields. (We'll change this in the future)
//!
//! Those acessors have "expects" in them to avoid Options/Results
//! everywhere – for the *required* fields that we absolutely expect
//! from the API. This might be a questionable decision, feel free
//! to open an issue if you think it should be changed.
//! (When we stop using `serde_json::Value` we won't need those at all)

use std::collections::HashMap;

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// For binary markets, this is YES or NO. For free response markets, this is the ID of the free response answer. For numeric markets, this is a string representing the target bucket, and an additional value parameter is required which is a number representing the target value. (Bet on numeric markets at your own peril.)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Outcome {
    #[serde(rename = "YES")]
    Yes,
    #[serde(rename = "NO")]
    No,
    FreeResponse(String),
    Numeric(String, f64),
}

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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Group(pub Value);

/// One of BINARY, FREE_RESPONSE, MULTIPLE_CHOICE, or PSEUDO_NUMERIC.
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub enum OutcomeType {
    #[serde(rename = "BINARY")]
    Binary,
    #[serde(rename = "FREE_RESPONSE")]
    FreeResponse,
    #[serde(rename = "MULTIPLE_CHOICE")]
    MultipleChoice,
    #[serde(rename = "PSEUDO_NUMERIC")]
    PseudoNumeric,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct FullMarket(pub Value);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LiteMarket(pub Value);

pub trait Market {
    fn data(&self) -> &Value;

    fn id(&self) -> &str {
        self.data()["id"]
            .as_str()
            .expect("Market.id is not a string")
    }

    fn outcome_type(&self) -> OutcomeType {
        serde_json::from_value(self.data()["outcomeType"].clone())
            .expect("Market.outcomeType is not an OutcomeType")
    }

    fn pool(&self) -> Vec<(Outcome, f64)> {
        let val: HashMap<String, f64> = serde_json::from_value(self.data()["pool"].clone())
            .expect("Market.pool is not a HashMap<String, f64>");

        match self.outcome_type() {
            OutcomeType::Binary => val
                .into_iter()
                .map(|(k, v)| {
                    let outcome = match k.as_str() {
                        "YES" => Outcome::Yes,
                        "NO" => Outcome::No,
                        _ => panic!("OutcomeType::Binary has an invalid outcome"),
                    };
                    (outcome, v)
                })
                .collect(),
            OutcomeType::FreeResponse => val
                .into_iter()
                .map(|(k, v)| (Outcome::FreeResponse(k), v))
                .collect(),
            OutcomeType::MultipleChoice => val
                .into_iter()
                .map(|(k, v)| (Outcome::FreeResponse(k), v))
                .collect(),
            OutcomeType::PseudoNumeric => val
                .into_iter()
                .map(|(k, v)| {
                    let outcome = match k.as_str() {
                        "YES" => Outcome::Yes,
                        "NO" => Outcome::No,
                        _ => panic!("OutcomeType::PseudoNumeric has an invalid outcome"),
                    };
                    (outcome, v)
                })
                .collect(),
        }
    }

    fn close_time(&self) -> Option<DateTime<Utc>> {
        let ts_ms: Option<i64> =
            serde_json::from_value(self.data()["closeTime"].clone()).expect("Invalid closeTime");

        let ts_ms = ts_ms?;

        Some(DateTime::from_utc(
            chrono::NaiveDateTime::from_timestamp_opt(
                ts_ms / 1000,
                (ts_ms % 1000) as u32 * 1_000_000,
            )
            .expect("Market.closeTime is not a valid timestamp"),
            chrono::Utc,
        ))
    }

    fn is_active(&self) -> bool {
        self.close_time().map(|t| t > Utc::now()).unwrap_or(true)
    }
}

impl Market for FullMarket {
    fn data(&self) -> &Value {
        &self.0
    }
}

impl Market for LiteMarket {
    fn data(&self) -> &Value {
        &self.0
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Bet(pub Value);
