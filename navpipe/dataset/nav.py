from __future__ import annotations
import asyncio
import polars as pl
from typing import Iterable
from navpipe.transport.async_http import AsyncNavPipeHTTP
from navpipe.transform.nav import nav_response_to_df
from navpipe.models.nav import NavResponse, DateRange
import msgspec


async def _fetch_nav_df(
    http: AsyncNavPipeHTTP,
    scheme_code: int,
    date_range: DateRange | None,
    sem: asyncio.Semaphore,
) -> pl.DataFrame:
    params: dict[str, str] = {}
    if date_range:
        params["startDate"] = date_range.start_date
        params["endDate"] = date_range.end_date

    async with sem:
        raw_bytes = await http.get_raw(f"/mf/{scheme_code}", params=params)

    # Decode directly into NavResponse
    typed = msgspec.json.decode(raw_bytes, type=NavResponse)

    return nav_response_to_df(typed)


async def fetch_nav_history_bulk(
    scheme_codes: Iterable[int],
    *,
    date_range: DateRange | None = None,
    max_concurrency: int = 5,
    rate_limit_per_sec: int | None = None,
) -> pl.DataFrame:
    sem = asyncio.Semaphore(max_concurrency)

    async with AsyncNavPipeHTTP(rate_limit_per_sec=rate_limit_per_sec) as http:
        tasks = [_fetch_nav_df(http, code, date_range, sem) for code in scheme_codes]

        dfs = await asyncio.gather(*tasks)

    return pl.concat(dfs, how="vertical")
