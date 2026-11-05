use crate::emu;

pub fn FillConsoleOutputAttribute(emu: &mut emu::Emu) {
    let h_console = emu.regs().rcx;
    let attr = emu.regs().rdx as u16;
    let length = emu.regs().r8 as u32;
    let _coord = emu.regs().r9; // COORD packed as DWORD
    let attrs_written_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    log_red!(
        emu,
        "kernel32!FillConsoleOutputAttribute h:0x{:x} attr:0x{:x} len:{}",
        h_console,
        attr,
        length
    );

    if attrs_written_ptr != 0 {
        emu.maps.write_dword(attrs_written_ptr, length);
    }
    emu.regs_mut().rax = 1;
}
