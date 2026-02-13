import polars as pl
from typing import Iterable
from navpipe.dataset.nav import fetch_nav_history_bulk, DateRange
from navpipe.utils.sync import run_async

class NavPipe:
    def __init__(
        self,
        *,
        max_concurrency: int = 5,
        rate_limit_per_sec: int | None = 3,
    ):
        self._max_concurrency = max_concurrency
        self._rate_limit = rate_limit_per_sec

    def nav_history(
        self,
        scheme_codes: Iterable[int],
        *,
        start_date: str | None = None,
        end_date: str | None = None
    ) -> pl.DataFrame:
        
        if (start_date and not end_date) or (end_date and not start_date):
            raise ValueError("Both start_date and end_date must be provided together.")
        
        date_range = None
        if start_date and end_date:
            date_range = DateRange(start_date=start_date, end_date=end_date)

        return run_async(
            fetch_nav_history_bulk(
                scheme_codes=scheme_codes,
                date_range=date_range,
                max_concurrency=self._max_concurrency,
                rate_limit_per_sec=self._rate_limit,
            )
        )
