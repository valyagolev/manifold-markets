#![feature(iterator_try_collect)]

mod client;
pub mod error;
pub mod streams;
pub mod types;
pub use client::{ManifoldAuthorization, ManifoldClient};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() -> anyhow::Result<()> {
        dotenv::dotenv().ok();

        let manifold = ManifoldClient::from_api_key(&std::env::var("MANIFOLD_API_KEY").expect(
            "The test requires a MANIFOLD_API_KEY environment variable (you can use .env)",
        ))?;

        let r = manifold.get_groups(None).await?;

        println!("{:#?}", r[0]);

        Ok(())
    }
}
