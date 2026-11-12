use super::bcrypt_algo_to_calg;
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn BCryptCreateHash(emu: &mut emu::Emu) {
    let h_algorithm = emu.regs().rcx;
    let ph_hash = emu.regs().rdx;
    let pb_hash_object = emu.regs().r8;
    let cb_hash_object = emu.regs().r9;
    let pb_secret = WinApiAbi::arg(emu, 4);
    let cb_secret = WinApiAbi::arg(emu, 5);
    let flags = WinApiAbi::arg(emu, 6);

    let algo_uri = helper::handler_get_uri(h_algorithm);
    let algo_name = algo_uri.strip_prefix("bcrypt://").unwrap_or(&algo_uri);
    let calg = bcrypt_algo_to_calg(algo_name);

    let hndl = helper::handler_create(&format!("bcrypthash://{}", algo_name));
    if calg != 0 {
        crypto_state::create_hash(hndl, calg);
    }

    if ph_hash != 0 {
        emu.maps.write_qword(ph_hash, hndl);
    }

    log_red!(
        emu,
        "bcrypt!BCryptCreateHash algo={} =0x{:x}",
        algo_name,
        hndl
    );

    emu.regs_mut().rax = STATUS_SUCCESS;
}
