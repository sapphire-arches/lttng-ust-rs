extern crate lttng_ust_logging;
#[macro_use]
extern crate log;

fn main() {
    // Initialize the logger
    lttng_ust_logging::init();

    // Give the user a change to list out tracepoints
    wait_for_enter();

    // Print some example texts
    trace!("Hello from trace");
    debug!("Hello from debug");
    info!("Hello from info");
    warn!("Hello from warn");
    error!("Hello from error");
}

// A small helper function to emulate the behavior of getchar().
// Nothing interesting to see here really
fn wait_for_enter() {
    use std::io;
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line from stdin");
}
