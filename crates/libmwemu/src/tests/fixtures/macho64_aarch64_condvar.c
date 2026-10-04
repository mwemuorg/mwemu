// Bounded producer/consumer queue on pthread condition variables. The
// consumer must see every item exactly once and in order. Exits 0 on success.
#include <errno.h>
#include <pthread.h>
#include <stdio.h>
#include <sys/time.h>

#define CAP 4
#define ITEMS 200

static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t not_full = PTHREAD_COND_INITIALIZER;
static pthread_cond_t not_empty = PTHREAD_COND_INITIALIZER;
static int buf[CAP], head, count;

static void *producer(void *arg) {
    (void)arg;
    for (int i = 1; i <= ITEMS; i++) {
        pthread_mutex_lock(&lock);
        while (count == CAP) pthread_cond_wait(&not_full, &lock);
        buf[(head + count++) % CAP] = i;
        pthread_cond_signal(&not_empty);
        pthread_mutex_unlock(&lock);
    }
    return NULL;
}

static void *consumer(void *arg) {
    (void)arg;
    long in_order = 1, expect = 1;
    for (int i = 0; i < ITEMS; i++) {
        pthread_mutex_lock(&lock);
        while (count == 0) pthread_cond_wait(&not_empty, &lock);
        int v = buf[head];
        head = (head + 1) % CAP;
        count--;
        pthread_cond_signal(&not_full);
        pthread_mutex_unlock(&lock);
        in_order &= v == expect++;
    }
    return (void *)in_order;
}

int main(void) {
    pthread_t p, c;
    void *in_order;
    pthread_create(&c, NULL, consumer, NULL);
    pthread_create(&p, NULL, producer, NULL);
    pthread_join(p, NULL);
    pthread_join(c, &in_order);

    /* A timed wait with an expired deadline returns ETIMEDOUT, holding the mutex. */
    struct timeval now;
    gettimeofday(&now, NULL);
    struct timespec past = {now.tv_sec - 1, 0};
    pthread_mutex_lock(&lock);
    int rc = pthread_cond_timedwait(&not_empty, &lock, &past);
    int relock = pthread_mutex_trylock(&lock); /* EBUSY: we hold it again */
    pthread_mutex_unlock(&lock);

    int ok = in_order == (void *)1 && rc == ETIMEDOUT && relock == EBUSY;
    printf("in_order=%ld timedwait=%d %s\n", (long)in_order, rc, ok ? "ALL OK" : "FAILED");
    return !ok;
}
