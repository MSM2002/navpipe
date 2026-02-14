from __future__ import annotations
import polars as pl
from navpipe.models.nav import NavResponse


def nav_response_to_df(resp: NavResponse) -> pl.DataFrame:
    meta = resp.meta
    data = resp.data

    df = pl.DataFrame(
        {
            "scheme_code": meta.scheme_code,
            "scheme_name": meta.scheme_name,
            "date": [x.date for x in data],
            "nav": [x.nav for x in data],
        }
    )

    return df.with_columns(
        [
            pl.col("date").str.strptime(pl.Date, "%Y-%m-%d"),
            pl.col("nav").cast(pl.Float64),
        ]
    )