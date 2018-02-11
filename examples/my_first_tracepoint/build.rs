#![feature(nll)]

extern crate lttng_ust;

use std::env;
use std::path::PathBuf;
use lttng_ust::{CTFType, CIntegerType};

fn main() {
    println!("cargo:rustc-link-lib=lttng-ust");

    // Create a provider and some tracepoints
    let mut provider = lttng_ust::Provider::new("hello_world");

    let ev_class1 = provider.create_class("class1");
    ev_class1
        .add_field("my_integer_field", CTFType::Integer(CIntegerType::I32))
        .add_field("my_string_field", CTFType::Text);

    let ev_class2 = provider.create_class("class2".to_string());
    ev_class2
        .add_field("int_field_2", CTFType::Integer(CIntegerType::U32))
        .add_field("native_string", CTFType::String);

    // Generate the tracepoint sources
    let tp_lib = "hello_world_tracepoints"
    let tracepoints = lttng_ust::Generator::default()
        .generated_lib_name(tp_lib)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("tracepoints.rs"))
        .expect("Couldn't write tracepoint bindings");
}
