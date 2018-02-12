//! # Rust bindings to LTTNG-UST
//!
//! This library provides a way for Rust code to define [LTTNG](https://lttng.org) tracepoints.
//! If your current platform doesn't support LTTNG, (i.e. you're not on a Linux system) this
//! crate probably isn't too useful to you. However, if you are on Linux and you have a need
//! for high-performance tracing with a rich tooling ecosystem, this is the crate for you!
//!
//! ## Getting started
//! To get started, you'll need to add `lttng-ust-generate` to the `[build-dependencies]`
//! section of your `Cargo.toml`. Then, in your `build.rs`, add the following code:
//!
//! ```no_run
//! use std::env;
//! use std::path::PathBuf;
//!
//! use lttng_ust_generate::{Provider, Generator, CTFType, CIntegerType};
//!
//! let mut provider = Provider::new("my_first_rust_provider"); // stage 1
//! provider.create_class("my_first_class") //stage 2
//!     .add_field("my_integer_field", CTFType::Integer(CIntegerType::I32))
//!     .add_field("my_string_field", CTFType::SequenceText)
//!     .instantiate("my_first_tracepoint"); // stage 3
//!
//! Generator::default()
//!     .generated_lib_name("tracepoint_library_link_name")
//!     .register_provider(provider)
//!     .output_file_name(PathBuf::from(env::var("OUT_DIR").unwrap()).join("tracepoints.rs"))
//!     .generate()
//!     .expect("Unable to generate tracepoint bindings");
//! ```
//!
//! To break this down, there are basically three phases to the creation of tracepoints in
//! `lttng-ust-rs`. The first is creating a provider, which we do using the
//! [`Provider::new`](::Provider::new) constructor above. Provider names should be globally
//! unique to ease identification of your particular application or library on systems with
//! many lttng-ust events registered.
//!
//! Second, we need to create an [event class](::EventClass). An event class describes the
//! layout of a tracepoint event. Events can have up to 10 different fields. All field names
//! should be unique within the event class. See [CTFType](::CTFType) for a list of all the
//! types we currently support and how those types map to the `ctf_*` macros from
//! `man 3 lttng-ust`. Also important to note is the order of the [`.add_field`](::EventClass::add_field)
//! calls, since these determine the order of the arguments to the generated tracepoint function.
//!
//! Finally, we can instantiate our event class to create a specific [event](::EventInstance).
//! This is what causes `lttng-usg-generate` to actually emit a tracepoint we can use in our code.
//!
//! To actually use the tracepoints generated here, you'll also need the `lttng-ust` crate, which
//! contains all the runtime support for `lttng-ust-rs`. So after adding `lttng-ust = "0.1.0"` to
//! your `Cargo.toml`, in the main file for your project (probably `lib.rs` or `main.rs`) add
//! something like the following:
//!
//! ```ignore
//! import_tracepoints!(concat!(env!("OUT_DIR"), "/tracepoints.rs"), tracepoints)
//! ```
//!
//! While we recommend placing this in the root of your crate, the macro should work anywhere.
//! Note the first argument will generate the path we used above when invoking the generator.
//! The second argument to the macro is the name of the module where all the tracepoints
//! should be placed.
//!
//! Now we can use our tracepoint from anywhere in the code like so:
//!
//! ```ignore
//! tracepoints::my_first_rust_provider::my_first_tracepoint(42, "the meaning of life");
//! ```
//!
//! Have a look in the `examples` directory of the repository
//! [on GitHub](https://github.com/bobtwinkles/lttng-ust-rs/tree/master/examples)
//! for a complete usage sample.
//!
//! Happy tracing!
#![deny(missing_docs)]

extern crate bindgen;
extern crate cc;

mod generator;

pub use generator::Generator;

/// A tracepoint provider.
/// You usually only need to create one of these
pub struct Provider {
    name: String,
    classes: Vec<EventClass>,
}

/// A lttng-ust event provider.
impl Provider {
    /// Create a new tracepoint provider
    pub fn new<S: Into<String>>(name: S) -> Provider {
        // TODO: validate name
        Provider {
            name: name.into(),
            classes: Vec::new(),
        }
    }

    /// Create a new class of tracepoint event.
    pub fn create_class<S: Into<String>>(&mut self, class_name: S) -> &mut EventClass {
        self.classes.push(EventClass::new(class_name.into()));
        let cls_len = self.classes.len();
        &mut self.classes[cls_len - 1]
    }
}

/// Represents a class of events that we would like to trace
pub struct EventClass {
    /// The name of this class
    class_name: String,
    /// The provider for this class of tracepoint events.
    fields: Vec<Field>,
    /// The set of instances
    instances: Vec<EventInstance>,
}

/// Represents a class of tracepoints.
/// Every tracepoint of the same class shares the same set of fields.
/// You can have as many tracepoints of the same class as you like, but tracepoint
/// names are namespaced per provider, not per-class.
impl EventClass {
    /// Create a new tracepoint event class
    fn new(class_name: String) -> Self {
        EventClass {
            class_name,
            fields: Vec::new(),
            instances: Vec::new(),
        }
    }

    /// Adds a new field to the tracepoint.
    /// See the [module level documentation](index.html) for examples.
    pub fn add_field<S: Into<String>>(&mut self, field_name: S, ty: CTFType) -> &mut Self {
        self.fields.push(Field::new(
            field_name.into(), ty
        ));
        // TODO: make sure field names don't conflict
        self
    }

    /// Instantiate the class, creating a new tracepoint.
    /// See the [module level documentation](index.html) for examples.
    pub fn instantiate<S: Into<String>>(&mut self, instance_name: S) -> &mut Self {
        // TODO: make sure instance names don't conflict.
        // This gets tricky because we can't conflict with any name in the parent provider's namespace.
        self.instances.push(EventInstance::new(
            instance_name.into()
        ));
        self
    }
}

/// A field in a tracing event
pub struct Field {
    ctf_type: CTFType,
    name: String,
}

impl Field {
    fn new(name: String, ctf_type: CTFType) -> Self {
        Self {
            ctf_type, name,
        }
    }
}

/// An instantiated [EventClass](::EventClass).
/// Every `EventInstance` represents a new tracepoint in the final binary
pub struct EventInstance {
    name: String,
}

impl EventInstance {
    fn new(name: String) -> Self {
        EventInstance {
            name,
        }
    }
}

/// Represents the log level for a given tracepoint
pub enum LogLevel {
    /// Corresponds to the `TRACE_EMERG` log level
    Emergency,
    /// Corresponds to the `TRACE_ALERT` log level
    Alert,
    /// Corresponds to the `TRACE_CRIT` log level
    Critical,
    /// Corresponds to the `TRACE_ERR` log level
    Error,
    /// Corresponds to the `TRACE_WARNING` log level
    Warning,
    /// Corresponds to the `TRACE_NOTICE` log level
    Notice,
    /// Corresponds to the `TRACE_INFO` log level
    Info,
    /// Corresponds to the `TRACE_DEBUG_SYSTEM` log level
    DebugSystem,
    /// Corresponds to the `TRACE_DEBUG_PROGRAM` log level
    DebugProgram,
    /// Corresponds to the `TRACE_DEBUG_PROCESS` log level
    DebugProcess,
    /// Corresponds to the `TRACE_DEBUG_MODULE` log level
    DebugModule,
    /// Corresponds to the `TRACE_DEBUG_UNIT` log level
    DebugUnit,
    /// Corresponds to the `TRACE_DEBUG_FUNCTION` log level
    DebugFunction,
    /// Corresponds to the `TRACE_DEBUG_LINE` log level
    DebugLine,
    /// Corresponds to the `TRACE_DEBUG` log level
    Debug
}

/// Represents a C integer type
#[derive(Copy,Clone,PartialEq,Eq,Debug)]
#[allow(missing_docs)]
pub enum CIntegerType {
    I8, I16, I32, I64,
    U8, U16, U32, U64,
}

impl CIntegerType {
    /// String version of the C type this represents
    fn c_type(&self) -> &'static str {
        match *self {
            CIntegerType::I8 =>   "int8_t",
            CIntegerType::U8 =>  "uint8_t",
            CIntegerType::I16 =>  "int16_t",
            CIntegerType::U16 => "uint16_t",
            CIntegerType::I32 =>  "int32_t",
            CIntegerType::U32 => "uint32_t",
            CIntegerType::I64 =>  "int64_t",
            CIntegerType::U64 => "uint64_t"
        }
    }

    /// String version of the C type this represents as a pointer
    fn c_pointer_type(&self) -> &'static str {
        match *self {
            CIntegerType::I8 =>   "int8_t *",
            CIntegerType::U8 =>  "uint8_t *",
            CIntegerType::I16 =>  "int16_t *",
            CIntegerType::U16 => "uint16_t *",
            CIntegerType::I32 =>  "int32_t *",
            CIntegerType::U32 => "uint32_t *",
            CIntegerType::I64 =>  "int64_t *",
            CIntegerType::U64 => "uint64_t *"
        }
    }

    /// String version of the Rust type this represents
    fn rust_type(&self) -> &'static str {
        match *self {
            CIntegerType::I8 => "i8",
            CIntegerType::U8 => "u8",
            CIntegerType::I16 => "i16",
            CIntegerType::U16 => "u16",
            CIntegerType::I32 => "i32",
            CIntegerType::U32 => "u32",
            CIntegerType::I64 => "i64",
            CIntegerType::U64 => "u64",
        }
    }
}

/// Represents a C float type
#[derive(Copy,Clone,PartialEq,Eq,Debug)]
#[allow(missing_docs)]
pub enum CFloatType {
    Single, Double
}

impl CFloatType {
    /// The C type represented by Self
    fn c_type(&self) -> &'static str {
        match *self {
            CFloatType::Single => "float",
            CFloatType::Double => "double",
        }
    }

    /// The analogous Rust type
    fn rust_type(&self) -> &'static str {
        match *self {
            CFloatType::Single => "f32",
            CFloatType::Double => "f64",
        }
    }
}

/// Represents a CTF type
#[derive(Copy,Clone,PartialEq,Eq,Debug)]

pub enum CTFType {
    /// A standard base-10 integer.
    /// Maps to `ctf_integer`.
    Integer(CIntegerType),
    /// A standard base-10 integer which is available to event filters, but is not persisted to the
    /// event itself.
    /// Maps to `ctf_integer_nowrite`.
    IntegerNoWrite(CIntegerType),
    /// Integer to be printed in hex format.
    /// Maps to `ctf_integer_hex`.
    IntegerHex(CIntegerType),
    /// Integer in network (BE) byte order.
    /// Maps to `ctf_integer_network`.
    IntegerNetwork(CIntegerType),
    /// Integer in network (BE) byte order, to be printed in hex.
    /// Maps to `ctf_niteger_network_hex`.
    IntegerNetworkHex(CIntegerType),
    /// IEEE single- or double- precision float.
    /// Maps to `ctf_float`.
    Float(CFloatType),
    /// IEEE single- or double- precision float which is available to event filters,
    /// but is not persisted to the event itself.
    /// Maps to `ctf_float_nowrite`.
    FloatNoWrite(CFloatType),
    /// A null-terminated string.
    /// Unless you're working with already-terminated `OsStrings`, you probably want to use a
    /// [SequenceText](CTFType::SequenceText) or [ArrayText](CTFType::ArrayText) instead.
    /// Maps to `ctf_string`.
    String,
    /// A null-terminated string which is available to event filters, but is not persisted.
    /// Unless you're working with already-terminated `OsStrings`, you probably want to use a
    /// [SequenceTextNoWrite](CTFType::SequenceTextNoWrite) instead.
    /// Maps to `ctf_string_nowrite`.
    StringNoWrite,
    /// A statically sized array of integers
    /// Maps to `ctf_array`.
    Array(CIntegerType, i32),
    /// A statically sized array of integers
    /// Maps to `ctf_array_text`.
    ArrayText(i32),
    /// A statically sized array of integers which is available to event filters, but is not
    /// persisted.
    /// Maps to `ctf_array_nowrite`.
    ArrayNoWrite(CIntegerType, i32),
    /* Things to add later: */
    // ArrayNetwork{NoWrite,Hex,NoWriteHex}, ArrayTextNoWrite
    /// Dynamically sized array of integers
    /// Maps to `ctf_sequence`.
    Sequence(CIntegerType),
    /// A dynamically sized array of integers which is available to event filters, but is not
    /// persisted.
    /// Maps to `ctf_sequence_nowrite`.
    SequenceNoWrite(CIntegerType),
    /* Things to add later */
    // SequenceHex, SequenceHexNoWrite, SequenceNetwork{,NoWrite,Hex,NoWriteHex}
    /// Dynamically-sized array, displayed as text
    /// Maps to `ctf_sequence_text`.
    SequenceText,
    /// Dynamically-sized array, displayed as text, but is not persisted.
    /// Maps to `ctf_sequence_text_nowrite`.
    SequenceTextNoWrite,
    /// Enumeration value.
    /// TODO: some sort of proc-macro skulduggery is probably required here.
    /// Maps to `ctf_enum`.
    Enum,
    /// Enumeration value. that is available to event filters but is not persisted
    /// TODO: some sort of proc-macro skulduggery is probably required here.
    /// Maps to `ctf_enum_nowrite`.
    EnumNoWrite,
}

impl CTFType {
    fn is_sequence(&self) -> bool {
        match *self {
            CTFType::Sequence(_) |
            CTFType::SequenceNoWrite(_) |
            CTFType::SequenceText |
            CTFType::SequenceTextNoWrite => true,
            _ => false,
        }
    }
}
