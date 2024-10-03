use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListedFundsResponse {
    pub page: Page,
    pub results: Vec<Fii>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub page_number: i32,
    pub page_size: i32,
    pub total_records: i32,
    pub total_pages: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fii {
    pub segment: String,
    pub acronym: String,
    pub fund_name: String,
    pub company_name: String,
    pub cnpj: Value,
}
