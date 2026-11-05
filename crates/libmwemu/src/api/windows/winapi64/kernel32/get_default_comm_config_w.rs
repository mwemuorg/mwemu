use crate::emu;
use crate::windows::constants;

pub fn GetDefaultCommConfigW(emu: &mut emu::Emu) {
    let lpszName = emu.regs().rcx as u32;
    let _lpCC = emu.regs().rdx as u32;
    let _lpdwSize = emu.regs().r8 as u32;

    let lpszName = emu.maps.read_wide_string(lpszName as u64);

    log_red!(emu, "kernel32!GetDefaultCommConfigW {}", lpszName);

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
