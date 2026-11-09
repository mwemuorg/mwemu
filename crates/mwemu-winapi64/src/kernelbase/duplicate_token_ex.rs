use crate::emu;
use crate::winapi::helper;

pub fn DuplicateTokenEx(emu: &mut emu::Emu) {
    let token = emu.regs().rcx;
    let desired_access = emu.regs().rdx;
    let _security_attr = emu.regs().r8;
    let imp_level = emu.regs().r9;
    let token_type = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let new_token = helper::handler_create("token");
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, new_token);
    }
    log_red!(
        emu,
        "kernelbase!DuplicateTokenEx h:0x{:x} access:0x{:x} level:{} type:{} => 0x{:x}",
        token,
        desired_access,
        imp_level,
        token_type,
        new_token
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
