use crate::emu;

pub fn CoIsOle1Class(emu: &mut emu::Emu) {
    let clsid_ptr = emu.regs().rcx;
    log_red!(emu, "ole32!CoIsOle1Class clsid:0x{:x}", clsid_ptr);
    if clsid_ptr != 0 && emu.maps.is_mapped(clsid_ptr) {
        let _ = emu.maps.read_dword(clsid_ptr).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
