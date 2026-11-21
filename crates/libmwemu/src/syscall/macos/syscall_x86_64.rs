use super::dispatch;
use crate::emu::Emu;

const BSD_CLASS: u64 = 2 << 24;
const SYSCALL_NUMBER_MASK: u64 = (1 << 24) - 1;

pub fn gateway(emu: &mut Emu) {
    let number = emu.regs().rax;
    let encoded = if number <= SYSCALL_NUMBER_MASK {
        BSD_CLASS | number
    } else {
        number
    };
    if !emu.call_syscall_hook(encoded) {
        return;
    }
    let r = emu.regs();
    let args = [r.rdi, r.rsi, r.rdx, r.r10, r.r8, r.r9];
    let number = if encoded & !SYSCALL_NUMBER_MASK == BSD_CLASS {
        encoded & SYSCALL_NUMBER_MASK
    } else {
        encoded
    };
    dispatch(emu, number, args);
}
