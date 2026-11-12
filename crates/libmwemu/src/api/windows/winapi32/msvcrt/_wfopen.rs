use crate::emu;
use crate::winapi::helper;

pub fn _wfopen(emu: &mut emu::Emu) {
    let fname_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let mode_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
