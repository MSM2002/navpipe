from __future__ import annotations
import asyncio
import threading
from collections.abc import Coroutine
from typing import TypeVar

T = TypeVar("T")


def run_async(coro: Coroutine[object, object, T]) -> T:
    try:
        loop = asyncio.get_running_loop()
    except RuntimeError:
        loop = None

    if loop is None:
        return asyncio.run(coro)

    result: T | None = None

    def runner() -> None:
        nonlocal result
        result = asyncio.run(coro)

    t = threading.Thread(target=runner)
    t.start()
    t.join()

    assert result is not None
    return result
