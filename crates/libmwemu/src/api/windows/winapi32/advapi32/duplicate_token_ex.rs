use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn DuplicateTokenEx(emu: &mut emu::Emu) {
    let existing_token = WinApiAbi::arg(emu, 0);
    let desired_access = WinApiAbi::arg(emu, 1) as u32;
    let _lp_attrs = WinApiAbi::arg(emu, 2) as u32;
    let imp_level = WinApiAbi::arg(emu, 3) as u32;
    let token_type = WinApiAbi::arg(emu, 4) as u32;
    let new_token_ptr = WinApiAbi::arg(emu, 5);
    let new_hndl = helper::handler_create("dup_token_ex");
    if new_token_ptr != 0 {
        emu.maps.write_dword(new_token_ptr, new_hndl as u32);
    }
    log_red!(
        emu,
        "advapi32!DuplicateTokenEx src:0x{:x} access:0x{:x} level:{} type:{} => 0x{:x}",
        existing_token,
        desired_access,
        imp_level,
        token_type,
        new_hndl
    );
    for _ in 0..6 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
