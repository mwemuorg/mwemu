use crate::emu;

pub fn GetCurrentThreadStackLimits(emu: &mut emu::Emu) {
    let low_limit_ptr = emu.regs().rcx;
    let high_limit_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetCurrentThreadStackLimits low:0x{:x} high:0x{:x}",
        low_limit_ptr,
        high_limit_ptr
    );
    let stack_base = emu.regs().rsp & 0xFFFFFFFFFFFF0000;
    if low_limit_ptr != 0 {
        emu.maps
            .write_qword(low_limit_ptr, stack_base.saturating_sub(0x100000));
    }
    if high_limit_ptr != 0 {
        emu.maps.write_qword(high_limit_ptr, stack_base + 0x10000);
    }
}
