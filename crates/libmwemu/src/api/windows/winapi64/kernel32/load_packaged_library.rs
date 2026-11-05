use crate::emu;
use crate::winapi::helper;

pub fn LoadPackagedLibrary(emu: &mut emu::Emu) {
    let lib_name_ptr = emu.regs().rcx;
    let reserved = emu.regs().rdx;
    let lib_name = emu.maps.read_wide_string(lib_name_ptr);
    log_red!(
        emu,
        "kernel32!LoadPackagedLibrary '{}' reserved:0x{:x}",
        lib_name,
        reserved
    );
    emu.regs_mut().rax = helper::handler_create(&format!("lib://{}", lib_name));
}
