// TODO: allow specification of level of publicness for tracepoints

// The @provider form Generates a module named $provider that contains all the tracepoints
// of a given class. Expects something of the form:
// ```
// (class1 =>
//    [instance1(a1name; arg1ty, a2name; arg2ty)
//    ,instance2(a1name; arg1ty, a2name; arg2ty)
//    , ...]
// ,class2 => [...]
// ,...
// )
// ```
// for the second argument

#[macro_export]
macro_rules! import_tracepoints {
    ($src:expr, $module_name:ident) => {
        mod $module_name {
            include!($src);
        }
    };
}
