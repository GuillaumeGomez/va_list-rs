#include <stdarg.h>
#include <stdio.h>

struct wrap {
    void (*f)(void*, va_list);
    void *arg;
};

void create_va_list(struct wrap *w, ...) {
    va_list ap;
    va_start(ap, w);
    w->f(w->arg, ap);
    va_end(ap);
}
