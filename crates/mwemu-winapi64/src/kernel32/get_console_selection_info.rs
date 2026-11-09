use crate::emu;

pub fn GetConsoleSelectionInfo(emu: &mut emu::Emu) {
    let info_ptr = emu.regs().rcx;
    log_red!(emu, "kernel32!GetConsoleSelectionInfo ptr:0x{:x}", info_ptr);
    if info_ptr != 0 {
        for i in 0..4u64 {
            emu.maps.write_dword(info_ptr + i * 4, 0);
        }
    }
    emu.regs_mut().rax = 1;
}
