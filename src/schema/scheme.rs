use serde::Deserialize;

// Metadata for scheme when all schemes are listed
#[derive(Debug, Clone, Deserialize)]
pub struct SchemeMeta {
    pub schemeCode: u32,
    pub schemeName: String,
    pub fundHouse: String,
    pub schemeType: String,
    pub schemeCategory: String,
    pub isinGrowth: Option<String>,
    pub isinDivReinvestment: Option<String>,
    pub nav: String,
    pub date: String,
}

// Scheme API response
pub type SchemeResponse = Vec<SchemeMeta>;
