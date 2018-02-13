# Changelog for lttng-ust-generate

## Version 0.1.1
  - Removed need to manually link the `lttng-ust` library, `Generator::generate`
  now emits the appropriate `cargo:rustc-link-lib` line automatically.
  - Added `EventClass::instantiate_with_level` which creates a new tracepoint at
  the specified logging level.

## Version 0.1.0
Initial release
