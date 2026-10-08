// fork() while another thread holds a mutex: the child gets only the calling
// thread, so the mutex stays locked by an owner that does not exist there and
// the child's lock() can never return. Natively the child hangs (alarm() turns
// that into SIGALRM here); mwemu must report the deadlock rather than let the
// child take the lock.
#include <pthread.h>
#include <stdio.h>
#include <sys/wait.h>
#include <unistd.h>

static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
static volatile int held, release;

static void *worker(void *arg) {
    (void)arg;
    pthread_mutex_lock(&lock);
    held = 1;
    while (!release) {}
    pthread_mutex_unlock(&lock);
    return NULL;
}

int main(void) {
    pthread_t t;
    pthread_create(&t, NULL, worker, NULL);
    while (!held) {}

    pid_t pid = fork();
    if (pid == 0) {
        alarm(1);
        pthread_mutex_lock(&lock); /* never returns */
        _exit(0);
    }
    int st = 0;
    waitpid(pid, &st, 0);
    release = 1;
    pthread_join(t, NULL);
    printf("child %s\n", WIFSIGNALED(st) ? "deadlocked (SIGALRM)" : "took the lock");
    return !WIFSIGNALED(st);
}
