use anyhow::Ok;
use reqwest::Client;

pub mod types;

use crate::constants::fii::LIST_FII_URL;
use crate::fii::listed::types::ListedFundsResponse;

pub async fn all() -> anyhow::Result<ListedFundsResponse> {
    let client = Client::new();

    let response = client.get(LIST_FII_URL).send().await?;
    let fiis = response.json::<ListedFundsResponse>().await?;

    Ok(fiis)
}

pub async fn tickers() -> anyhow::Result<Vec<String>> {
    let fiis = all().await?;
    let tickers = fiis
        .results
        .iter()
        .map(|fii| {
            let acronym = &fii.acronym;

            format!("{acronym}11")
        })
        .collect::<Vec<String>>();

    Ok(tickers)
}
