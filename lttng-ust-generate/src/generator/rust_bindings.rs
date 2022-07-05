use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

use ::{CTFType, EventClass, EventInstance, Field, Provider};

use super::tracepoint_interface::generate_func_name;

pub(in super) fn generate_rust_bindings(output_path: &PathBuf,
                                        providers: &[Provider],
                                        raw_bindings: &PathBuf) -> io::Result<()> {
    let mut outf = File::create(output_path)?;
    write_include(&mut outf, raw_bindings)?;
    write_providers(&mut outf, providers)?;

    Ok(())
}

fn write_include<F: Write>(outf: &mut F, raw_bindings_path: &PathBuf) -> io::Result<()> {
    write!(outf, "#[allow(non_upper_case_globals)]\n")?;
    write!(outf, "#[allow(non_camel_case_types)]\n")?;
    write!(outf, "#[allow(non_snake_case)]\n")?;
    write!(outf, "mod detail {{\n")?;
    write!(outf, "    include!(\"{}\");\n", raw_bindings_path.to_string_lossy())?;
    write!(outf, "}}\n")?;

    Ok(())
}

fn write_providers<F: Write>(outf: &mut F, providers: &[Provider]) -> io::Result<()> {
    for provider in providers {
        write!(outf, "pub(in super) mod {} {{", provider.name)?;
        for event_class in &provider.classes {
            for instance in &event_class.instances {
                let f = generate_instance_call(provider, event_class, instance);
                write!(outf, "{}\n", f)?;
            }
        }
        write!(outf, "}}\n\n")?;
    }
    Ok(())
}

fn generate_instance_call(provider: &Provider, class: &EventClass, instance: &EventInstance) -> String {
    let name = &instance.name;
    let type_args = "";
    let args: Vec<String> = class.fields.iter().enumerate()
        .map(|(i, field)| {
            format!("a{}: {}", i, rust_type_for(&field.ctf_type))
        }).collect();
    let args = &args.join(", ");
    let c_args: Vec<String> = class.fields.iter().enumerate()
        .map(|(i, field)| {
            c_arg_for_field(format!("a{}", i), field)
        }).collect();
    let c_args = &c_args.join(", ");
    let native_name = generate_func_name(provider, class, instance);
    format!(r"
        pub(in super::super) fn {}<{}>({}) {{
            unsafe {{
                super::detail::{}({})
            }}
        }}
", name, type_args, args, native_name, c_args)
}

fn rust_type_for(ty: &CTFType) -> String {
    use CTFType::*;
    match *ty {
        Integer(i) |
        IntegerNoWrite(i) |
        IntegerHex(i) |
        IntegerNetwork(i) |
        IntegerNetworkHex(i) => i.rust_type().into(),

        Float(f) |
        FloatNoWrite(f) => f.rust_type().into(),

        String |
        StringNoWrite => "::std::ffi::CStr".into(),

        Array(i, l) |
        ArrayNoWrite(i, l) => format!("&[{}; {}]", i.rust_type(), l),

        ArrayText(_) => "&str".into(),

        Sequence(i) |
        SequenceNoWrite(i) => format!("&[{}]", i.rust_type()),

        SequenceText |
        SequenceTextNoWrite => "&str".into(),

        Enum |
        EnumNoWrite => unimplemented!(),
    }
}

fn c_arg_for_field(base_name: String, field: &Field) -> String {
    if let CTFType::SequenceText = field.ctf_type {
        format!("::std::mem::transmute({0}.as_bytes().as_ptr()), {0}.len()", base_name)
    } else if field.ctf_type.is_sequence() {
        base_name.clone() + ", " + &base_name + ".len()"
    } else if let CTFType::Array(_, _) | CTFType::ArrayNoWrite(_, _) = field.ctf_type {
        format!("{}.as_ptr()", base_name)
    } else {
        base_name
    }
}
