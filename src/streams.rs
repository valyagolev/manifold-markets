use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::{stream, Stream, StreamExt, TryStreamExt};

use crate::error::Result;
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
                    .json::<Vec<Value>>()
                    .await?;

                let Some(last) = result.last() else {
                return Result::Ok(None)
            };

                let last_id = last["id"].as_str().expect("is not id").to_owned(); // todo: use a real error

                let result = result
                    .into_iter()
                    .map(|v| serde_json::from_value(v))
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
}
