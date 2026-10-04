// pthread_atfork handlers run in POSIX order around fork(): prepare in
// reverse registration order, parent and child in registration order. The
// prepare handler takes a lock that both sides then release and reuse.
// Exits 0 when every check passes.
#include <pthread.h>
#include <stdio.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>

static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
static char trace[32];

static void add(char c) { trace[strlen(trace)] = c; }
static void prepare1(void) { add('a'); pthread_mutex_lock(&lock); }
static void prepare2(void) { add('b'); }
static void parent1(void) { add('c'); pthread_mutex_unlock(&lock); }
static void parent2(void) { add('d'); }
static void child1(void) { add('e'); pthread_mutex_unlock(&lock); }
static void child2(void) { add('f'); }

int main(void) {
    pthread_atfork(prepare1, parent1, child1);
    pthread_atfork(prepare2, parent2, child2);

    pid_t pid = fork();
    if (pid == 0) {
        /* child: prepare b,a then child e,f; the lock must be usable */
        int ok = strcmp(trace, "baef") == 0 && pthread_mutex_trylock(&lock) == 0;
        _exit(ok ? 0 : 1);
    }
    int st = 0;
    waitpid(pid, &st, 0);
    int ok = WIFEXITED(st) && WEXITSTATUS(st) == 0;
    ok &= strcmp(trace, "bacd") == 0 && pthread_mutex_trylock(&lock) == 0;
    printf("parent=%s child=%d %s\n", trace, WEXITSTATUS(st), ok ? "ALL OK" : "FAILED");
    return !ok;
}
