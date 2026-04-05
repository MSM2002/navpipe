use crate::schema::nav::{DateRange, NavResponse};
use crate::transform::nav::nav_response_to_df;
use crate::transport::client::AsyncNavPipeHTTP;
use futures::future::join_all;
use polars::prelude::*;
use std::sync::Arc;
use tokio::sync::Semaphore;

/// Fetch NAV data concurrently for an owned list of scheme codes
pub async fn fetch_nav_history_bulk(
    scheme_codes: Vec<u32>, // Accepts the owned list
    _date_range: Option<DateRange>,
    max_concurrency: usize,
) -> Result<DataFrame, PolarsError> {
    // We initialize the client once to reuse the connection pool
    let client = AsyncNavPipeHTTP::new();
    let sem = Arc::new(Semaphore::new(max_concurrency));

    let tasks: Vec<_> = scheme_codes
        .into_iter() // Consumes the Vec, turning each u32 into an owned value
        .map(|code| {
            let client_clone = client.clone();
            let sem_clone = sem.clone();

            tokio::spawn(async move {
                let _permit = sem_clone.acquire().await.unwrap();

                // Path construction
                let path = format!("mf/{}", code);

                let resp: NavResponse = client_clone
                    .get_path(&path)
                    .await
                    .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;

                nav_response_to_df(&resp)
            })
        })
        .collect();

    let results = join_all(tasks).await;
    let mut dfs = Vec::with_capacity(results.len());

    for res in results {
        // Handle Task Join Error (Tokio) then the Polars Result
        let df = res.map_err(|e| PolarsError::ComputeError(e.to_string().into()))??;
        dfs.push(df);
    }

    // Combine all individual DataFrames into one large one
    polars::functions::concat_df_diagonal(&dfs)
}

