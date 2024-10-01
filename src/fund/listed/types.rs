use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListedFundsResponse {
    pub page: Page,
    pub results: Vec<Fund>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub page_number: i64,
    pub page_size: i64,
    pub total_records: i64,
    pub total_pages: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fund {
    pub segment: String,
    pub acronym: String,
    pub fund_name: String,
    pub company_name: String,
    pub cnpj: Value,
}
