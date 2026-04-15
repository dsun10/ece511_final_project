<!--
Copyright James Wainwright

SPDX-License-Identifier: MPL-2.0
-->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- Support `E` base ISA.
- Support `G` extension group.
- Support `Zca`, `Zcf`, and `Zcd` sub-extensions.

### Changed

- Renamed `Target::supports` to `Target::has_ext`.
- `Target::has_ext` no longer returns an error on unknown extensions.
- `Target` fields are now private and they must be constructed with `from_str`.

### Fixed

- Enable `F` when `D` is given to `Target::from_str`.
- Enable `Zicsr` when `F` is given to `Target::from_str`.
- Enable `F` when `Zfh` is given to `Target::from_str`.
- Enable `D` when `Q` is given to `Target::from_str`.

## 0.3.1 - 2025-03-01

### Fixed

- Corrected license text in README for MPL-2.0.

## 0.3.0 - 2025-03-01

### Added

- `supports` function for target extensions.
- `contains` function for targets.
- `Display` implementation for `Target`.
- `from_str_strict` function for `Target`.
- `load` and `store` checking functions for instructions.
- CSR definitions and functions for lookup by number and name.

### Fixed

- `RORIW` is no longer decoded on RV32.

## 0.2.0 - 2024-12-20

### Added

- `branch` checking function for instructions.
- `decode_le_bytes` for decoding and decompressing from a byte stream.

### Changed

- Documentation improved.
- Sub-modules removed.
- `LUI` and `AUIPC` immediates are no longer shifted to upper bits.

## 0.1.0 - 2024-11-30

### Added

- Decoder for RV{32,64}IMAFDQCBZifencei_Zicsr_Zawrs_Zfh_Zba_Zbb_Zbc_Zbs_Zbkb.
- Basic assembly serialization.
- Tests for decoding.
- Target string parser.
