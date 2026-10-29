/* x86-64 hardware oracle (run under Rosetta): each stdin line is
 *   <hex machine code> r0 r1 ... r9
 * The code is a complete function taking a pointer to the 10 u64 registers in rdi.
 * Prints "OK r0..r9" or "EXC <signal>". */
#include <setjmp.h>
#include <signal.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>

typedef struct { uint64_t r[10]; } st_t;
static sigjmp_buf jb;
static volatile sig_atomic_t excno;
static void on_sig(int s) { excno = s; siglongjmp(jb, 1); }
static int ishex(int c) { return (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F'); }
static int hexval(int c) { return c <= '9' ? c - '0' : (c | 32) - 'a' + 10; }

int main(void) {
    size_t cap = 1 << 16;
    uint8_t *code = mmap(NULL, cap, PROT_READ | PROT_WRITE | PROT_EXEC, MAP_PRIVATE | MAP_ANON, -1, 0);
    if (code == MAP_FAILED) { perror("mmap rwx"); return 1; }
    struct sigaction sa; memset(&sa, 0, sizeof sa);
    sa.sa_handler = on_sig; sigemptyset(&sa.sa_mask); sa.sa_flags = SA_NODEFER;
    int sigs[] = {SIGFPE, SIGILL, SIGSEGV, SIGBUS, SIGTRAP};
    for (unsigned i = 0; i < sizeof sigs / sizeof *sigs; i++) sigaction(sigs[i], &sa, NULL);
    char *line = NULL; size_t n = 0;
    while (getline(&line, &n, stdin) > 0) {
        char *p = line; size_t len = 0;
        while (ishex(p[0]) && ishex(p[1]) && len < cap) { code[len++] = (uint8_t)(hexval(p[0]) * 16 + hexval(p[1])); p += 2; }
        st_t st;
        for (int i = 0; i < 10; i++) st.r[i] = strtoull(p, &p, 16);
        excno = 0;
        if (sigsetjmp(jb, 1) == 0) {
            ((void (*)(st_t *))code)(&st);
            printf("OK");
            for (int i = 0; i < 10; i++) printf(" %llx", (unsigned long long)st.r[i]);
            printf("\n");
        } else {
            __asm__ volatile("cld");
            printf("EXC %d\n", (int)excno);
        }
        fflush(stdout);
    }
    return 0;
}
