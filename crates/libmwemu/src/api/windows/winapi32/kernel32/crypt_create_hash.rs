use crate::emu;
use crate::winapi::helper;
use crate::windows::constants;

pub fn CryptCreateHash(emu: &mut emu::Emu) {
    let hprov = emu.maps.read_dword(emu.regs().rsp).unwrap_or(0);
    let algid = emu.maps.read_dword(emu.regs().rsp + 4).unwrap_or(0);
    let hkey = emu.maps.read_dword(emu.regs().rsp + 8).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().rsp + 12).unwrap_or(0);
    let ptr_hash = emu.maps.read_dword(emu.regs().rsp + 16).unwrap_or(0) as u64;

    let alg_name = constants::get_cryptoalgorithm_name(algid);

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
