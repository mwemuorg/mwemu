use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn AdjustTokenGroups(emu: &mut emu::Emu) {
    let token = emu.regs().rcx;
    let reset_to_default = emu.regs().rdx;
    let new_state = emu.regs().r8;
    let buf_len = emu.regs().r9;
    let prev_state = WinApiAbi::arg(emu, 4);
    let ret_len = WinApiAbi::arg(emu, 5);
    log_red!(
        emu,
        "kernelbase!AdjustTokenGroups token:0x{:x} reset:{} new:0x{:x}",
        token,
        reset_to_default,
        new_state
    );
    if ret_len != 0 {
        emu.maps.write_dword(ret_len, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
