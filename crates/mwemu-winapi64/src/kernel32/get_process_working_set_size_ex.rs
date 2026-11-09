use crate::emu;

pub fn GetProcessWorkingSetSizeEx(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let min_ptr = emu.regs().rdx;
    let max_ptr = emu.regs().r8;
    let flags_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetProcessWorkingSetSizeEx h:0x{:x}",
        h_process
    );
    if min_ptr != 0 {
        emu.maps.write_qword(min_ptr, 0x20000);
    }
    if max_ptr != 0 {
        emu.maps.write_qword(max_ptr, 0x100000);
    }
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
