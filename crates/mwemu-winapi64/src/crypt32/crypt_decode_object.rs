use crate::emu;

pub fn CryptDecodeObject(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    log_red!(emu, "crypt32!CryptDecodeObject 0x{:x} 0x{:x}", arg1, arg2);
    let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
