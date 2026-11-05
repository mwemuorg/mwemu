use crate::emu;

pub fn RtlVirtualUnwind2(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    let param2 = emu.regs().rdx;
    let param3 = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!RtlVirtualUnwind2 param1:0x{:x} param2:0x{:x} param3:0x{:x}",
        param1,
        param2,
        param3
    );
    emu.set_last_error(0);
}
