use base64::prelude::*;
use reqwest::Client;

pub mod types;

use crate::constants::fii::FII_DETAILS_URL;
use crate::fii::details::types::DetailFundResponse;
use crate::fii::listed::types::Fii;

pub async fn fii(fund: &Fii) -> anyhow::Result<DetailFundResponse> {
    let client = Client::new();

    let params = format!(r#"{{"typeFund":7,"identifierFund":"{}"}}"#, fund.acronym);
    let mut data = String::new();

    BASE64_STANDARD.encode_string(params, &mut data);

    let details_url = format!("{FII_DETAILS_URL}/{data}");

    let response = client.get(details_url).send().await?;

    let fund_details = response.json::<DetailFundResponse>().await?;

    Ok(fund_details)
}
