#include <stdarg.h>
#include <stdio.h>

struct wrap {
    void (*f)(void*, va_list);
    void *arg;
    unsigned int num_args;
};

void create_va_list(struct wrap *w, ...) {
    va_list ap;
    va_start(ap, w->num_args);
    w->f(w->arg, &ap);
    va_end(ap);
}