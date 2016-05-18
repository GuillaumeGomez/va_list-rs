#include <stdarg.h>

void create_va_list(void (*f)(void*, va_list), void *arg, unsigned int num_args, ...) {
    va_list ap;
    va_start(ap, num_args);
    f(arg, ap);
    va_end(ap);
}