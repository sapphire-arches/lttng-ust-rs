
use bindgen::{Builder, Bindings};
use cc;
use std::env;
use std::path::PathBuf;

pub struct Generator {
    generated_lib_name: String,
}

impl Default for Generator {
    fn default() -> Self {
        Self {
            generated_lib_name: "tracepoints".into(),
        }
    }
}

impl Generator {
    pub fn generate(self) -> Result<Bindings, ()> {
        let mut generate_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        generate_path.push("lttng-tracepoints");

        let mut header_path = generate_path.clone();
        header_path.push(self.generated_lib_name.clone() + ".h");

        self.do_generate(&generate_path);

        let builder = Builder::default();
        builder
            .header(header_path.to_string_lossy())
            .generate()
    }

    pub fn do_generate(&self, generate_path: &PathBuf) {
        let mut code_path = generate_path.clone();
        code_path.push(self.generated_lib_name.clone() + ".c");
        // TODO: actually generate things
        cc::Build::new()
            .file(&code_path)
            .include(generate_path)
            .compile(&self.generated_lib_name)
    }
}
