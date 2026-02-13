from __future__ import annotations
import polars as pl
from navpipe.models.nav import NavResponse


def nav_response_to_df(resp: NavResponse) -> pl.DataFrame:
    df = pl.DataFrame(
        [
            {
                "scheme_code": int(resp.meta.scheme_code),
                "scheme_name": resp.meta.scheme_name,
                "date": item.date,
                "nav": float(item.nav),
            }
            for item in resp.data
        ]
    )

    return df.with_columns(
        [
            pl.col("date").str.strptime(pl.Date),
        ]
    ).select(["scheme_code", "scheme_name", "date", "nav"])
