use crate::emu;

pub fn GetFileType(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().rsp)
        .expect("kernel32!GetFileType error getting hndl param");

    log_red!(emu, "kernel32!GetFileType 0x{:x}", hndl);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 3;

    /*
     * FILE_TYPE_CHAR 0x0002
     * FILE_TYPE_DISK 0x0001
     * FILE_TYPE_PIPE 0x0003
     * FILE_TYPE_REMOTE 0x8000
     * FILE_TYPE_UNKNOWN 0x0000
     */
}
