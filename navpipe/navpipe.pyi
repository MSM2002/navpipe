from __future__ import annotations

import polars as pl

class NavPipe:
    def __init__(self, max_concurrency: int) -> None: ...
    def nav_history(
        self,
        scheme_codes: list[int],
        start_date: str | None = None,
        end_date: str | None = None,
    ) -> pl.DataFrame: ...
