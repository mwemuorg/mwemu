// Edge cases for mwemu's fork/pthread model. Exits 0 when every check passes.
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <pthread.h>
#include <sys/wait.h>

static volatile long counter[2];

static void *spin(void *arg) {
    long n = (long)arg;
    for (int i = 0; i < 1000; i++) counter[n]++;
    pthread_exit((void *)(n + 100));
}

int main(void) {
    int ok = 1, st = 0;

    // Nested fork: the grandchild exits 7, the child returns its status + 1 from main.
    pid_t c = fork();
    if (c == 0) {
        pid_t g = fork();
        if (g == 0) _exit(7);
        waitpid(g, &st, 0);
        return WEXITSTATUS(st) + 1;
    }
    ok &= wait(&st) == c && WIFEXITED(st) && WEXITSTATUS(st) == 8;

    // A child killed by abort() reports SIGABRT.
    pid_t a = fork();
    if (a == 0) abort();
    ok &= waitpid(a, &st, 0) == a && WIFSIGNALED(st) && WTERMSIG(st) == 6;
    ok &= waitpid(-1, &st, 0) == -1;

    // Threads run interleaved with main and hand back their exit values.
    pthread_t t[2];
    for (long i = 0; i < 2; i++) pthread_create(&t[i], NULL, spin, (void *)i);
    void *r0, *r1;
    pthread_join(t[0], &r0);
    pthread_join(t[1], &r1);
    ok &= counter[0] == 1000 && counter[1] == 1000;
    ok &= (long)r0 == 100 && (long)r1 == 101;
    ok &= !pthread_equal(pthread_self(), t[0]);

    printf("%s\n", ok ? "ALL OK" : "FAILED");
    return !ok;
}
