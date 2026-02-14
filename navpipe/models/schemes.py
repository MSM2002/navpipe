from __future__ import annotations
import msgspec

class SchemeMeta(msgspec.Struct):
    schemeCode: int
    schemeName: str
    isinGrowth: str | None
    isinDivReinvestment: str | None

SchemeResponse = list[SchemeMeta]
