# drivers/

Kernel-mode targets used to develop and test mwemu's driver emulation.

These are **not** meant to be loaded into a real kernel. They contain
deliberate memory-safety bugs, and their whole purpose is to be linked and
executed inside mwemu, where a use-after-free produces a report instead of a
crashed machine.

| target | OS | what it exercises |
| --- | --- | --- |
| [`linux/tlm`](linux/tlm) | Linux (`.ko`) | refcounted slab objects, a stale hot-path cache, use-after-free |

Build the Linux target with `make driver` from the repo root; it lands in
`test/linux_uaf_driver.ko`, where the kernel-emulation tests look for it.
