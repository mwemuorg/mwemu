// Mutual exclusion under mwemu's per-instruction thread interleaving: pthread
// mutex types, os_unfair_lock, pthread_once and an LDXR/STXR loop.
// Exits 0 when every check passes.
#include <errno.h>
#include <os/lock.h>
#include <pthread.h>
#include <stdio.h>

#define NT 2
#define ITERS 500

static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
static os_unfair_lock ulock = OS_UNFAIR_LOCK_INIT;
static pthread_once_t once = PTHREAD_ONCE_INIT;
static long counter, ucounter, xcounter, init_runs;

static void init(void) { init_runs++; }

/* A read-modify-write long enough to be torn without the lock. */
static void slow_increment(long *p) {
    long v = *p;
    for (volatile int d = 0; d < 10; d++) {}
    *p = v + 1;
}

/* Exclusive load/store retry loop; inline asm so it cannot become LSE. */
static void llsc_increment(long *p) {
    long tmp;
    int fail;
    __asm__ volatile("1: ldxr %0, [%2]\n"
                     "   add %0, %0, #1\n"
                     "   stxr %w1, %0, [%2]\n"
                     "   cbnz %w1, 1b\n"
                     : "=&r"(tmp), "=&r"(fail)
                     : "r"(p)
                     : "memory");
}

static void *work(void *arg) {
    (void)arg;
    pthread_once(&once, init);
    for (int i = 0; i < ITERS; i++) {
        pthread_mutex_lock(&lock);
        slow_increment(&counter);
        pthread_mutex_unlock(&lock);

        os_unfair_lock_lock(&ulock);
        slow_increment(&ucounter);
        os_unfair_lock_unlock(&ulock);

        llsc_increment(&xcounter);
    }
    return NULL;
}

static int check_types(void) {
    int ok = 1;
    pthread_mutexattr_t attr;
    pthread_mutex_t rec, chk;

    pthread_mutexattr_init(&attr);
    pthread_mutexattr_settype(&attr, PTHREAD_MUTEX_RECURSIVE);
    pthread_mutex_init(&rec, &attr);
    ok &= pthread_mutex_lock(&rec) == 0 && pthread_mutex_lock(&rec) == 0;
    ok &= pthread_mutex_trylock(&rec) == 0;
    ok &= pthread_mutex_unlock(&rec) == 0 && pthread_mutex_unlock(&rec) == 0;
    ok &= pthread_mutex_unlock(&rec) == 0;
    ok &= pthread_mutex_unlock(&rec) == EPERM; /* no longer owned */

    pthread_mutexattr_settype(&attr, PTHREAD_MUTEX_ERRORCHECK);
    pthread_mutex_init(&chk, &attr);
    ok &= pthread_mutex_lock(&chk) == 0;
    ok &= pthread_mutex_lock(&chk) == EDEADLK;
    ok &= pthread_mutex_unlock(&chk) == 0;

    ok &= pthread_mutex_trylock(&lock) == 0;
    ok &= pthread_mutex_destroy(&lock) == EBUSY;
    ok &= pthread_mutex_unlock(&lock) == 0;
    ok &= os_unfair_lock_trylock(&ulock);
    os_unfair_lock_unlock(&ulock);
    return ok;
}

static void *try_held(void *arg) {
    return (void *)(long)(pthread_mutex_trylock((pthread_mutex_t *)arg) == EBUSY);
}

int main(void) {
    int ok = check_types();

    /* trylock from another thread while main holds the lock */
    pthread_t t[NT];
    void *busy;
    pthread_mutex_lock(&lock);
    pthread_create(&t[0], NULL, try_held, &lock);
    pthread_join(t[0], &busy);
    pthread_mutex_unlock(&lock);
    ok &= busy == (void *)1;

    for (int i = 0; i < NT; i++) pthread_create(&t[i], NULL, work, NULL);
    for (int i = 0; i < NT; i++) pthread_join(t[i], NULL);
    ok &= counter == NT * ITERS && ucounter == NT * ITERS && xcounter == NT * ITERS;
    ok &= init_runs == 1;

    printf("mutex=%ld unfair=%ld llsc=%ld once=%ld %s\n", counter, ucounter, xcounter,
           init_runs, ok ? "ALL OK" : "FAILED");
    return !ok;
}
