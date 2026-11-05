use crate::emu;
use crate::winapi::helper;

pub fn FindFirstFileA(emu: &mut emu::Emu) {
    let file_ptr = emu.regs().rcx;
    let find_data = emu.regs().rdx;

    let file = emu.maps.read_string(file_ptr);

    log_red!(emu, "kernel32!FindFirstFileA file: {}", file);

    let hndl = helper::handler_create(&format!("find://{}", file));

    if find_data != 0 {
        // WIN32_FIND_DATAA: dwFileAttributes at offset 0
        emu.maps.write_dword(find_data, 0x80); // FILE_ATTRIBUTE_NORMAL
        // ftCreationTime (offset 4, 8 bytes)
        emu.maps.write_qword(find_data + 4, 0x01D0_0000_0000_0000);
        // ftLastAccessTime (offset 12, 8 bytes)
        emu.maps.write_qword(find_data + 12, 0x01D0_0000_0000_0000);
        // ftLastWriteTime (offset 20, 8 bytes)
        emu.maps.write_qword(find_data + 20, 0x01D0_0000_0000_0000);
        // nFileSizeHigh (offset 28)
        emu.maps.write_dword(find_data + 28, 0);
        // nFileSizeLow (offset 32)
        emu.maps.write_dword(find_data + 32, 4096);
        // cFileName at offset 44 (MAX_PATH=260 chars)
        let fname = if let Some(pos) = file.rfind('\\') {
            &file[pos + 1..]
        } else {
            &file
        };
        let fname = fname.replace('*', "file.dat").replace('?', "f");
        emu.maps.write_string(find_data + 44, &fname);
    }

    emu.regs_mut().rax = hndl;
}
