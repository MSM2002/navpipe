# NavPipe

**NavPipe** is a lightweight, typed Python SDK for fetching mutual fund NAV history from the unofficial API at:

👉 https://www.mfapi.in

It provides:

- 🚀 Simple synchronous interface  
- ⚡ Async HTTP under the hood  
- 📊 Native `polars` DataFrame output  
- 🧵 Built-in concurrency control  
- ⏱ Optional rate limiting  
- 🔁 Automatic retries with exponential backoff  
- 🧾 Fully typed models using `msgspec`  

> ⚠️ This project is **not affiliated with mfapi.in**. It is an unofficial wrapper.

---

## Important: Scheme Codes

NavPipe requires **scheme codes** to be provided manually.

You can find scheme codes directly on:

👉 https://www.mfapi.in

Example:

    https://api.mfapi.in/mf/119551

Here, `119551` is the scheme code.

NavPipe does not (yet) provide scheme discovery or search functionality.  
This may be added in future versions.

---

## Installation

```bash
pip install navpipe
```


---

## Requirements

- Python **3.9+**
- polars
- aiohttp
- msgspec
- aiolimiter

---

## Quick Start

```python
from navpipe import NavPipe

client = NavPipe(
max_concurrency=5,
rate_limit_per_sec=3,
)

df = client.nav_history(
scheme_codes=[119551, 120503],
start_date="2023-01-01",
end_date="2023-12-31",
)

print(df)
```


### Output

Returns a `polars.DataFrame` with:

| column        | type     |
|--------------|----------|
| scheme_code  | int      |
| scheme_name  | str      |
| date         | pl.Date  |
| nav          | float    |

---

## Public API

### NavPipe

```python
NavPipe(
*,
max_concurrency: int = 5,
rate_limit_per_sec: int | None = 3,
)
```

### nav_history(...)

```python
nav_history(
scheme_codes: Iterable[int],
*,
start_date: str | None = None,
end_date: str | None = None,
) -> pl.DataFrame
```


- `scheme_codes` – Iterable of mutual fund scheme codes  
- `start_date` / `end_date` – Optional date range (must be provided together)  
- Returns a vertically concatenated `polars.DataFrame`

---

## Error Handling

- Retries on:
  - 429
  - 500
  - 502
  - 503
  - 504
- Exponential backoff with jitter
- Raises `RuntimeError` after max retries
- Validates date range parameters

---

## Current Scope

- Fetch NAV history  
- Bulk concurrent fetching  
- Rate limiting  
- Typed decoding  
- Polars transformation  

---