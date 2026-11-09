use crate::emu;

pub fn AdjustTokenPrivileges(emu: &mut emu::Emu) {
    let token = emu.regs().rcx;
    let disable_all = emu.regs().rdx;
    let new_state = emu.regs().r8;
    let buf_len = emu.regs().r9;
    let prev_state = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let ret_len = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    log_red!(
        emu,
        "kernelbase!AdjustTokenPrivileges token:0x{:x} disable_all:{} new:0x{:x}",
        token,
        disable_all,
        new_state
    );
    if ret_len != 0 {
        emu.maps.write_dword(ret_len, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
