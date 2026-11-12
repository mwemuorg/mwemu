use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn _wfopen(emu: &mut emu::Emu) {
    let fname_ptr = WinApiAbi::arg(emu, 0);
    let mode_ptr = WinApiAbi::arg(emu, 1);
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

    emu.regs_mut().rax = hndl;
}
