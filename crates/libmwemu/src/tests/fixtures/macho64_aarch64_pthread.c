#include <stdio.h>
#include <pthread.h>

#define NT 3
static unsigned long results[NT];

static void *work(void *arg) {
    long n = (long)arg;
    unsigned long sum = 0;
    for (int i = 1; i <= (n + 1) * 10; i++) sum += (unsigned long)i * i;
    results[n] = sum;
    return (void *)sum;
}

int main(void) {
    pthread_t t[NT];
    for (long i = 0; i < NT; i++) pthread_create(&t[i], NULL, work, (void *)i);
    int ok = 1;
    for (long i = 0; i < NT; i++) {
        void *ret;
        pthread_join(t[i], &ret);
        printf("thread %ld result=%lu ret=%lu\n", i, results[i], (unsigned long)ret);
        if ((unsigned long)ret != results[i]) ok = 0;
    }
    printf("%s\n", ok ? "ALL OK" : "FAILED");
    return !ok;
}
