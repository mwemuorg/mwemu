use crate::emu;

pub fn CertGetNameString(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let buf_ptr = emu.regs().r8;
    let buf_size = emu.regs().r9;
    log_red!(emu, "crypt32!CertGetNameString arg1:0x{:x}", arg1);
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    if buf_ptr != 0 {
        emu.maps.write_string(buf_ptr, "CN=localhost");
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 13;
}
