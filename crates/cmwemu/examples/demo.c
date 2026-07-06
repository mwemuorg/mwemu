/*
 * cmwemu demo: emulate a tiny x86-64 shellcode and read back a register.
 *
 * Build (from the repo root, after `cargo build`):
 *
 *   cc examples/demo.c -I include \
 *      -L ../target/debug -lcmwemu \
 *      -o /tmp/mwemu_demo
 *   LD_LIBRARY_PATH=../target/debug /tmp/mwemu_demo
 *
 * Or link the static lib (no LD_LIBRARY_PATH needed at runtime):
 *
 *   cc examples/demo.c -I include \
 *      ../target/debug/libcmwemu.a -lpthread -ldl -lm \
 *      -o /tmp/mwemu_demo && /tmp/mwemu_demo
 */

#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

#include "mwemu.h"

int main(void) {
    /* mov rax, 1 ; mov rbx, 2 ; add rax, rbx   (rax should end up 3) */
    static const uint8_t code[] = {
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, /* mov rax, 1 */
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, /* mov rbx, 2 */
        0x48, 0x01, 0xd8,                         /* add rax, rbx */
    };

    char *ver = mwemu_version();
    printf("cmwemu version: %s\n", ver ? ver : "(null)");
    mwemu_free_string(ver);

    struct MwemuEmu *emu = mwemu_init64();
    if (!emu) {
        fprintf(stderr, "failed to create emulator\n");
        return 1;
    }

    mwemu_load_code_bytes(emu, code, sizeof(code));

    /* step through the three instructions */
    for (int i = 0; i < 3; i++) {
        if (!mwemu_step(emu)) {
            fprintf(stderr, "step %d failed: %s\n", i, mwemu_last_error());
            break;
        }
    }

    uint64_t rax = 0;
    if (mwemu_get_reg(emu, "rax", &rax)) {
        printf("rax = %" PRIu64 " (expected 3)\n", rax);
    } else {
        fprintf(stderr, "get_reg failed: %s\n", mwemu_last_error());
    }

    /* allocate a scratch buffer and round-trip a qword through guest memory */
    uint64_t base = 0;
    if (mwemu_alloc(emu, "scratch", 0x1000, MWEMU_PERM_RWX, &base)) {
        mwemu_write_qword(emu, base, 0xcafebabedeadbeefULL);
        uint64_t got = 0;
        mwemu_read_qword(emu, base, &got);
        printf("scratch @ 0x%" PRIx64 " holds 0x%" PRIx64 "\n", base, got);
    }

    mwemu_free_emu(emu);
    return (rax == 3) ? 0 : 1;
}
