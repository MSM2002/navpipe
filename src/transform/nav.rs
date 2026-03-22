use crate::schema::nav::NavResponse;
use polars::prelude::*;

/// Convert NavResponse struct into a DataFrame
pub fn nav_response_to_df(resp: &NavResponse) -> Result<DataFrame, PolarsError> {
    let meta = &resp.meta;
    let dates: Vec<&str> = resp.data.iter().map(|x| x.date.as_str()).collect();
    let height = dates.len();

    let navs: Result<Vec<f64>, _> = resp.data.iter().map(|x| x.nav.parse::<f64>()).collect();

    let navs = navs.map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;

    let df = DataFrame::new(
        height,
        vec![
            Column::from(Series::new(
                "scheme_code".into(),
                vec![meta.scheme_code; height],
            )),
            Column::from(Series::new(
                "scheme_name".into(),
                vec![meta.scheme_name.clone(); height],
            )),
            Column::from(Series::new("date".into(), dates)),
            Column::from(Series::new("nav".into(), navs)),
        ],
    )?;

    let mut df = df;
    df.try_apply("date", |s| s.str()?.as_date(Some("%Y-%m-%d"), false))?;

    Ok(df)
}
