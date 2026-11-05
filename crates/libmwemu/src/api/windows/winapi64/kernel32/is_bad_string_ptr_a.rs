use crate::emu;

pub fn IsBadStringPtrA(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    let max_len = emu.regs().rdx;
    log_red!(emu, "kernel32!IsBadStringPtrA ptr:0x{:x} max:{}", ptr, max_len);
    let result = if ptr != 0 && emu.maps.is_mapped(ptr) { 0u64 } else { 1u64 };
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
