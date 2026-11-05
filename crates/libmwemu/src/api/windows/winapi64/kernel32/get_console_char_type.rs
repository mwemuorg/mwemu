use crate::emu;

pub fn GetConsoleCharType(emu: &mut emu::Emu) {
    let h_console = emu.regs().rcx;
    let coord = emu.regs().rdx as u32;
    let char_type_ptr = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!GetConsoleCharType h:0x{:x} coord:0x{:x} ptr:0x{:x}",
        h_console,
        coord,
        char_type_ptr
    );
    if char_type_ptr != 0 {
        emu.maps.write_dword(char_type_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
