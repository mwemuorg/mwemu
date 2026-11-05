use crate::emu;

pub fn GetConsoleProcessList(emu: &mut emu::Emu) {
    let list_ptr = emu.regs().rcx;
    let count = emu.regs().rdx as u32;
    log_red!(
        emu,
        "kernel32!GetConsoleProcessList ptr:0x{:x} count:{}",
        list_ptr,
        count
    );
    if list_ptr != 0 && count >= 1 {
        emu.maps.write_dword(list_ptr, 0x123);
    }
    emu.regs_mut().rax = 1;
}
