# Changelog

All notable changes to this project are documented here.

## [Unreleased]

## [0.1.0] - 2026-07-23

### Added

- Added independent `panic-halt` and WS63 `panic-uart0` handlers.
- Added standalone official-nightly CI, lockfile checks, packaging, and tagged
  crates.io publishing.

### Fixed

- Use the WS63 UART register block's 32-bit MMIO width.
