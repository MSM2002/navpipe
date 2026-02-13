from __future__ import annotations
import msgspec


class NavData(msgspec.Struct):
    date: str
    nav: str


class NavMeta(msgspec.Struct):
    fund_house: str
    scheme_type: str
    scheme_category: str
    scheme_code: int
    scheme_name: str
    isin_growth: str | None
    isin_div_reinvestment: str | None


class NavResponse(msgspec.Struct):
    meta: NavMeta
    data: list[NavData]


class DateRange(msgspec.Struct, frozen=True):
    start_date: str
    end_date: str
