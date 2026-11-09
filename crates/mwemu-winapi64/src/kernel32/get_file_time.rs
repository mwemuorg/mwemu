use crate::emu;

pub fn GetFileTime(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let creation_ptr = emu.regs().rdx;
    let access_ptr = emu.regs().r8;
    let write_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetFileTime h:0x{:x} creation:0x{:x} access:0x{:x} write:0x{:x}",
        h_file,
        creation_ptr,
        access_ptr,
        write_ptr
    );
    let filetime: u64 = 132514560000000000;
    if creation_ptr != 0 {
        emu.maps.write_qword(creation_ptr, filetime);
    }
    if access_ptr != 0 {
        emu.maps.write_qword(access_ptr, filetime);
    }
    if write_ptr != 0 {
        emu.maps.write_qword(write_ptr, filetime);
    }
    emu.regs_mut().rax = 1;
}
