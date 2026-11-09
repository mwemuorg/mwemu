use super::bcrypt_algo_to_calg;
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub(super) fn BCryptHash(emu: &mut emu::Emu) {
    let h_algorithm = arg(emu, 0) as u64;
    let _pb_secret = arg(emu, 1) as u64;
    let _cb_secret = arg(emu, 2) as usize;
    let pb_input = arg(emu, 3) as u64;
    let cb_input = arg(emu, 4) as usize;
    let pb_output = arg(emu, 5) as u64;
    let cb_output = arg(emu, 6) as usize;

    let algo_uri = helper::handler_get_uri(h_algorithm);
    let algo_name = algo_uri.strip_prefix("bcrypt://").unwrap_or(&algo_uri);
    let calg = bcrypt_algo_to_calg(algo_name);

    log_red!(
        emu,
        "bcrypt!BCryptHash algo={} in_len={} out_len={}",
        algo_name,
        cb_input,
        cb_output
    );

    if calg != 0 && pb_input != 0 && cb_input > 0 && pb_output != 0 && cb_output > 0 {
        let temp_h = helper::handler_create(&format!("bcrypthash_temp://{}", algo_name));
        crypto_state::create_hash(temp_h, calg);
        let data = emu.maps.read_bytes(pb_input, cb_input.min(0x100_0000));
        crypto_state::hash_data(temp_h, &data);
        if let Some(digest) = crypto_state::get_hash_param(temp_h, crypto_state::HP_HASHVAL) {
            let wlen = digest.len().min(cb_output);
            emu.maps.write_bytes(pb_output, &digest[..wlen]);
        }
        crypto_state::destroy_hash(temp_h);
        helper::handler_close(temp_h);
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
    stdcall_ret(emu, 7);
}
