use anyhow::Ok;
use reqwest::Client;

pub mod types;

use crate::constants::fund::LIST_FUND_URL;
use crate::fund::listed::types::ListedFundsResponse;

pub async fn all() -> anyhow::Result<ListedFundsResponse> {
    let client = Client::new();

    let response = client.get(LIST_FUND_URL).send().await?;
    let fiis = response.json::<ListedFundsResponse>().await?;

    Ok(fiis)
}
