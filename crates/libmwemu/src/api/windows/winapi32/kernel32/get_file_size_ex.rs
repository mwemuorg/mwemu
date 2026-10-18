use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

/// BOOL GetFileSizeEx(HANDLE hFile, PLARGE_INTEGER lpFileSize)
pub fn GetFileSizeEx(emu: &mut emu::Emu) {
    let handle = arg(emu, 0);
    let size_ptr = arg(emu, 1) as u64;

    let size = emu
        .handle_management
        .get_mut_file_handle(handle)
        .map(|f| f.file_size);

    log_red!(
        emu,
        "kernel32!GetFileSizeEx hndl: 0x{:x} ={:?}",
        handle,
        size
    );

    match size {
        Some(size) => {
            emu.maps.write_qword(size_ptr, size);
            emu.regs_mut().rax = 1;
        }
        None => emu.regs_mut().rax = 0,
    }
    stdcall_ret(emu, 2);
}
