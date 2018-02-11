#include <stdio.h>
#include "hello-tp.h"

int main(int argc, char * argv[]) {
  int x;
  puts("Hello, world!\nPress enter to continue...");

  // Pause so we have a chance to list tracepoints
  getchar();

  tracepoint(hello_world, my_first_tracepoint, 23, "hi there!");

  for (x = 0; x < argc; ++x) {
    tracepoint(hello_world, my_first_tracepoint, x, argv[x]);
  }

  puts("Quitting now!");
  tracepoint(hello_world, my_first_tracepoint, x * x, "x^2");

  return 0;
}
