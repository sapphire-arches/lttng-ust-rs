extern crate lttng_ust_generate;

use std::env;
use std::path::PathBuf;

use lttng_ust_generate::{Provider, CTFType, CIntegerType, Generator, LogLevel};

fn main() {
    let mut provider = Provider::new("rust_logging");
    {
        let log_entry_class = provider.create_class("log_entry")
            .add_field("file", CTFType::SequenceText)
            .add_field("line", CTFType::Integer(CIntegerType::U32))
            .add_field("module_path", CTFType::SequenceText)
            .add_field("target", CTFType::SequenceText)
            .add_field("message", CTFType::SequenceText);

        log_entry_class.instantiate_with_level("trace",LogLevel::Debug);
        log_entry_class.instantiate_with_level("debug",LogLevel::DebugLine);
        log_entry_class.instantiate_with_level("info" ,LogLevel::Info);
        log_entry_class.instantiate_with_level("warn" ,LogLevel::Warning);
        log_entry_class.instantiate_with_level("error",LogLevel::Error);
    }

    Generator::default()
        .generated_lib_name("rust_lttng_logging")
        .register_provider(provider)
        .output_file_name(PathBuf::from(env::var("OUT_DIR").unwrap()).join("logging_tracepoints.rs"))
        .generate()
        .expect("Unable to generate tracepoint bindings");
}
