from __future__ import annotations
import polars as pl
from navpipe.models.schemes import SchemeResponse

def scheme_response_to_df(resp: SchemeResponse) -> pl.DataFrame:
    return pl.DataFrame(
        {
            "scheme_code": [x.schemeCode for x in resp],
            "scheme_name": [x.schemeName for x in resp],
            "isin_growth": [x.isinGrowth for x in resp],
            "isin_div_reinvestment": [x.isinDivReinvestment for x in resp],
        }
    )

