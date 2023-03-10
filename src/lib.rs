//! Unofficial, shallowly-typed client for the Manifold Markets API.
//!
//! Based on the docs at [https://docs.manifold.markets/api][docs]. Tested, but not thorougly.
//!
//! [docs]: https://docs.manifold.markets/api
//!
//! See [`ManifoldClient`](ManifoldClient) for usage.

#![feature(iterator_try_collect)]

mod client;
pub mod error;
pub mod streams;
pub mod types;
pub use client::{ManifoldAuthorization, ManifoldClient};

#[cfg(test)]
mod tests {
    use futures_util::{StreamExt, TryStreamExt};
    use rand::seq::SliceRandom;

    use crate::types::Market;

    use super::*;

    #[tokio::test]
    async fn it_works() -> anyhow::Result<()> {
        dotenv::dotenv().ok();

        let manifold = ManifoldClient::from_api_key(&std::env::var("MANIFOLD_API_KEY").expect(
            "The test requires a MANIFOLD_API_KEY environment variable (you can use .env)",
        ))?;

        let r = manifold
            .stream_markets()
            .take(510)
            .try_collect::<Vec<_>>()
            .await?;

        println!("{:#?}", r[509]);

        let mut r = r.iter().filter(|m| m.is_active()).collect::<Vec<_>>();

        r.shuffle(&mut rand::thread_rng());

        {
            println!("testing yes/no");
            let yes_no = r
                .iter().find(|m| m.outcome_type() == types::OutcomeType::Binary)
                .unwrap();

            println!("will bet on {yes_no:#?}");

            let pool = yes_no.pool();

            let winning = pool
                .iter()
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();

            println!("will bet on {winning:#?}");

            let bet = manifold
                .post_bet(1, yes_no.id(), winning.0.clone(), None)
                .await?;

            println!("bet: {bet:#?}");
        }
        {
            println!("now for a FreeResponse");

            let free_response = r
                .iter().find(|m| m.outcome_type() == types::OutcomeType::FreeResponse)
                .unwrap();

            println!("will bet on {free_response:#?}");

            let pool = free_response.pool();
            let winning = pool
                .iter()
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();

            println!("will bet on {winning:#?}");

            let bet = manifold
                .post_bet(1, free_response.id(), winning.0.clone(), None)
                .await?;

            println!("bet: {bet:#?}");
        }
        {
            println!("now for a MultipleChoice");

            let multiple_choice = r
                .iter().find(|m| m.outcome_type() == types::OutcomeType::MultipleChoice)
                .unwrap();

            println!("will bet on {multiple_choice:#?}");

            let pool = multiple_choice.pool();
            let winning = pool
                .iter()
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();

            println!("will bet on {winning:#?}");

            let bet = manifold
                .post_bet(1, multiple_choice.id(), winning.0.clone(), None)
                .await?;

            println!("bet: {bet:#?}");
        }
        {
            println!("now for a PseudoNumeric");

            let pseudo_numeric = r
                .iter().find(|m| m.outcome_type() == types::OutcomeType::PseudoNumeric)
                .unwrap();

            println!("will bet on {pseudo_numeric:#?}");

            let pool = pseudo_numeric.pool();
            let winning = pool
                .iter()
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();

            println!("will bet on {winning:#?}");

            let bet = manifold
                .post_bet(1, pseudo_numeric.id(), winning.0.clone(), None)
                .await?;

            println!("bet: {bet:#?}");
        }

        let me = manifold.get_me().await?;

        println!("me: {me:#?}");

        println!("test streaming my bets...");
        let bets = manifold.stream_bets(Some(me.id()), None, None, None);

        let bets = bets.take(10).try_collect::<Vec<_>>().await?;

        println!("bet: {:#?}", bets[0]);

        Ok(())
    }
}
