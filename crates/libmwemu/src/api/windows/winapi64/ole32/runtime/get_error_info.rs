use crate::emu;

pub fn GetErrorInfo(emu: &mut emu::Emu) {
    let _reserved = emu.regs().rcx;
    let pperrinfo = emu.regs().rdx;
    if pperrinfo != 0 {
        emu.maps.write_dword(pperrinfo, 0); // no error info
    }
    log_red!(emu, "ole32!GetErrorInfo => S_FALSE (no error info)");
    emu.regs_mut().rax = 1; // S_FALSE
}
