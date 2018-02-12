# LTTNG-UST-RS
This is a collection of crates which provide bindings to `lttng-ust`
(Linux Trace Toolkit Next Generation - User Space Tracing).
`lttng-ust` is a low-overhead Linux usespace tracing framework with 10+ years
of open source development. This project aims to leverage all of that work
to allow for low-overhead tracing of Rust applications under Linux.

## Note for non-Linux users
Due to the nature of LTTNG, this crate is unlikely to work on non-Linux systems ;).
I am interested in writing a more general pure-Rust tracing framework but that is
a much more involved undertaking, and I'd like to get this crate polished first.

## Basic usage
Add the following to your `Cargo.toml`:

```toml
[dependencies]
lttng-ust = "0.1.0"

[build-dependencies]
lttng-ust-generate = "0.1.0"
```

And make sure you run something like this in your `build.rs`:

```rust
use std::env;
use std::path::PathBuf;

use lttng_ust_generate::{Provider, Generator, CTFType, CIntegerType};

let mut provider = Provider::new("my_first_rust_provider"); // stage 1
provider.create_class("my_first_class") //stage 2
    .add_field("my_integer_field", CTFType::Integer(CIntegerType::I32))
    .add_field("my_string_field", CTFType::SequenceText)
    .instantiate("my_first_tracepoint"); // stage 3

Generator::default()
    .generated_lib_name("tracepoint_library_link_name")
    .register_provider(provider)
    .output_file_name(PathBuf::from(env::var("OUT_DIR").unwrap()).join("tracepoints.rs"))
    .generate()
    .expect("Unable to generate tracepoint bindings");
```

And finally, use your brand new tracepoint!

```rust
extern crate lttng_ust;

import_tracepoints!(
    concat!(env!("OUT_DIR"), "/tracepoints.rs"),
    tracepoints
);

fn main() {
    tracepoints::my_first_rust_provider::my_first_tracepoint(42, "the meaning of life");
}
```

For more detailed documentation and examples, see the module docs for `lttng-ust-generate` and
the `examples/` documentation.
