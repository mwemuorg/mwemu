use crate::emu;
use crate::winapi::helper;

pub fn _wfopen(emu: &mut emu::Emu) {
    let fname_ptr = emu.regs().rcx;
    let mode_ptr = emu.regs().rdx;
    let fname = emu.maps.read_wide_string(fname_ptr);
    let mode = emu.maps.read_wide_string(mode_ptr);
    let hndl = helper::handler_create(&fname);
    log_red!(
        emu,
        "msvcrt!_wfopen '{}' mode:'{}' => 0x{:x}",
        fname,
        mode,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
