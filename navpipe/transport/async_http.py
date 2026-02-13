from __future__ import annotations
import aiohttp
import asyncio
import random
import msgspec
from aiolimiter import AsyncLimiter
import types


class AsyncNavPipeHTTP:
    BASE_URL = "https://api.mfapi.in"

    def __init__(
        self,
        *,
        timeout: int = 20,
        rate_limit_per_sec: int | None = None,
        max_retries: int = 4,
    ):
        self.timeout: aiohttp.ClientTimeout = aiohttp.ClientTimeout(total=timeout)
        self._session: "aiohttp.ClientSession | None" = None
        self._limiter: "AsyncLimiter | None" = (
            AsyncLimiter(rate_limit_per_sec, time_period=1)
            if rate_limit_per_sec
            else None
        )
        self._max_retries: int = max_retries
        self._decoder = msgspec.json.Decoder()

    async def __aenter__(self) -> AsyncNavPipeHTTP:
        self._session = aiohttp.ClientSession(timeout=self.timeout)
        return self

    async def __aexit__(
        self,
        _exc_type: type | None,
        _exc_value: BaseException | None,
        _tb: "types.TracebackType | None",
    ) -> None:
        _ = (_exc_type, _exc_value, _tb)
        if self._session:
            await self._session.close()
            self._session = None

    async def get_raw(
        self, endpoint: str, params: dict[str, str] | None = None
    ) -> bytes:
        if self._limiter:
            async with self._limiter:
                return await self._request_raw(endpoint, params)
        return await self._request_raw(endpoint, params)

    async def get(
        self, endpoint: str, params: dict[str, str] | None = None
    ) -> dict[str, object]:
        raw_bytes = await self.get_raw(endpoint, params)
        return self._decoder.decode(raw_bytes)

    async def _request_raw(
        self, endpoint: str, params: dict[str, str] | None = None
    ) -> bytes:
        if not self._session:
            raise RuntimeError(
                "HTTP session not initialized. Use async context manager."
            )

        url = f"{self.BASE_URL}{endpoint}"

        for attempt in range(self._max_retries):
            try:
                async with self._session.get(url, params=params) as resp:
                    if resp.status in {429, 500, 502, 503, 504}:
                        retry_after = resp.headers.get("Retry-After")
                        if retry_after:
                            await asyncio.sleep(float(retry_after))
                        else:
                            backoff = 2**attempt + random.uniform(0, 0.3)
                            await asyncio.sleep(backoff)
                        continue

                    resp.raise_for_status()
                    return await resp.read()

            except (aiohttp.ClientError, asyncio.TimeoutError) as e:
                if attempt < self._max_retries - 1:
                    backoff = 2**attempt + random.uniform(0, 0.3)
                    await asyncio.sleep(backoff)
                    continue
                raise RuntimeError(f"Max retries exceeded for {url}") from e

        raise RuntimeError(f"Max retries exceeded for {url}")
