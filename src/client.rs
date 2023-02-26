use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION};
use serde_json::{json, Value};

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
    pub async fn get_group_markets(&self, id: &str) -> Result<Vec<LiteMarket>> {
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
    ) -> Result<Vec<LiteMarket>> {
        let mut req = self.http_get("/markets");

        if let Some(limit) = limit {
            req = req.query(&[("limit", limit.to_string())]);
        }

        if let Some(before) = before {
            req = req.query(&[("before", before)]);
        }

        Ok(req.send().await?.json().await?)
    }

    /// GET /v0/market/[marketId]
    /// Gets information about a single market by ID. Includes answers, but not bets and comments. Use /bets or /comments with a market ID to retrieve bets or comments.

    /// Requires no authorization.
    pub async fn get_market(&self, market_id: &str) -> Result<FullMarket> {
        Ok(self
            .http_get(&format!("/market/{}", market_id))
            .send()
            .await?
            .json()
            .await?)
    }

    /// GET /v0/slug/[marketSlug]
    /// Gets information about a single market by slug (the portion of the URL path after the username).
    ///
    /// Requires no authorization.
    pub async fn get_market_by_slug(&self, market_slug: &str) -> Result<FullMarket> {
        Ok(self
            .http_get(&format!("/slug/{}", market_slug))
            .send()
            .await?
            .json()
            .await?)
    }

    /*
                    GET /v0/users
                    Lists all users, ordered by creation date descending.

                    Parameters:

                    limit: Optional. How many users to return. The maximum and the default are 1000.
                    before: Optional. The ID of the user before which the list will start. For example, if you ask for the most recent 10 users, and then perform a second query for 10 more users with before=[the id of the 10th user], you will get users 11 through 20.
                    Requires no authorization.

                    POST /v0/bet
                    Places a new bet on behalf of the authorized user.

                    Parameters:

                    amount: Required. The amount to bet, in mana, before fees.
                    contractId: Required. The ID of the contract (market) to bet on.
                    outcome: Required. The outcome to bet on. For binary markets, this is YES or NO. For free response markets, this is the ID of the free response answer. For numeric markets, this is a string representing the target bucket, and an additional value parameter is required which is a number representing the target value. (Bet on numeric markets at your own peril.)
                    limitProb: Optional. A number between 0.001 and 0.999 inclusive representing the limit probability for your bet (i.e. 0.1% to 99.9% — multiply by 100 for the probability percentage). The bet will execute immediately in the direction of outcome, but not beyond this specified limit. If not all the bet is filled, the bet will remain as an open offer that can later be matched against an opposite direction bet.
                    For example, if the current market probability is 50%:
                    A M$10 bet on YES with limitProb=0.4 would not be filled until the market probability moves down to 40% and someone bets M$15 of NO to match your bet odds.
                    A M$100 bet on YES with limitProb=0.6 would fill partially or completely depending on current unfilled limit bets and the AMM's liquidity. Any remaining portion of the bet not filled would remain to be matched against in the future.
                    An unfilled limit order bet can be cancelled using the cancel API.


                    POST /v0/bet/cancel/[id]
                    Cancel the limit order of a bet with the specified id. If the bet was unfilled, it will be cancelled so that no other bets will match with it. This action is irreversible.

                    POST /v0/market
                    Creates a new market on behalf of the authorized user.

                    Note: this costs mana. If you have insufficient mana, this call will return an error. The cost to create each type of market is:

                    Market Type	Creation Cost
                    BINARY	M$50
                    PSEUDO_NUMERIC	M$250
                    FREE_RESPONSE	M$100
                    MULTIPLE_CHOICE	M$100
                    Parameters:

                    outcomeType: Required. One of BINARY, FREE_RESPONSE, MULTIPLE_CHOICE, or PSEUDO_NUMERIC.
                    question: Required. The headline question for the market.
                    description: Optional. A long description describing the rules for the market.
                    Note: string descriptions do not turn into links, mentions, formatted text. You may instead use descriptionMarkdown or descriptionHtml for rich text formatting.
                    closeTime: Optional. The time at which the market will close, represented as milliseconds since the epoch. Defaults to 7 days from now.
                    visibility: Optional. One of public (default) or unlisted. Controls whether the market can be shown on homepage and in search results.
                    groupId: Optional. A group to create this market under.
                    For binary markets, you must also provide:

                    initialProb: An initial probability for the market, between 1 and 99.
                    For numeric markets, you must also provide:

                    min: The minimum value that the market may resolve to.
                    max: The maximum value that the market may resolve to.
                    isLogScale: If true, your numeric market will increase exponentially from min to max.
                    initialValue: An initial value for the market, between min and max, exclusive.
                    For multiple choice markets, you must also provide:

                    answers: An array of strings, each of which will be a valid answer for the market.


                POST /v0/market/[marketId]/add-liquidity
                Adds a specified amount of liquidity into the market.

                amount: Required. The amount of liquidity to add, in M$.
                POST /v0/market/[marketId]/close
                Closes a market on behalf of the authorized user.

                closeTime: Optional. Milliseconds since the epoch to close the market at. If not provided, the market will be closed immediately. Cannot provide close time in the past.
                POST /v0/market/[marketId]/resolve
                Resolves a market on behalf of the authorized user.

                Parameters:

                For binary markets:

                outcome: Required. One of YES, NO, MKT, or CANCEL.
                probabilityInt: Optional. The probability to use for MKT resolution.
                For free response or multiple choice markets:

                outcome: Required. One of MKT, CANCEL, or a number indicating the answer index.
                resolutions: An array of { answer, pct } objects to use as the weights for resolving in favor of multiple free response options. Can only be set with MKT outcome. Note that the total weights must add to 100.
                For numeric markets:

                outcome: Required. One of CANCEL, or a number indicating the selected numeric bucket ID.
                value: The value that the market resolves to.
                probabilityInt: Required if value is present. Should be equal to
                If log scale: log10(value - min + 1) / log10(max - min + 1)
                Otherwise: (value - min) / (max - min)

            POST /v0/market/[marketId]/sell
            Sells some quantity of shares in a binary market on behalf of the authorized user.

            Parameters:

            outcome: Optional. One of YES, or NO. If you leave it off, and you only own one kind of shares, you will sell that kind of shares.
            shares: Optional. The amount of shares to sell of the outcome given above. If not provided, all the shares you own will be sold.

        POST /v0/comment
        Creates a comment in the specified market. Only supports top-level comments for now.

        Parameters:

        contractId: Required. The ID of the market to comment on.
        content: The comment to post, formatted as TipTap json, OR
        html: The comment to post, formatted as an HTML string, OR
        markdown: The comment to post, formatted as a markdown string.


        GET /v0/comments
        Gets a list of comments for a contract, ordered by creation date descending.

        Parameters:

        contractId: Optional. Which contract to read comments for. Either an ID or slug must be specified.
        contractSlug: Optional.
        Requires no authorization.


    GET /v0/bets
    Gets a list of bets, ordered by creation date descending.

    Parameters:

    userId: Optional. If set, the response will include only bets created by this user.
    username: Optional. If set, the response will include only bets created by this user.
    contractId: Optional. If set, the response will only include bets on this contract.
    contractSlug: Optional. If set, the response will only include bets on this contract.
    limit: Optional. How many bets to return. The maximum and the default are 1000.
    before: Optional. The ID of the bet before which the list will start. For example, if you ask for the most recent 10 bets, and then perform a second query for 10 more bets with before=[the id of the 10th bet], you will get bets 11 through 20.
    Requires no authorization.
                     */

    /// GET /v0/users
    /// Gets a list of users, ordered by creation date descending.
    /// Parameters:
    ///  limit: Optional. How many users to return. The maximum and the default are 1000.
    /// before: Optional. The ID of the user before which the list will start. For example, if you ask for the most recent 10 users, and then perform a second query for 10 more users with before=[the id of the 10th user], you will get users 11 through 20.
    /// Requires no authorization.
    pub async fn get_users(&self, limit: Option<u32>, before: Option<u32>) -> Result<Vec<User>> {
        let mut query = vec![];
        if let Some(limit) = limit {
            query.push(("limit", limit.to_string()));
        }
        if let Some(before) = before {
            query.push(("before", before.to_string()));
        }

        let response = self.http_get("/users").query(&query).send().await?;
        Ok(response.json().await?)
    }

    /// POST /v0/bet
    /// Creates a new bet on behalf of the authorized user.
    /// Parameters:
    /// amount: Required. The amount to bet, in mana, before fees.
    /// contractId: Required. The ID of the contract to bet on.
    /// outcome: Required. The outcome to bet on. For binary markets, this is YES or NO. For free response markets, this is the ID of the free response answer. For numeric markets, this is a string representing the target bucket, and an additional value parameter is required which is a number representing the target value. (Bet on numeric markets at your own peril.)
    /// limitProb: Optional. A number between 0.001 and 0.999 inclusive representing the limit probability for your bet (i.e. 0.1% to 99.9% — multiply by 100 for the probability percentage). The bet will execute immediately in the direction of outcome, but not beyond this specified limit. If not all the bet is filled, the bet will remain as an open offer that can later be matched against an opposite direction bet.
    /// For example, if the current market probability is 50%:
    /// A M$10 bet on YES with limitProb=0.4 would not be filled until the market probability moves down to 40% and someone bets M$15 of NO to match your bet odds.
    /// A M$100 bet on YES with limitProb=0.6 would fill partially or completely depending on current unfilled limit bets and the AMM's liquidity. Any remaining portion of the bet not filled would remain to be matched against in the future.
    /// An unfilled limit order bet can be cancelled using the cancel API.
    pub async fn post_bet(
        &self,
        amount: u64,
        contract_id: &str,
        outcome: Outcome,
        limit_prob: Option<f64>,
    ) -> Result<Bet> {
        let mut body = json!(
            {
                "amount": amount,
                "contractId": contract_id,
            }
        );

        {
            let body = body.as_object_mut().unwrap();

            let outcome = match outcome {
                Outcome::Yes => json!("YES"),
                Outcome::No => json!("NO"),
                Outcome::FreeResponse(id) => json!(id),
                Outcome::Numeric(bucket, value) => {
                    body.insert("value".to_owned(), json!(value));
                    json!(bucket)
                }
            };

            body.insert("outcome".to_owned(), outcome);

            if let Some(limit_prob) = limit_prob {
                body.insert("limitProb".to_owned(), json!(limit_prob));
            }
        }

        let response = self.http_post("/bet").json(&body).send().await?;
        Ok(response.json().await?)
    }
}
