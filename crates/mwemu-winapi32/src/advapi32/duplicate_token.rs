use crate::emu;
use crate::winapi::helper;

pub fn DuplicateToken(emu: &mut emu::Emu) {
    let existing_token = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let imp_level = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let new_token_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let new_hndl = helper::handler_create("dup_token");
    if new_token_ptr != 0 {
        emu.maps.write_dword(new_token_ptr, new_hndl as u32);
    }
    log_red!(
        emu,
        "advapi32!DuplicateToken src:0x{:x} level:{} => 0x{:x}",
        existing_token,
        imp_level,
        new_hndl
    );
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
