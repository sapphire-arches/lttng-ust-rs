use bindgen::Builder;
use cc;
use std::env;
use std::path::PathBuf;
use super::{CTFType, Provider};

mod tracepoint_impl;
mod tracepoint_interface;

use self::tracepoint_impl::{generate_tp_impl, generate_tp_header};
use self::tracepoint_interface::{generate_interface_impl, generate_interface_header};

pub struct Generator {
    lib_name: String,
    providers: Vec<Provider>,
    output_file_name: PathBuf,
}

impl Default for Generator {
    fn default() -> Self {
        let generate_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        Self {
            lib_name: "tracepoints".into(),
            providers: Vec::new(),
            output_file_name: generate_path.join("tracepoints.rs"),
        }
    }
}

impl Generator {
    /// Sets the base name for the generated C library
    pub fn generated_lib_name<S: Into<String>>(mut self, s: S) -> Self {
        self.lib_name = s.into();
        self
    }

    /// Add a new provider to generate tracepoints for
    pub fn register_provider(mut self, p: Provider) -> Self {
        self.providers.push(p);
        self
    }

    pub fn output_file_name<P: Into<PathBuf>>(mut self, p: P) -> Self {
        self.output_file_name = p.into();
        self
    }

    /// Perform generation. Hands back a bindgen Bindings object that has been mostly set up for
    /// you, but you will need to call [`write_to_file`][bindgen::Bindings::write_to_file] to
    /// actually generate the bindings.
    pub fn generate(self) -> Result<(), ()> {
        // TODO: replace expects with something that passes errors upward
        let mut generate_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        generate_path.push("lttng-tracepoints");
        generate_path.push(&self.lib_name);

        self.generate_c_sources(&generate_path);

        let builder = Builder::default();
        let bindings = builder
            .header(self.interface_header(&generate_path).to_string_lossy())
            .generate()
            .expect(&format!("Failed to generate tracepoint bindings for {}", self.lib_name));

        bindings
            .write_to_file(&self.output_file_name)
            .expect(&format!("Failed to write raw tracepoint bindings for {}", self.lib_name));

        Ok(())
    }

    fn generate_c_sources(&self, generate_path: &PathBuf) {
        use std::fs;
        // Make sure the output directory exists
        fs::create_dir_all(generate_path)
            .expect("Failed to create source directory");

        // Generate and build C-language files
        let tp_hdr_pth = &self.tracepoint_header(&generate_path);
        let in_hdr_pth = &self.interface_header(&generate_path);
        generate_tp_header(tp_hdr_pth, &self.providers)
            .expect("Failed to generate tracepoint header");
        generate_interface_header(in_hdr_pth, &self.providers)
            .expect("Failed to generate interface header");

        let impl_paths = [
            self.local_path(generate_path, "_tp_impl.c"),
            self.local_path(generate_path, "_interface.c"),
        ];
        generate_tp_impl(&impl_paths[0], tp_hdr_pth)
            .expect("Failed to generate tracepoint implementation");
        generate_interface_impl(&impl_paths[1], &self.providers, tp_hdr_pth, in_hdr_pth)
            .expect("Failed to generate interface implementation");

        cc::Build::new()
            .files(&impl_paths)
            .include(generate_path)
            .compile(&self.lib_name);


    }

    fn tracepoint_header(&self, generate_path: &PathBuf) -> PathBuf {
        self.local_path(generate_path, "_tps.h")
    }

    fn interface_header(&self, generate_path: &PathBuf) -> PathBuf {
        self.local_path(generate_path, "_int.h")
    }

    fn local_path(&self, generate_path: &PathBuf, suffix: &str) -> PathBuf {
        generate_path.join(format!("{}{}", self.lib_name, suffix))
    }
}

fn ctf_field_c_type(ty: CTFType) -> &'static str {
    match ty {
        CTFType::Integer(i) |
        CTFType::IntegerNoWrite(i) |
        CTFType::IntegerHex(i) |
        CTFType::IntegerNetwork(i) |
        CTFType::IntegerNetworkHex(i) => i.c_type(),

        CTFType::Float(f) |
        CTFType::FloatNoWrite(f) => f.c_type(),

        CTFType::String | CTFType::StringNoWrite => "char *",
        CTFType::Array(i, _) | CTFType::ArrayNoWrite(i, _) => i.c_pointer_type(),
        CTFType::ArrayText(_) => "char *",
        CTFType::Sequence(i) | CTFType::SequenceNoWrite(i) => i.c_pointer_type(),
        CTFType::SequenceText | CTFType::SequenceTextNoWrite => "char *",
        CTFType::Enum | CTFType::EnumNoWrite => unimplemented!(),
    }
}
