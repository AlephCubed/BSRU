# Beatsaber Rust Utilities (BSRU)

[![Version](https://img.shields.io/crates/v/bsru)](https://crates.io/crates/bsru)
[![Docs](https://img.shields.io/docsrs/bsru)](https://docs.rs/bsru)
![License](https://img.shields.io/crates/l/bsru)

A Beatsaber V3 parsing library.

## Status

This project should support info file version `2.X` and difficulty file version `3.X`.
Modded features such as custom data are not currently supported.

There are also some experimental methods to help with lighting calculations.
These do not currently support all features and are marked as depreciated.

## Feature Flags

| Flag           | Description                                                              |
|----------------|--------------------------------------------------------------------------|
| `bevy_color`   | Adds `From` implementations for converting color schemes to Bevy colors. |
| `bevy_reflect` | Adds `Reflect` derives for all types.                                    |

### Bevy Version Compatibility

| Bevy   | BSRU          |
|--------|---------------|
| `0.17` | `0.4`         |
| `0.16` | `0.1` - `0.3` |
