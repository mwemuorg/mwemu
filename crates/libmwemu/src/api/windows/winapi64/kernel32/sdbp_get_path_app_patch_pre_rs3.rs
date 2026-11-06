use crate::emu;
use crate::winapi::helper;

pub fn SdbpGetPathAppPatchPreRS3(emu: &mut emu::Emu) {
    let buffer = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!SdbpGetPathAppPatchPreRS3 buffer:0x{:x}",
        buffer
    );
    let _h = helper::handler_create("sdb_path");

    if buffer != 0 {
        emu.maps.write_wide_string(buffer, "C:\\Windows\\AppPatch");
    }
    emu.regs_mut().rax = 1;
}
