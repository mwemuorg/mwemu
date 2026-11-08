use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetFileInformationByHandle(emu: &mut emu::Emu) {
    let h_file = arg(emu, 0) as u64;
    let file_info_ptr = arg(emu, 1) as u64;

    log_red!(
        emu,
        "kernel32!GetFileInformationByHandle h:0x{:x} info:0x{:x}",
        h_file,
        file_info_ptr
    );

    if !helper::handler_exist(h_file) {
        emu.set_last_error(6); // ERROR_INVALID_HANDLE
        emu.regs_mut().rax = 0;
        return;
    }

    if file_info_ptr != 0 {
        // BY_HANDLE_FILE_INFORMATION struct
        emu.maps.write_dword(file_info_ptr, 0x80); // dwFileAttributes = NORMAL
        emu.maps
            .write_qword(file_info_ptr + 4, 0x01D0_0000_0000_0000); // ftCreationTime
        emu.maps
            .write_qword(file_info_ptr + 12, 0x01D0_0000_0000_0000); // ftLastAccessTime
        emu.maps
            .write_qword(file_info_ptr + 20, 0x01D0_0000_0000_0000); // ftLastWriteTime
        emu.maps.write_dword(file_info_ptr + 28, 0); // dwVolumeSerialNumber
        emu.maps.write_dword(file_info_ptr + 32, 0); // nFileSizeHigh
        emu.maps.write_dword(file_info_ptr + 36, 4096); // nFileSizeLow
        emu.maps.write_dword(file_info_ptr + 40, 1); // nNumberOfLinks
        emu.maps.write_dword(file_info_ptr + 44, 0); // nFileIndexHigh
        emu.maps.write_dword(file_info_ptr + 48, 1); // nFileIndexLow
    }

    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
