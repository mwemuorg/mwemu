use crate::emu;
use crate::winapi::helper;

pub fn GetFileType(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;

    let file_type = if helper::handler_exist(hndl) {
        let uri = helper::handler_get_uri(hndl);
        if uri.contains("file://") || uri.contains("find://") {
            1 // FILE_TYPE_DISK
        } else if uri.contains("pipe://") || uri.contains("named_pipe://") {
            3 // FILE_TYPE_PIPE
        } else if uri.contains("console")
            || uri.contains("stdout")
            || uri.contains("stderr")
            || uri.contains("stdin")
        {
            2 // FILE_TYPE_CHAR
        } else {
            1 // FILE_TYPE_DISK default for handlers
        }
    } else if hndl == 0xfffffff5 || hndl == 0xfffffff6 || hndl == 0xfffffff4 {
        2 // FILE_TYPE_CHAR for std handles
    } else {
        0 // FILE_TYPE_UNKNOWN
    };

    log_red!(
        emu,
        "kernel32!GetFileType hndl:0x{:x} => {}",
        hndl,
        file_type
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = file_type;
}
