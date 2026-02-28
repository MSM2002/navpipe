use crate::schema::nav::DateRange;
use crate::transform::nav::nav_response_to_df;
use crate::transport::client::AsyncNavPipeHTTP;
use polars::prelude::*;
use futures::future::join_all;
use tokio::sync::Semaphore;
use std::sync::Arc;

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
                let resp = client_clone.get_nav(code, dr_clone.as_ref()).await?;
                nav_response_to_df(&resp)
            })
        })
        .collect();

        let dfs: Vec<DataFrame> = join_all(tasks)
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    Ok(concat_df(&dfs)?.lazy())
}

/// Eager version (for convenience)
pub async fn fetch_nav_history_bulk_eager(
    scheme_codes: &[u32],
    date_range: Option<&DateRange>,
    max_concurrency: usize,
) -> Result<DataFrame, PolarsError> {
    fetch_nav_history_bulk(scheme_codes, date_range, max_concurrency)
        .await?
        .collect()?
}