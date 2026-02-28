use crate::schema::nav::NavResponse;
use polars::prelude::*;

/// Convert NavResponse struct into a DataFrame
pub fn nav_response_to_df(resp: &NavResponse) -> Result<DataFrame, PolarsError> {
    let meta = &resp.meta;
    let dates: Vec<&str> = resp.data.iter().map(|x| x.date.as_str()).collect();
    let navs: Result<Vec<f64>, _> = resp
        .data
        .iter()
        .map(|x| x.nav.parse::<f64>())
        .collect();

    let navs = navs.map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;

    let mut df = DataFrame::new(vec![
        Series::new("scheme_code", vec![meta.scheme_code; dates.len()]),
        Series::new("scheme_name", vec![meta.scheme_name.clone(); dates.len()]),
        Series::new("date", dates),
        Series::new("nav", navs),
    ])?;

    // Parse date column
    df.try_apply("date", |s| s.utf8()?.as_date(Some("%Y-%m-%d"), false))?;

    Ok(df)
}