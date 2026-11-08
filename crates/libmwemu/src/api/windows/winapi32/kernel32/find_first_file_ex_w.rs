use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn FindFirstFileExW(emu: &mut emu::Emu) {
    let filename_ptr = arg(emu, 0) as u64;
    let _info_level = arg(emu, 1) as u64;
    let find_data_ptr = arg(emu, 2) as u64;
    let _search_op = arg(emu, 3) as u64;

    let filename = emu.maps.read_wide_string(filename_ptr);

    log_red!(emu, "kernel32!FindFirstFileExW file: {}", filename);

    let hndl = helper::handler_create(&format!("find://{}", filename));

    if find_data_ptr != 0 {
        emu.maps.write_dword(find_data_ptr, 0x80); // FILE_ATTRIBUTE_NORMAL
        emu.maps
            .write_qword(find_data_ptr + 4, 0x01D0_0000_0000_0000);
        emu.maps
            .write_qword(find_data_ptr + 12, 0x01D0_0000_0000_0000);
        emu.maps
            .write_qword(find_data_ptr + 20, 0x01D0_0000_0000_0000);
        emu.maps.write_dword(find_data_ptr + 28, 0);
        emu.maps.write_dword(find_data_ptr + 32, 4096);
        let fname = if let Some(pos) = filename.rfind('\\') {
            filename[pos + 1..].to_string()
        } else {
            filename.clone()
        };
        let fname = fname.replace('*', "file.dat").replace('?', "f");
        emu.maps.write_wide_string(find_data_ptr + 44, &fname);
    }

    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 4);
}
