use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetFileInformationByName(emu: &mut emu::Emu) {
    let file_name_ptr = arg(emu, 0) as u64;
    let info_class = arg(emu, 1);
    let info_ptr = arg(emu, 2) as u64;
    let info_size = arg(emu, 3);
    let file_name = emu.maps.read_wide_string(file_name_ptr);
    log_red!(
        emu,
        "kernel32!GetFileInformationByName '{}' class:{}",
        file_name,
        info_class
    );
    emu.regs_mut().rax = 0;
    emu.set_last_error(2);
    stdcall_ret(emu, 4);
}
