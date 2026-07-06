use crate::emu;

pub fn GetFileType(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;

    log_red!(emu, "kernel32!GetFileType hndl:0x{:x}", hndl);

    /*
     * FILE_TYPE_CHAR 0x0002
     * FILE_TYPE_DISK 0x0001
     * FILE_TYPE_PIPE 0x0003
     * FILE_TYPE_REMOTE 0x8000
     * FILE_TYPE_UNKNOWN 0x0000
     */

    // Default to FILE_TYPE_PIPE (3) to mimic 32-bit impl or maybe FILE_TYPE_DISK?
    // 32-bit returns 3 (PIPE).
    emu.regs_mut().rax = 3;
}
