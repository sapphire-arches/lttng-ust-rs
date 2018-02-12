# My First (rust) Tracepoint
This example is essentially a direct port of the
[LTTNG-2.10 user space tracing example application](http://lttng.org/docs/v2.10/#doc-tracing-your-own-user-application).
Tracepoints are defined in `build.rs` and consumed in `src/main.rs` and some effort has been made to
make the resulting `my_first_tracepoint` binary behave exactly the same way that the `hello` application
from the LTTNG example does.
