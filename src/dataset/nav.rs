use crate::transform::nav::nav_response_to_df;
use crate::transport::client::AsyncNavPipeHTTP;
use polars::prelude::*;
use futures::future::join_all;
use tokio::sync::Semaphore;
use std::sync::Arc;
use crate::schema::nav::{NavResponse, DateRange}; 

/// Fetch NAV data concurrently and return a LazyFrame
pub async fn fetch_nav_history_bulk(
    scheme_codes: &[u32],
    date_range: Option<&DateRange>,
    max_concurrency: usize,
) -> Result<LazyFrame, PolarsError> {
    let client = AsyncNavPipeHTTP::new();
    let sem = Arc::new(Semaphore::new(max_concurrency));

    let tasks: Vec<_> = scheme_codes
        .iter()
        .map(|&code| {
            let client_clone = client.clone();
            let dr_clone = date_range.cloned();
            let sem_clone = sem.clone();
            tokio::spawn(async move {
                let _permit = sem_clone.acquire().await.unwrap();
                let resp = client_clone.get_nav::<NavResponse>(code, dr_clone.as_ref())
                    .await
                    .map_err(|e: reqwest::Error| PolarsError::ComputeError(e.to_string().into()))?;
                
                nav_response_to_df(&resp)
            })
        })
        .collect();

    let results = join_all(tasks).await;
    let mut dfs = Vec::with_capacity(results.len());
    
    for res in results {
        let df = res.map_err(|e| PolarsError::ComputeError(e.to_string().into()))??;
        dfs.push(df);
    }

    polars::functions::concat_df_diagonal(&dfs).map(|df: polars::frame::DataFrame| df.lazy())
}

pub async fn fetch_nav_history_bulk_eager(
    scheme_codes: Vec<u32>, 
    date_range: Option<DateRange>, 
    max_concurrency: usize
) -> Result<DataFrame, PolarsError> {
    let lf = fetch_nav_history_bulk(&scheme_codes, date_range.as_ref(), max_concurrency).await?;
    let df = lf.collect()?;
    Ok(df)
}