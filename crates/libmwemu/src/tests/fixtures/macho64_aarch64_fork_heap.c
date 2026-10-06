// Child frees/reallocs heap blocks inherited from its parent. Under
// --memory-guard this must produce no findings. Exits 0 when every check passes.
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/wait.h>
int main(void) {
    char *buf = malloc(64);
    strcpy(buf, "parent-data");
    pid_t c = fork();
    if (c == 0) {
        char *mine = malloc(64);       /* child allocates next to inherited blocks */
        strcpy(mine, "child");
        buf = realloc(buf, 128);       /* child reallocs an inherited block */
        int ok = buf && strcmp(buf, "parent-data") == 0;
        free(buf);                     /* child frees an inherited block */
        free(mine);
        _exit(ok ? 0 : 1);
    }
    int st; waitpid(c, &st, 0);
    char *after = malloc(64);          /* parent allocates after child exit */
    strcpy(after, "x");
    int ok = WEXITSTATUS(st) == 0 && strcmp(buf, "parent-data") == 0 && after != buf;
    printf("child=%d parent=%s\n", WEXITSTATUS(st), ok ? "ALL OK" : "FAILED");
    free(buf); free(after);
    return !ok;
}
