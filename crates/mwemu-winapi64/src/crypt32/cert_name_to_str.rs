use crate::emu;

pub fn CertNameToStr(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let buf_ptr = emu.regs().r8;
    let buf_size = emu.regs().r9;
    log_red!(emu, "crypt32!CertNameToStr arg1:0x{:x}", arg1);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    if buf_ptr != 0 {
        emu.maps.write_string(buf_ptr, "CN=localhost");
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 13;
}
