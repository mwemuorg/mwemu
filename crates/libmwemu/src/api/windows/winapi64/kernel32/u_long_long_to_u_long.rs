use crate::emu;

pub fn ULongLongToULong(emu: &mut emu::Emu) {
    let val = emu.regs().rcx;
    let p_result = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!ULongLongToULong val:{} pResult:0x{:x}",
        val,
        p_result
    );

    if val > 0xFFFFFFFF {
        emu.regs_mut().rax = 0x80070216;
    } else {
        if p_result != 0 {
            emu.maps.write_dword(p_result, val as u32);
        }
        emu.regs_mut().rax = 0;
    }
}
