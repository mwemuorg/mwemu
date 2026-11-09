use crate::emu;
use crate::winapi::helper;

pub fn DuplicateTokenEx(emu: &mut emu::Emu) {
    let existing_token = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let desired_access = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lp_attrs = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let imp_level = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let token_type = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let new_token_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
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
