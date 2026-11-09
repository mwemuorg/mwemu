use crate::emu;

pub fn GetFileTime(emu: &mut emu::Emu) {
    let hfile = emu.regs().rcx;
    let creation_ptr = emu.regs().rdx;
    let access_ptr = emu.regs().r8;
    let write_ptr = emu.regs().r9;
    let filetime: u64 = 0x01D7_A000_0000_0000;
    if creation_ptr != 0 {
        emu.maps.write_qword(creation_ptr, filetime);
    }
    if access_ptr != 0 {
        emu.maps.write_qword(access_ptr, filetime);
    }
    if write_ptr != 0 {
        emu.maps.write_qword(write_ptr, filetime);
    }
    log_red!(emu, "kernelbase!GetFileTime h:0x{:x}", hfile);
    emu.regs_mut().rax = 1; // TRUE
}
