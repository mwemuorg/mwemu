use crate::arch::Arch;
use crate::emu;
use crate::syscall::linux::constants::syscalls::*;
use crate::syscall::linux::mappings;
use crate::syscall::linux::unsupported;

pub(super) fn dispatch(emu: &mut emu::Emu) -> bool {
    match emu.regs().get_eax() {
        NR32_BREAK => unsupported(emu, "break"),
        NR32_BRK => mappings::brk32(emu),
        NR32_MSYNC => mappings::msync(emu, Arch::X86),
        NR32_OLD_MMAP => mappings::old_mmap(emu),
        NR32_MMAP2 => mappings::mmap(emu, Arch::X86),
        NR32_MPROTECT => mappings::mprotect(emu, Arch::X86),
        NR32_MUNMAP => mappings::munmap(emu, Arch::X86),
        _ => return false,
    }

    true
}
