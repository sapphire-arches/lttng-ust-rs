#![feature(nll)]

extern crate lttng_ust_generate as lttng_ust;

use std::env;
use std::path::PathBuf;
use lttng_ust::{CTFType, CIntegerType};

fn main() {
    // Create a provider name "hello_world"
    let mut provider = lttng_ust::Provider::new("hello_world");

    // Create an event layout
    let ev_class1 = provider.create_class("class1");
    ev_class1
        .add_field("my_integer_field", CTFType::Integer(CIntegerType::I32))
        .add_field("my_string_field", CTFType::SequenceText);

    // Instantiate that layout to get an actual tracepoint
    ev_class1.instantiate("my_first_tracepoint");

    // Generate the tracepoint sources
    let tp_lib = "hello_world_tracepoints";
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    lttng_ust::Generator::default()
        // Sets the name of the generated static library file. This is important
        // if for whatever reason there's already a library name "tracepoints"
        // being linked.
        .generated_lib_name(tp_lib)
        // Set the filename where we should write tracepoint code to.
        .output_file_name(out_path.join("tracepoints.rs"))
        // Registers our provider with the generator so it will actually generate.
        // Registering a provider implicitly registers all event classes created
        // from it which in turn registers all instantiated tracepoints.
        .register_provider(provider)
        // Perform the generation
        .generate()
        // ... and error out if we can't generate the bindings.
        .expect("Unable to generate tracepoint bindings");
}
