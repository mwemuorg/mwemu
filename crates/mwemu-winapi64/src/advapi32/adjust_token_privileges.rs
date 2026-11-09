use crate::emu;

pub fn AdjustTokenPrivileges(emu: &mut emu::Emu) {
    let token_handle = emu.regs().rcx;
    let disable_all = emu.regs().rdx;
    let new_state = emu.regs().r8;
    let buffer_length = emu.regs().r9 as u32;
    let previous_state = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let return_length = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    log_red!(
        emu,
        "advapi32!AdjustTokenPrivileges handle=0x{:x} disableAll={}",
        token_handle,
        disable_all
    );

    emu.regs_mut().rax = 1;
}
