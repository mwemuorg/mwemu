use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::crypto::*;

pub fn CryptCreateHash(emu: &mut emu::Emu) {
    let hprov = WinApiAbi::arg(emu, 0) as u32;
    let algid = WinApiAbi::arg(emu, 1) as u32;
    let hkey = WinApiAbi::arg(emu, 2) as u32;
    let flags = WinApiAbi::arg(emu, 3) as u32;
    let ptr_hash = WinApiAbi::arg(emu, 4);

    let alg_name = get_cryptoalgorithm_name(algid);

    log_red!(emu, "kernel32!CryptCreateHash alg:{}", alg_name);

    for _ in 0..5 {
        emu.stack_pop32(false);
    }

    emu.maps.write_dword(
        ptr_hash,
        helper::handler_create(&format!("alg://{}", alg_name)) as u32,
    );
    emu.regs_mut().rax = 1;
}
