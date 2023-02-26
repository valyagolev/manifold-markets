#![feature(iterator_try_collect)]

mod client;
pub mod error;
pub mod streams;
pub mod types;
pub use client::{ManifoldAuthorization, ManifoldClient};

#[cfg(test)]
mod tests {
    use futures_util::{StreamExt, TryStreamExt};

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

        let r = r.iter().filter(|m| m.is_active()).collect::<Vec<_>>();

        {
            println!("testing yes/no");
            let yes_no = r
                .iter()
                .filter(|m| m.outcome_type() == types::OutcomeType::Binary)
                .next()
                .unwrap();

            println!("will bet on {:#?}", yes_no);

            let pool = yes_no.pool();

            let winning = pool
                .iter()
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();

            println!("will bet on {:#?}", winning);

            let bet = manifold
                .post_bet(1, yes_no.id(), winning.0.clone(), None)
                .await?;

            println!("bet: {:#?}", bet);
        }
        {
            println!("now for a FreeResponse");

            let free_response = r
                .iter()
                .filter(|m| m.outcome_type() == types::OutcomeType::FreeResponse)
                .next()
                .unwrap();

            println!("will bet on {:#?}", free_response);

            let pool = free_response.pool();
            let winning = pool
                .iter()
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();

            println!("will bet on {:#?}", winning);

            let bet = manifold
                .post_bet(1, free_response.id(), winning.0.clone(), None)
                .await?;

            println!("bet: {:#?}", bet);
        }

        Ok(())
    }
}
