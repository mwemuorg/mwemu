use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CryptDuplicateHash(emu: &mut emu::Emu) {
    let hhash = WinApiAbi::arg(emu, 0);
    let _reserved = WinApiAbi::arg(emu, 1) as u32;
    let _flags = WinApiAbi::arg(emu, 2) as u32;
    let ph_hash = WinApiAbi::arg(emu, 3);
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
