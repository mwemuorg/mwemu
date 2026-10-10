/*
 * extract_dyld_cache.c — extract dylibs from macOS dyld shared cache
 *
 * Uses Apple's dsc_extractor.bundle (ships with macOS) to extract all dylibs
 * from the system's dyld shared cache into a target directory.
 *
 * Build:  cc -o extract_dyld_cache extract_dyld_cache.c
 * Usage:  ./extract_dyld_cache <output_dir>
 */

#include <dlfcn.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>

typedef int (*extractor_func_t)(const char *cache_path, const char *output_dir,
                                void (^progress)(unsigned current,
                                                 unsigned total));

static const char *cache_paths[] = {
    "/System/Volumes/Preboot/Cryptexes/OS/System/Library/dyld/"
    "dyld_shared_cache_arm64e",
    "/System/Library/dyld/dyld_shared_cache_arm64e",
    "/System/Library/dyld/dyld_shared_cache_arm64",
    NULL,
};

int main(int argc, char **argv) {
    if (argc != 2) {
        fprintf(stderr, "usage: %s <output_dir>\n", argv[0]);
        return 1;
    }
    const char *output_dir = argv[1];

    void *handle =
        dlopen("/usr/lib/dsc_extractor.bundle", RTLD_LAZY);
    if (!handle) {
        fprintf(stderr, "error: cannot load dsc_extractor.bundle: %s\n",
                dlerror());
        return 1;
    }

    extractor_func_t extract = (extractor_func_t)dlsym(
        handle, "dyld_shared_cache_extract_dylibs_progress");
    if (!extract) {
        fprintf(stderr, "error: cannot find extraction function: %s\n",
                dlerror());
        dlclose(handle);
        return 1;
    }

    const char *cache_path = NULL;
    for (int i = 0; cache_paths[i]; i++) {
        struct stat st;
        if (stat(cache_paths[i], &st) == 0) {
            cache_path = cache_paths[i];
            break;
        }
    }
    if (!cache_path) {
        fprintf(stderr, "error: dyld shared cache not found\n");
        dlclose(handle);
        return 1;
    }

    printf("cache: %s\n", cache_path);
    printf("output: %s\n", output_dir);

    mkdir(output_dir, 0755);

    int result = extract(cache_path, output_dir,
                         ^(unsigned current, unsigned total) {
                             printf("\r[%u/%u] extracting...", current, total);
                             fflush(stdout);
                         });

    printf("\n");
    if (result == 0) {
        printf("done.\n");
    } else {
        fprintf(stderr, "extraction failed with code %d\n", result);
    }

    dlclose(handle);
    return result;
}
