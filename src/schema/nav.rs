use serde::Deserialize;

/// Single NAV entry
#[derive(Debug, Clone, Deserialize)]
pub struct NavData {
    pub date: String,
    pub nav: String,
}

/// Metadata for the fund/scheme
#[derive(Debug, Clone, Deserialize)]
pub struct NavMeta {
    pub fund_house: String,
    pub scheme_type: String,
    pub scheme_category: String,
    pub scheme_code: u32,
    pub scheme_name: String,
    pub isin_growth: Option<String>,
    pub isin_div_reinvestment: Option<String>,
}

/// Full API response: meta + list of NAVs
#[derive(Debug, Clone, Deserialize)]
pub struct NavResponse {
    pub meta: NavMeta,
    pub data: Vec<NavData>,
}

/// Date range object (immutable)
#[derive(Debug, Clone, Deserialize)]
pub struct DateRange {
    pub start_date: String,
    pub end_date: String,
}