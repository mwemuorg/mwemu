use crate::emu;
use crate::winapi::helper;

pub fn DuplicateToken(emu: &mut emu::Emu) {
    let token = emu.regs().rcx;
    let imp_level = emu.regs().rdx;
    let out_ptr = emu.regs().r8;
    let new_token = helper::handler_create("token");
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, new_token);
    }
    log_red!(
        emu,
        "kernelbase!DuplicateToken h:0x{:x} level:{} => 0x{:x}",
        token,
        imp_level,
        new_token
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
