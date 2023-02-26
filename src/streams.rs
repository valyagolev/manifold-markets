use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::{stream, Stream, StreamExt, TryStreamExt};

use crate::error::{ManifoldError, Result};
use reqwest::Error;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{types::*, ManifoldClient};

impl ManifoldClient {
    pub fn stream_paginated<T: DeserializeOwned>(
        &self,
        path: String,
        params: Vec<(String, String)>,
    ) -> impl Stream<Item = Result<T>> + '_ {
        stream::try_unfold(None, move |before| {
            let params = params.clone();
            let path = path.clone();

            async move {
                let params = match before {
                    Some(before) => {
                        let mut params = params.clone();
                        params.push(("before".to_owned(), before));
                        params
                    }
                    None => params.clone(),
                };

                let result = self
                    .http_get(&path)
                    .query(&params)
                    .send()
                    .await?
                    .error_for_status()?
                    .json::<Value>()
                    .await?;

                let result = result.as_array().ok_or_else(|| {
                    ManifoldError::SchemaError(
                        "Streaming response returned not an array?".to_owned(),
                        Some(result.clone()),
                    )
                })?;

                let Some(last) = result.last() else {
                return Result::Ok(None)
            };

                let last_id = last["id"]
                    .as_str()
                    .ok_or_else(|| {
                        ManifoldError::SchemaError(
                            "Not a string id?".to_owned(),
                            Some(last["id"].clone()),
                        )
                    })?
                    .to_owned();

                let result = result
                    .into_iter()
                    .map(|v| serde_json::from_value(v.clone()))
                    .try_collect::<Vec<T>>()?;

                Ok(Some((result, Some(last_id))))
            }
        })
        .map_ok(|v| stream::iter(v.into_iter().map(Ok)))
        .try_flatten()
    }

    pub fn stream_markets(&self) -> impl Stream<Item = Result<LiteMarket>> + '_ {
        self.stream_paginated("/markets".to_owned(), vec![])
    }

    pub fn stream_users(&self) -> impl Stream<Item = Result<User>> + '_ {
        self.stream_paginated("/users".to_owned(), vec![])
    }

    pub fn stream_bets(
        &self,
        user_id: Option<&str>,
        username: Option<&str>,
        contract_id: Option<&str>,
        contract_slug: Option<&str>,
    ) -> impl Stream<Item = Result<Bet>> + '_ {
        let mut params = vec![];

        if let Some(user_id) = user_id {
            params.push(("userId".to_owned(), user_id.to_owned()));
        }

        if let Some(username) = username {
            params.push(("username".to_owned(), username.to_owned()));
        }

        if let Some(contract_id) = contract_id {
            params.push(("contractId".to_owned(), contract_id.to_owned()));
        }

        if let Some(contract_slug) = contract_slug {
            params.push(("contractSlug".to_owned(), contract_slug.to_owned()));
        }

        self.stream_paginated("/bets".to_owned(), params)
    }
}
