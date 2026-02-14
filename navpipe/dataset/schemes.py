from __future__ import annotations
import asyncio
import polars as pl
from navpipe.transport.async_http import AsyncNavPipeHTTP
import msgspec
from navpipe.models.schemes import SchemeResponse
from navpipe.transform.schemes import scheme_response_to_df

async def fetch_schemes(
    http: AsyncNavPipeHTTP,
    sem: asyncio.Semaphore,
) -> pl.DataFrame:
    
    async with sem:
        raw_bytes = await http.get_raw(f"/mf")

    # Decode directly into NavResponse
    typed = msgspec.json.decode(raw_bytes, type=SchemeResponse)

    return scheme_response_to_df(typed)