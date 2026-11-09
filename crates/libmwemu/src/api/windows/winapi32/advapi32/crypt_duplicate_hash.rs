use crate::emu;
use crate::winapi::helper;

pub fn CryptDuplicateHash(emu: &mut emu::Emu) {
    let hhash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let ph_hash = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let new_handle = helper::handler_create("crypt_hash_dup");
    log_red!(
        emu,
        "advapi32!CryptDuplicateHash src:0x{:x} => 0x{:x}",
        hhash,
        new_handle
    );
    if ph_hash != 0 {
        emu.maps.write_dword(ph_hash, new_handle as u32);
    }
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
