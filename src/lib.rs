mod client;
pub mod types;

pub use client::{ManifoldAuthorization, ManifoldClient};

#[cfg(test)]
mod tests {
    use crate::types::User;

    use super::*;

    #[tokio::test]
    async fn it_works() -> anyhow::Result<()> {
        dotenv::dotenv().ok();

        let manifold = ManifoldClient::from_api_key(&std::env::var("MANIFOLD_API_KEY").expect(
            "The test requires a MANIFOLD_API_KEY environment variable (you can use .env)",
        ))?;

        let r = manifold
            .http_get("/user/ValentinGolev")
            .send()
            .await?
            .json::<User>()
            .await?;

        println!("{:#?}", r);
        println!("{:#?}", r.balance());
        println!("{:#?}", r.profit_cached());

        Ok(())
    }
}
