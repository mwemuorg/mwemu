use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn LoadPackagedLibrary(emu: &mut emu::Emu) {
    let lib_name_ptr = arg(emu, 0) as u64;
    let reserved = arg(emu, 1) as u64;
    let lib_name = emu.maps.read_wide_string(lib_name_ptr);
    log_red!(
        emu,
        "kernel32!LoadPackagedLibrary '{}' reserved:0x{:x}",
        lib_name,
        reserved
    );
    emu.regs_mut().rax = helper::handler_create(&format!("lib://{}", lib_name));
    stdcall_ret(emu, 2);
}
