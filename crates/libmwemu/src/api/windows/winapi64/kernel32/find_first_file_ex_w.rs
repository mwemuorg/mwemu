use crate::emu;

pub fn FindFirstFileExW(emu: &mut emu::Emu) {
    let filename_ptr = emu.regs().rcx;
    let info_level = emu.regs().rdx;
    let find_data_ptr = emu.regs().r8;
    let search_op = emu.regs().r9;
    let search_filter = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("kernel32!FindFirstFileExW cannot read_qword search_filter");
    let additional_flags = emu
        .maps
        .read_qword(emu.regs().rsp + 0x28)
        .expect("kernel32!FindFirstFileExW cannot read_qword additional_flags");

    let filename = emu.maps.read_wide_string(filename_ptr);

    // TODO: Read wide string filename from filename_ptr
    // TODO: Parse info_level (FindExInfoStandard=0, FindExInfoBasic=1, FindExInfoMaxInfoLevel=2)
    // TODO: Parse search_op (FindExSearchNameMatch=0, FindExSearchLimitToDirectories=1, FindExSearchLimitToDevices=2)
    // TODO: Read lpSearchFilter from stack+0x28
    // TODO: Read dwAdditionalFlags from stack+0x30

    // TODO: Check if file/pattern exists in emulated filesystem
    // TODO: Fill WIN32_FIND_DATAW structure at find_data_ptr:
    //   - dwFileAttributes (DWORD)
    //   - ftCreationTime (FILETIME)
    //   - ftLastAccessTime (FILETIME)
    //   - ftLastWriteTime (FILETIME)
    //   - nFileSizeHigh (DWORD)
    //   - nFileSizeLow (DWORD)
    //   - dwReserved0, dwReserved1 (DWORD)
    //   - cFileName[MAX_PATH] (WCHAR array)
    //   - cAlternateFileName[14] (WCHAR array)

    log_red!(
        emu,
        "kernel32!FindFirstFileExW filename_ptr: 0x{:x} filename: {} info_level: {} find_data_ptr: 0x{:x} search_op: {} search_filter: {} additional_flags: {}",
        filename_ptr,
        filename,
        info_level,
        find_data_ptr,
        search_op,
        search_filter,
        additional_flags
    );

    // TODO: Return valid HANDLE (not INVALID_HANDLE_VALUE = -1) on success
    // TODO: Set appropriate error code with SetLastError on failure
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF; // INVALID_HANDLE_VALUE for now
}
