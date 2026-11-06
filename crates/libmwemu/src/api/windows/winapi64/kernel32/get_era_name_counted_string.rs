use crate::emu;

pub fn GetEraNameCountedString(emu: &mut emu::Emu) {
    let locale_ptr = emu.regs().rcx;
    let calendar = emu.regs().rdx as u32;
    let era = emu.regs().r8 as u32;
    let result_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetEraNameCountedString cal:{} era:{} result:0x{:x}",
        calendar,
        era,
        result_ptr
    );
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
