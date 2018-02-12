//! # Runtime support for `lttng-ust-rs`.
//! Currently only exports a macro to make importing your tracepoints more convenient.
#![deny(missing_docs)]

/// Imports tracepoints. See the module documentation for `lttng-ust-generate` or the `examples` folder in
/// [the repo](https://github.com/bobtwinkles/lttng-ust-rs/tree/master/examples)
/// for an examples of how to use this macro.
// TODO: allow specification of level of publicness for tracepoints
#[macro_export]
macro_rules! import_tracepoints {
    ($src:expr, $module_name:ident) => {
        mod $module_name {
            include!($src);
        }
    };
}
