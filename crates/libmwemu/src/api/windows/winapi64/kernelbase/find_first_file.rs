use crate::emu;
use crate::winapi::helper;

pub fn FindFirstFileA(emu: &mut emu::Emu) {
    let filename_ptr = emu.regs().rcx;
    let find_data = emu.regs().rdx;
    let pattern = emu.maps.read_string(filename_ptr);
    log_red!(
        emu,
        "kernelbase!FindFirstFileA pattern:'{}' data:0x{:x}",
        pattern,
        find_data
    );
    if find_data != 0 {
        emu.maps.write_dword(find_data, 0x80); // dwFileAttributes = FILE_ATTRIBUTE_NORMAL
        emu.maps.write_dword(find_data + 28, 0); // nFileSizeHigh
        emu.maps.write_dword(find_data + 32, 1024); // nFileSizeLow
        emu.maps.write_string(find_data + 44, "dummy.txt");
    }
    let hndl = helper::handler_create("findfile://search");
    emu.regs_mut().rax = hndl;
}

pub fn FindFirstFileW(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "kernelbase!FindFirstFileW arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let hndl = helper::handler_create("findfile://FindFirstFileW");
    emu.regs_mut().rax = hndl;
}
