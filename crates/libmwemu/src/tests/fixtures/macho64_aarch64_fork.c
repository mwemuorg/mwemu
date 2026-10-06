#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/wait.h>

#define NCHILD 3

static unsigned long work(int n) {
    unsigned long sum = 0;
    for (int i = 1; i <= (n + 1) * 10; i++)
        sum += (unsigned long)i * i;
    return sum;
}

int main(void) {
    pid_t pids[NCHILD];
    printf("parent pid=%d\n", getpid());
    fflush(stdout);
    for (int i = 0; i < NCHILD; i++) {
        pid_t pid = fork();
        if (pid < 0) { perror("fork"); return 1; }
        if (pid == 0) {
            unsigned long r = work(i);
            printf("child %d pid=%d ppid=%d result=%lu\n", i, getpid(), getppid(), r);
            fflush(stdout);
            _exit((int)(r & 0xff));
        }
        pids[i] = pid;
    }
    int ok = 1;
    for (int i = 0; i < NCHILD; i++) {
        int status = 0;
        pid_t w = waitpid(pids[i], &status, 0);
        int code = WIFEXITED(status) ? WEXITSTATUS(status) : -1;
        int expect = (int)(work(i) & 0xff);
        printf("reaped child %d pid=%d exit=%d expect=%d %s\n", i, w, code, expect,
               code == expect ? "OK" : "MISMATCH");
        if (code != expect) ok = 0;
    }
    printf("%s\n", ok ? "ALL OK" : "FAILED");
    return ok ? 0 : 1;
}
