use super::bcrypt_algo_to_calg;
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::windows::constants;

pub fn BCryptCreateHash(emu: &mut emu::Emu) {
    let h_algorithm = emu.regs().rcx;
    let ph_hash = emu.regs().rdx;
    let pb_hash_object = emu.regs().r8;
    let cb_hash_object = emu.regs().r9;
    let pb_secret = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let cb_secret = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);

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

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}
