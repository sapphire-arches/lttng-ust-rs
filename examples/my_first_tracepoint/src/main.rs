#[macro_use]
extern crate lttng_ust;

import_tracepoints!(
    concat!(env!("OUT_DIR"), "/tracepoints.rs"),
    tracepoints
);

fn wait_for_enter() {
    use std::io;
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line from stdin");
}

fn main() {
    println!("Hello, world!");

    wait_for_enter();
    tracepoints::hello_world::class1::my_first_tracepoint(26, "hello world");
}
