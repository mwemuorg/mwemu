- WinAPI64 ABI must use the parameter getter instead of harcoded stack offsets every time that could lead in errors or unmaintainbility.
- Windows Drivers simulation + harness similar to Linux drivers harness
- In MacOS more tests are applied than for Linux
- end the WinAPI full-coverage kernel32 98.9%→100%, winhttp 13%→más, iphlpapi 2.6%→más, gdi32 10%→más


# For  Drivers Windows 

- mov crN, reg / mov reg, crN: add cr0, cr2, cr3, cr4, cr8 to the register file with sane defaults (cr0 = paging+protection bits, cr4 with PAE/PGE/OSXSAVE/SMEP, cr3 = a plausible kernel PML4 physical address), log a cr_read/cr_write privileged op. iced reports these as Mnemonic::Mov with a CR* register operand, so it is one branch at the top of the mov handler.
- mov drN the same way, usually only read for anti-debug.
- sgdt/sidt/sldt/str: store a 10-byte pseudo-descriptor pointing at a fake GDT/IDT in kernel data; lgdt/lidt/ltr: record and log (a driver writing the IDT is a hook primitive worth flagging).
- swapgs: swap gs_base with a kernel GS base (a fake KPCR); only matters if a driver reads gs:[...] for the current thread, which the KTHREAD fake already covers.
- invlpg, wbinvd, invd, clts, lfence-style serialisation: no-ops.
- xsetbv, rdtscp, rdpid, monitor/mwait: trivial.






