use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

use ::{CTFType, EventClass, Field, Provider};
use super::ctf_field_c_type;

pub(in super) fn generate_tp_impl(path: &PathBuf, include_path: &PathBuf) -> io::Result<()> {
    let mut outf = File::create(path)
        .expect(&format!("Failed to create tracepoint impl {:?}\n", path));
    write!(outf, "#define TRACEPOINT_CREATE_PROBES\n")?;
    write!(outf, "#define TRACEPOINT_DEFINE\n")?;
    write!(outf, "#include \"{}\"\n", include_path.to_string_lossy())
}

pub(in super) fn generate_tp_header(path: &PathBuf, providers: &[Provider]) -> io::Result<()> {
    let mut outf = File::create(path)
        .expect(&format!("Failed to create tracepoint header {:?}\n", path));

    write!(outf, "#undef TRACEPOINT_INCLUDE\n")?;
    write!(outf, "#define TRACEPOINT_INCLUDE \"{}\"\n\n", path.to_string_lossy())?;

    write!(outf, "#if !defined(_RUST_TRACEPOINT_GUARD)")?;
    write!(outf, " || defined(TRACEPOINT_HEADER_MULTI_READ)\n")?;

    write!(outf, "#define _RUST_TRACEPOINT_GUARD\n\n")?;
    for provider in providers {
        generate_provider(provider, &mut outf)?;
    }
    write!(outf, "#endif\n")?;
    write!(outf, "#include <lttng/tracepoint-event.h>\n")?;

    Ok(())
}

fn generate_provider<F: Write>(provider: &Provider, outf: &mut F) -> io::Result<()> {
    write!(outf, "#undef TRACEPOINT_PROVIDER\n")?;
    write!(outf, "#define TRACEPOINT_PROVIDER {}\n\n", provider.name)?;
    write!(outf, "#include <lttng/tracepoint.h>\n\n")?;
    write!(outf, "#include <stdint.h>\n")?;

    for event_class in &provider.classes {
        write!(outf, "TRACEPOINT_EVENT_CLASS(\n")?;
        write!(outf, "    {},\n", provider.name)?;
        generate_event_class(event_class, outf)?;

        write!(outf, "/**--== {} instances ==--**/\n", event_class.class_name)?;
        for instance in &event_class.instances {
            write!(outf, "TRACEPOINT_EVENT_INSTANCE(\n")?;
            write!(outf, "    {},\n", provider.name)?;
            write!(outf, "    {},\n", event_class.class_name)?;
            write!(outf, "    {},\n", instance.name)?;
            generate_tp_args(&event_class.fields, outf)?;
            write!(outf, "\n)\n\n")?;
        }
    }

    Ok(())
}

fn generate_event_class<F: Write>(event_class: &EventClass, outf: &mut F) -> io::Result<()> {
    write!(outf, "    {},\n", event_class.class_name)?;
    generate_tp_args(&event_class.fields, outf)?;
    write!(outf, ",\n    TP_FIELDS(\n")?;
    let mut first = true;
    for field in &event_class.fields {
        if first {
            first = false;
        } else {
            write!(outf, "\n")?;
        }
        write!(outf, "        ")?;
        generate_ctf_call(field, outf)?;
    }
    write!(outf, "\n    )\n")?;
    write!(outf, ")\n\n")?;

    Ok(())
}

fn generate_tp_args<F: Write>(fields: &[Field], outf: &mut F) -> io::Result<()> {
    write!(outf, "    TP_ARGS(\n")?;
    let mut first = true;
    for field in fields {
        if first {
            first = false;
        } else {
            write!(outf, ",\n")?;
        }
        write!(outf, "        {}, {}_arg",
               ctf_field_c_type(field.ctf_type),
               field.name)?;
        if field.ctf_type.is_sequence() {
            // TODO: actually pick a reasonably type here instead of using int
            write!(outf, ",\n        int, {}_len", field.name)?;
        }
    }
    write!(outf, "\n    )")
}

fn generate_ctf_call<F: Write>(field: &Field, outf: &mut F) -> io::Result<()> {
    match field.ctf_type {
        CTFType::Integer(i) =>
            write!(outf, "ctf_integer({0}, {1}, {1}_arg)", i.c_type(), field.name),
        CTFType::IntegerNoWrite(i) =>
            write!(outf, "ctf_integer_nowrite({0}, {1}, {1}_arg)", i.c_type(), field.name),
        CTFType::IntegerHex(i) =>
            write!(outf, "ctf_integer_hex({0}, {1}, {1}_arg)", i.c_type(), field.name),
        CTFType::IntegerNetwork(i) =>
            write!(outf, "ctf_network({0}, {1}, {1}_arg)", i.c_type(), field.name),
        CTFType::IntegerNetworkHex(i) =>
            write!(outf, "ctf_network_hex({0}, {1}, {1}_arg)", i.c_type(), field.name),
        CTFType::Float(f) =>
            write!(outf, "ctf_float({0}, {1}, {1}_arg)", f.c_type(), field.name),
        CTFType::FloatNoWrite(f) =>
            write!(outf, "ctf_float_nowrite({0}, {1}, {1}_arg)", f.c_type(), field.name),
        CTFType::String =>
            write!(outf, "ctf_string({0}, {0}_arg)", field.name),
        CTFType::StringNoWrite =>
            write!(outf, "ctf_string_nowrite({0}, {0}_arg)", field.name),
        CTFType::Array(i, l) =>
            write!(outf, "ctf_array({0}, {1}, {1}_arg, {2})", i.c_type(), field.name, l),
        CTFType::ArrayText(l) =>
            write!(outf, "ctf_array_text(text, {0}, {0}_arg, {1})", field.name, l),
        CTFType::ArrayNoWrite(i, l) =>
            write!(outf, "ctf_array_nowrite({0}, {1}, {1}_arg, {2})", i.c_type(), field.name, l),
        CTFType::Sequence(i) =>
            write!(outf, "ctf_sequence({0}, {1}, {1}_arg, int, {1}_len)", i.c_type(), field.name),
        CTFType::SequenceNoWrite(i) =>
            write!(outf, "ctf_sequence({0}, {1}, {1}_arg, int, {1}_len)", i.c_type(), field.name),
        CTFType::SequenceText =>
            write!(outf, "ctf_sequence_text(char, {0}, {0}_arg, int, {0}_len)", field.name),
        CTFType::SequenceTextNoWrite =>
            write!(outf, "ctf_sequence_text_nowrite(char, {0}, {0}_arg, int, {0}_len)", field.name),
        CTFType::Enum | CTFType::EnumNoWrite => unimplemented!(),
    }
}
