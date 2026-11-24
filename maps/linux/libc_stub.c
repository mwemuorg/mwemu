int __libc_start_main(
    int (*main_fn)(int, char **, char **),
    int argc,
    char **argv,
    void (*init)(void),
    void (*fini)(void),
    void (*rtld_fini)(void),
    void *stack_end
) {
    (void)main_fn;
    (void)argc;
    (void)argv;
    (void)init;
    (void)fini;
    (void)rtld_fini;
    (void)stack_end;
    return 0;
}

int puts(const char *s) {
    (void)s;
    return 0;
}

int printf(const char *fmt, ...) {
    (void)fmt;
    return 0;
}

int __cxa_finalize(void *dso_handle) {
    (void)dso_handle;
    return 0;
}

int __cxa_atexit(void (*func)(void *), void *arg, void *dso_handle) {
    (void)func;
    (void)arg;
    (void)dso_handle;
    return 0;
}

void *__gmon_start__(void) {
    return 0;
}

void exit(int status) {
    (void)status;
    for (;;) {
    }
}

void _exit(int status) {
    (void)status;
    for (;;) {
    }
}

void *malloc(unsigned long size) {
    (void)size;
    return 0;
}

void *calloc(unsigned long count, unsigned long size) {
    (void)count;
    (void)size;
    return 0;
}

void *realloc(void *ptr, unsigned long size) {
    (void)ptr;
    (void)size;
    return 0;
}

void free(void *ptr) {
    (void)ptr;
}

void *memcpy(void *dst, const void *src, unsigned long len) {
    (void)src;
    (void)len;
    return dst;
}

void *memset(void *dst, int value, unsigned long len) {
    (void)value;
    (void)len;
    return dst;
}


// The emulator dispatches these exported addresses to its Linux libc handlers.
void *mmap(void *addr, unsigned long len, int prot, int flags, int fd, long offset) { return (void *)-1; }
int mprotect(void *addr, unsigned long len, int prot) { return -1; }
long syscall(long number, ...) { return -1; }
long sysconf(int name) { return -1; }
void perror(const char *label) { (void)label; }
