use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn BCryptDeriveKeyPBKDF2(emu: &mut emu::Emu) {
    let h_prf = emu.regs().rcx;
    let pb_password = emu.regs().rdx;
    let cb_password = emu.regs().r8 as usize;
    let pb_salt = emu.regs().r9;
    let cb_salt = WinApiAbi::arg(emu, 4) as usize;
    let iterations = WinApiAbi::arg(emu, 5);
    let pb_derived_key = WinApiAbi::arg(emu, 6);
    let cb_derived_key = WinApiAbi::arg(emu, 7) as usize;

    if pb_derived_key != 0 && cb_derived_key > 0 {
        let pwd = if pb_password != 0 && cb_password > 0 {
            emu.maps
                .read_bytes(pb_password, cb_password.min(4096))
                .to_vec()
        } else {
            vec![]
        };
        let salt = if pb_salt != 0 && cb_salt > 0 {
            emu.maps.read_bytes(pb_salt, cb_salt.min(4096)).to_vec()
        } else {
            vec![]
        };
        let uri = helper::handler_get_uri(h_prf);
        let alg_id = if uri.contains("SHA1") || uri.contains("sha1") {
            0x00008004u32
        } else {
            0x0000800cu32
        };
        let mut key = vec![0u8; cb_derived_key];
        crypto_state::pbkdf2_derive(alg_id, &pwd, &salt, iterations as u32, &mut key);
        emu.maps.write_bytes(pb_derived_key, &key);
    }

    log_red!(
        emu,
        "bcrypt!BCryptDeriveKeyPBKDF2 pwd_len={} salt_len={} iters={} key_len={}",
        cb_password,
        cb_salt,
        iterations,
        cb_derived_key
    );

    emu.regs_mut().rax = STATUS_SUCCESS;
}
