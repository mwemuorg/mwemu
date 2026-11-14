use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn FindFirstFileA(emu: &mut emu::Emu) {
    find_first_file(emu, "FindFirstFileA", false);
}

pub fn FindFirstFileW(emu: &mut emu::Emu) {
    find_first_file(emu, "FindFirstFileW", true);
}

fn find_first_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let file_ptr = emu.regs().rcx;
    let find_data = emu.regs().rdx;
    let file = strings::read_text(emu, file_ptr, wide);
    log_red!(emu, "kernel32!{api} file: {}", file);
    let hndl = helper::handler_create(&format!("find://{}", file));
    if wide {
        if find_data != 0 {
            // WIN32_FIND_DATAW: dwFileAttributes at offset 0
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
            // cFileName at offset 44 (MAX_PATH=260 wide chars = 520 bytes)
            let fname = if let Some(pos) = file.rfind('\\') {
                file[pos + 1..].to_string()
            } else {
                file.clone()
            };
            let fname = fname.replace('*', "file.dat").replace('?', "f");
            strings::write_string(emu, find_data + 44, &fname, wide);
        }
    } else {
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
            strings::write_string(emu, find_data + 44, &fname, wide);
        }
    }

    emu.regs_mut().rax = hndl;
}
