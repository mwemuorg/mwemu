use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn GetThreadPreferredUILanguages(emu: &mut emu::Emu) {
    let flags = WinApiAbi::arg(emu, 0) as u32;
    let num_langs_ptr = WinApiAbi::arg(emu, 1);
    let buff = WinApiAbi::arg(emu, 2);
    let out_sz = WinApiAbi::arg(emu, 3);

    emu.maps.write_dword(num_langs_ptr, 0);
    log_red!(emu, "kernel32!GetThreadPreferredUILanguages");

    emu.maps.write_dword(out_sz, 0);
    emu.maps.write_dword(num_langs_ptr, 0);

    for _ in 0..4 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 1;
}
