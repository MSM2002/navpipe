# NavPipe

**NavPipe** is a high-performance, compiled Python SDK for fetching mutual fund NAV history from the unofficial API at:

👉 https://www.mfapi.in

By moving the core logic to **Rust**, NavPipe now offers massive concurrency with minimal overhead, returning results directly into **Polars** memory.

### Why the Rust rewrite?
- 🦀 **Safety:** Memory-safe concurrent fetching using the `tokio` runtime.
- 🚀 **Speed:** Zero-copy data transfers between Rust and Python via `pyo3-polars`.
- 📊 **Lazy-First:** Native support for Polars `LazyFrame`, allowing you to optimize queries before they run.
- 🧵 **True Parallelism:** Bypasses the Python Global Interpreter Lock (GIL) for network I/O and data transformation.

---

## Installation

```bash
pip install navpipe
```
---

## Quick Start

```python
import navpipe
import polars as pl

# Initialize the Rust-backed client
client = navpipe.NavPipe(max_concurrency=10)

# Eager execution: Returns a polars.DataFrame immediately
df = client.nav_history(
    scheme_codes=[119551, 120503],
    start_date="2023-01-01",
    end_date="2023-12-31",
)

print(df)
```

### Output Schema

The Rust engine performs diagonal concatenation to ensure all scheme data is merged efficiently:

|Column|Type|Description|
|------|----|-----------|
|scheme_code|Int64|The MFAPI unique identifier|
|date|Date|The NAV date|
|nav|Float64|The Net Asset Value|

---

## Important: Scheme Codes

NavPipe requires scheme codes to be provided manually. You can find these on:
👉 https://www.mfapi.in

Example: `https://api.mfapi.in/mf/119551` → `119551` is the code.

---

## Public API

`NavPipe(max_concurrency: int)`

Initializes the engine.

- `max_concurrency`: Limits the number of simultaneous HTTP requests using a `tokio::sync::Semaphore`.

`nav_history(...) -> pl.DataFrame`

Eagerly fetches and collects data into a standard Polars DataFrame.

---
