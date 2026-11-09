use crate::emu;

pub fn GetErrorInfo(emu: &mut emu::Emu) {
    let reserved = emu.regs().rcx;
    let pperrinfo = emu.regs().rdx;
    log_red!(
        emu,
        "oleaut32!GetErrorInfo reserved: {} pperrinfo: 0x{:x}",
        reserved,
        pperrinfo
    );
    if pperrinfo != 0 {
        emu.maps.write_qword(pperrinfo, 0); // no error info
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // S_FALSE (no error info available)
}
