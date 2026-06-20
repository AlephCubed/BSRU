# Changelog

## 0.8.0 (2026-06-19)

- Updated to Bevy 0.19.
- Moved primary repository to Tangled.
- Moved changelog in-repo.

## 0.7.0 (2026-01-14)

- Updated to Bevy 0.18.

## 0.7.0-rc.1 (2025-12-18)

- Updated to Bevy 0.18.0-rc.1.

## 0.6.0 (2025-12-18)

- No additional changes compared to the release candidate.

## 0.6.0-rc.1 (2025-12-16)

- Added `get_duration` method to `EventGroup`.

## 0.5.0 (2025-12-16)

- No additional changes compared to the beta.

## 0.5.0-beta.1 (2025-12-11)

- Move loose enum macro to its own external crate, `loose_enum`.
  - With this change, all `Unknown` enum variants have been renamed to `Undefined`.
- Implemented `TryFrom<LooseBool>` for `bool`.

## 0.4.0 (2025-09-30)

- Update to Bevy 0.17.0.

## 0.4.0-rc.1 (2025-09-12)

- Update to Bevy 0.17.0-rc.1.

## 0.3.0 (2025-09-12)

- Implement `From`/`Into` conversions for `Color` and `bevy_color::Color`.

## 0.3.0-beta.1 (2025-08-09)

- Fixed bug when wave distribution value is less than the data offset.
- Added support for FX events.
  - Unlike the other V3 group event types, FX events use a template-like JSON syntax.
   In order to have standardized structure across all V3 events,
   serialization has been written in `FxEventContainer` instead of
   `FxEventBox` or `FxEventGroup`.

## 0.2.0 (2025-07-28)

- Minor doc improvements.

## 0.2.0-beta.2 (2025-07-25)

- Added support for limits in lighting calculations.
- Refactored `LimitBehaviour`:
  - `LimitBehaviour::Duration` was renamed to `Beat`.
  - `LimitBehaviour::Distribution` was renamed to `Value`.
  - Added `LimitBehaviour::Both`.
  - Added `beat_enabled` and `value_enabled` helper methods.
- Re-exports are now hidden from docs.
- The `utils` module is not private.
  - `LooseBool` can be accessed via the library root.

## 0.2.0-beta.1 (2025-07-24)

- Added support for chunks in lighting calculations.
- Changed the default chunk value for `Filter` from `Some(1)` to `Some(0)`.

## 0.1.0 (2025-07-22)

- Initial release.
