# Beatsaber Rust Utilities (BSRU)

A Beatsaber V3 parsing library.

## Version Support

### Info File

| Version | Description                                   | Supported |
|---------|-----------------------------------------------|-----------|
| 2.0     | Standard info format.                         | Yes       |
| 2.1     | Per difficulty environment and color schemes. | Yes       |
| 4.X     | Format overhaul.                              | No        |

### Difficulty File

| Version | Description                           | Supported |
|---------|---------------------------------------|-----------|
| 2.X     | Old un-abbreviated format.            | No        |
| 3.0     | Group lighting system.                | Yes       |
| 3.1     | Chunk, limit, and randomized filters. | Yes [^1]  |
| 3.2     | Translation events.                   | Yes       |
| 3.3     | More strobe functionality.            | No        |
| 4.X     | New template-like format.             | No        |

[^1]: Not supported by experimental lighting calculation methods.

## Feature Flags

| Flag           | Description                                                              |
|----------------|--------------------------------------------------------------------------|
| `bevy_color`   | Adds `From` implementations for converting color schemes to Bevy colors. |
| `bevy_reflect` | Adds `Reflect` derives for all types.                                    |