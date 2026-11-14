use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn AdjustTokenPrivileges(emu: &mut emu::Emu) {
    let token_handle = emu.regs().rcx;
    let disable_all = emu.regs().rdx;
    let new_state = emu.regs().r8;
    let buffer_length = emu.regs().r9 as u32;
    let previous_state = WinApiAbi::arg(emu, 4);
    let return_length = WinApiAbi::arg(emu, 5);

    log_red!(
        emu,
        "advapi32!AdjustTokenPrivileges handle=0x{:x} disableAll={}",
        token_handle,
        disable_all
    );

    emu.regs_mut().rax = 1;
}
