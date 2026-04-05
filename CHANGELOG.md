# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog,
and this project adheres to Semantic Versioning.

---

## [Unreleased]

### Added
- 

### Changed
- 

### Fixed
- 

---

## [0.3.1] - 2026-04-05
### Added
- 
### Changed
- Internal changes to help with scaling future features.
### Fixed
- 
---

## [0.3.0] - 2026-03-22
### Added
- 
### Changed
- Updated internal rust crates to make the stable ABI build with python 3.13.
### Fixed
- Removed `nav_history_lazy()` method as it will work with polars versions that have polars rust crate version 0.53.0.
---

## [0.2.0] - 2026-03-08
### Added
- `nav_history_lazy()` to return results into a `pl.LazyFrame`.
- Python type stubs for the public API.
### Changed
- Migrated all backend logic to Rust.
- Removed rate limiting per second.
### Fixed
- 
---

## [0.1.3] - 2026-02-14

### Added
- 

### Changed
- Made `nav_response_to_df()` faster by removing `.select()` and not creating a `dict` object.

### Fixed
- 

---

## [0.1.2] - 2026-02-14

### Added
- Added `py.typed` marker (PEP 561) to enable static type checking for downstream users.

---

## [0.1.1] - 2026-02-13

### Added
- Initial public release.
- Core functionality.
- Inline type hints.
