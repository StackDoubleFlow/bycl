#include <stdbool.h>

int test = 0xFFFFAAAA;

[[noreturn]] void entry() {
    int *port = (int *) 0x80000000;
    *port = test;

    while (true) {}
}
