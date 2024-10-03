use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailFundResponse {
    pub detail_fund: DetailFund,
    pub share_holder: ShareHolder,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailFund {
    pub acronym: String,
    pub trading_name: String,
    pub trading_code: String,
    pub trading_code_others: String,
    pub cnpj: String,
    pub classification: String,
    pub web_site: String,
    pub fund_address: String,
    #[serde(rename = "fundPhoneNumberDDD")]
    pub fund_phone_number_ddd: String,
    pub fund_phone_number: String,
    pub fund_phone_number_fax: String,
    pub position_manager: String,
    pub manager_name: String,
    pub company_address: String,
    #[serde(rename = "companyPhoneNumberDDD")]
    pub company_phone_number_ddd: String,
    pub company_phone_number: String,
    pub company_phone_number_fax: String,
    pub company_email: String,
    pub company_name: String,
    pub quota_count: String,
    pub quota_date_approved: String,
    #[serde(rename = "typeFNET")]
    pub type_fnet: Value,
    pub codes: Vec<String>,
    pub codes_other: Value,
    pub segment: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShareHolder {
    pub share_holder_name: String,
    pub share_holder_address: String,
    #[serde(rename = "shareHolderPhoneNumberDDD")]
    pub share_holder_phone_number_ddd: String,
    pub share_holder_phone_number: String,
    pub share_holder_fax_number: String,
    pub share_holder_email: String,
}
