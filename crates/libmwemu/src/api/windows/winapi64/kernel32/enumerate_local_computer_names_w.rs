use crate::emu;

pub fn EnumerateLocalComputerNamesW(emu: &mut emu::Emu) {
    let name_type = emu.regs().rcx as u32;
    let flags = emu.regs().rdx as u32;
    let buf_ptr = emu.regs().r8;
    let buf_size_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!EnumerateLocalComputerNamesW type:{} flags:0x{:x}",
        name_type,
        flags
    );
    if buf_size_ptr != 0 {
        emu.maps.write_dword(buf_size_ptr, 0);
    }
    emu.regs_mut().rax = 0;
}
