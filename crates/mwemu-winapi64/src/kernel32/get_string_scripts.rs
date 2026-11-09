use crate::emu;

pub fn GetStringScripts(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx as u32;
    let str_ptr = emu.regs().rdx;
    let str_len = emu.regs().r8 as i32;
    let scripts_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetStringScripts flags:0x{:x} str:0x{:x} len:{}",
        flags,
        str_ptr,
        str_len
    );
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
