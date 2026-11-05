use crate::emu;

pub fn GetStateFolder(emu: &mut emu::Emu) {
    let state_type = emu.regs().rcx as u32;
    let buf_ptr = emu.regs().rdx;
    let buf_len_ptr = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!GetStateFolder type:{} buf:0x{:x} len:0x{:x}",
        state_type,
        buf_ptr,
        buf_len_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x80070057u64;
}
