use crate::emu;

pub fn FtpGetCurrentDirectoryA(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let dir_ptr = emu.regs().rdx;
    let dir_len_ptr = emu.regs().r8;
    log_red!(emu, "wininet!FtpGetCurrentDirectoryA handle:0x{:x}", handle);
    if dir_ptr != 0 {
        emu.maps.write_string(dir_ptr, "/");
    }
    if dir_len_ptr != 0 {
        emu.maps.write_dword(dir_len_ptr, 1);
    }
    emu.regs_mut().rax = 1;
}

pub fn FtpGetCurrentDirectoryW(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let dir_ptr = emu.regs().rdx;
    let dir_len_ptr = emu.regs().r8;
    log_red!(emu, "wininet!FtpGetCurrentDirectoryW handle:0x{:x}", handle);
    if dir_ptr != 0 {
        emu.maps.write_string(dir_ptr, "/");
    }
    if dir_len_ptr != 0 {
        emu.maps.write_dword(dir_len_ptr, 1);
    }
    emu.regs_mut().rax = 1;
}
