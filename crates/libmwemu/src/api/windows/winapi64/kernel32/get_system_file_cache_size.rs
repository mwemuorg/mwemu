use crate::emu;

pub fn GetSystemFileCacheSize(emu: &mut emu::Emu) {
    let min_ptr = emu.regs().rcx;
    let max_ptr = emu.regs().rdx;
    let flags_ptr = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!GetSystemFileCacheSize min:0x{:x} max:0x{:x} flags:0x{:x}",
        min_ptr,
        max_ptr,
        flags_ptr
    );
    if min_ptr != 0 {
        emu.maps.write_qword(min_ptr, 0x100000);
    }
    if max_ptr != 0 {
        emu.maps.write_qword(max_ptr, 0x10000000);
    }
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
