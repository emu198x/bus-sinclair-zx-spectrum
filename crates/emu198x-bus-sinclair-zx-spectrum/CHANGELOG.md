# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-11-02

### Fixed

- Run the Pentagon 128 at 3.5 MHz from a 14 MHz master clock, as the original
  did, rather than 14.336 MHz: a 71,680 T-state frame lasts 20.48 ms, the
  Pentagon's 48.83 Hz refresh (emu198x/emu198x#2231). 14.336 MHz is the ZX
  Spectrum Next's VGA 50Hz Pentagon mode.

## 0.1.0

### Added

- The `Ula`, `MemoryBus`, `Peripheral` and `MemoryIntercept` traits, and the
  `FrameTiming` descriptor with each Spectrum-family video configuration's
  frame data. They moved here unchanged from `common-sinclair-zx-spectrum`,
  the family's glue, which re-exports them, so that the ULAs and peripherals
  that implement them can be published without the glue.
