use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi32::kernel32;
use crate::winapi::winapi32::{arg, stdcall_ret};
use crate::windows::constants;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "BCryptOpenAlgorithmProvider" => BCryptOpenAlgorithmProvider(emu),
        "BCryptGenRandom" => BCryptGenRandom(emu),
        "BCryptCloseAlgorithmProvider" => BCryptCloseAlgorithmProvider(emu),
        "BCryptCreateHash" => BCryptCreateHash(emu),
        "BCryptHashData" => BCryptHashData(emu),
        "BCryptFinishHash" => BCryptFinishHash(emu),
        "BCryptDestroyHash" => BCryptDestroyHash(emu),
        "BCryptGetProperty" => BCryptGetProperty(emu),
        "BCryptDeriveKeyPBKDF2" => BCryptDeriveKeyPBKDF2(emu),
        "BCryptGenerateSymmetricKey" => BCryptGenerateSymmetricKey(emu),
        "BCryptDestroyKey" => BCryptDestroyKey(emu),
        "BCryptEncrypt" => BCryptEncrypt(emu),
        "BCryptDecrypt" => BCryptDecrypt(emu),
        "BCryptSetProperty" => BCryptSetProperty(emu),
        "BCryptImportKeyPair" => BCryptImportKeyPair(emu),
        "BCryptExportKey" => BCryptExportKey(emu),
        "BCryptSignHash" => BCryptSignHash(emu),
        "BCryptVerifySignature" => BCryptVerifySignature(emu),
        "BCryptGenerateKeyPair" => BCryptGenerateKeyPair(emu),
        "BCryptFinalizeKeyPair" => BCryptFinalizeKeyPair(emu),
        "BCryptHash" => BCryptHash(emu),
        "ProcessPrng" => ProcessPrng(emu),
        _ => {
            if emu.cfg.skip_unimplemented {
                let nparams = emu.banzai.get_params(api);
                if nparams >= 0 {
                    for _ in 0..nparams {
                        emu.stack_pop32(false);
                    }
                }
                log::warn!(
                    "banzai: skipping unimplemented API 0x{:x} {} (nparams={}) at 0x{:x}",
                    addr,
                    api,
                    nparams,
                    emu.regs().rip
                );
                emu.regs_mut().rax = 0;
                return String::new();
            }

            if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                serialization::Serialization::dump(
                    emu,
                    emu.cfg.dump_filename.as_deref().unwrap_or("mwemu_dump.bin"),
                );
            }

            log::error!("attempt to call unimplemented API 0x{:x} {}", addr, api);
            log::warn!(
                "calling unimplemented API 0x{:x} {} at 0x{:x}",
                addr,
                api,
                emu.regs().rip
            );
            return api.to_ascii_lowercase();
        }
    }
    String::new()
}

fn BCryptOpenAlgorithmProvider(emu: &mut emu::Emu) {
    let out_handle = arg(emu, 0) as u64;
    let alg_id_ptr = arg(emu, 1) as u64;
    let algo = emu.maps.read_wide_string(alg_id_ptr);
    log_red!(emu, "bcrypt!BCryptOpenAlgorithmProvider algo: {}", algo);
    if out_handle != 0 {
        let hndl = helper::handler_create(&format!("bcrypt://{}", algo));
        emu.maps.write_dword(out_handle, hndl as u32);
    }
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 4);
}

fn BCryptGenRandom(emu: &mut emu::Emu) {
    let buf = arg(emu, 1) as u64;
    let len = arg(emu, 2) as u64;
    log_red!(emu, "bcrypt!BCryptGenRandom buf: 0x{:x} len: {}", buf, len);
    for i in 0..len {
        emu.maps
            .write_byte(buf + i, (emu.pos.wrapping_add(i) & 0xff) as u8);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 4);
}

fn BCryptCloseAlgorithmProvider(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    log_red!(
        emu,
        "bcrypt!BCryptCloseAlgorithmProvider handle: 0x{:x}",
        handle
    );
    helper::handler_close(handle);
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 2);
}

fn bcrypt_algo_to_calg(name: &str) -> u32 {
    match name.to_uppercase().as_str() {
        "SHA256" => 0x0000800c,
        "SHA1" => 0x00008004,
        "MD5" => 0x00008003,
        _ => 0,
    }
}

fn BCryptCreateHash(emu: &mut emu::Emu) {
    let h_algorithm = arg(emu, 0) as u64;
    let ph_hash = arg(emu, 1) as u64;
    let _pb_hash_object = arg(emu, 2) as u64;
    let _cb_hash_object = arg(emu, 3) as u64;
    let _pb_secret = arg(emu, 4) as u64;
    let _cb_secret = arg(emu, 5) as u64;
    let _flags = arg(emu, 6) as u64;

    let algo_uri = helper::handler_get_uri(h_algorithm);
    let algo_name = algo_uri.strip_prefix("bcrypt://").unwrap_or(&algo_uri);
    let calg = bcrypt_algo_to_calg(algo_name);

    let hndl = helper::handler_create(&format!("bcrypthash://{}", algo_name));
    if calg != 0 {
        crypto_state::create_hash(hndl, calg);
    }

    if ph_hash != 0 {
        emu.maps.write_dword(ph_hash, hndl as u32);
    }

    log_red!(
        emu,
        "bcrypt!BCryptCreateHash algo={} =0x{:x}",
        algo_name,
        hndl
    );

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 7);
}

fn BCryptHashData(emu: &mut emu::Emu) {
    let h_hash = arg(emu, 0) as u64;
    let pb_input = arg(emu, 1) as u64;
    let cb_input = arg(emu, 2) as usize;
    let _flags = arg(emu, 3);

    let len = cb_input.min(0x100_0000);
    if pb_input != 0 && len > 0 {
        let data = emu.maps.read_bytes(pb_input, len);
        crypto_state::hash_data(h_hash, data);
    }

    log_red!(
        emu,
        "bcrypt!BCryptHashData hash=0x{:x} len={}",
        h_hash,
        cb_input
    );

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 4);
}

fn BCryptFinishHash(emu: &mut emu::Emu) {
    let h_hash = arg(emu, 0) as u64;
    let pb_output = arg(emu, 1) as u64;
    let cb_output = arg(emu, 2) as usize;
    let _flags = arg(emu, 3);

    if let Some(digest) = crypto_state::get_hash_param(h_hash, crypto_state::HP_HASHVAL) {
        if pb_output != 0 {
            let write_len = digest.len().min(cb_output);
            emu.maps.write_bytes(pb_output, &digest[..write_len]);
        }
        log_red!(
            emu,
            "bcrypt!BCryptFinishHash hash=0x{:x} digest_len={}",
            h_hash,
            digest.len()
        );
    } else {
        log_red!(
            emu,
            "bcrypt!BCryptFinishHash hash=0x{:x} (no state)",
            h_hash
        );
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 4);
}

fn BCryptDestroyHash(emu: &mut emu::Emu) {
    let h_hash = arg(emu, 0) as u64;

    log_red!(emu, "bcrypt!BCryptDestroyHash hash=0x{:x}", h_hash);

    crypto_state::destroy_hash(h_hash);
    helper::handler_close(h_hash);
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 1);
}

fn BCryptGetProperty(emu: &mut emu::Emu) {
    let h_object = arg(emu, 0) as u64;
    let psz_property = arg(emu, 1) as u64;
    let pb_output = arg(emu, 2) as u64;
    let cb_output = arg(emu, 3) as usize;
    let pcb_result = arg(emu, 4) as u64;
    let _flags = arg(emu, 5);

    let prop_name = emu.maps.read_wide_string(psz_property);

    let algo_uri = helper::handler_get_uri(h_object);
    let algo_name = algo_uri
        .strip_prefix("bcrypt://")
        .or_else(|| algo_uri.strip_prefix("bcrypthash://"))
        .unwrap_or(&algo_uri);

    let value: u32 = match prop_name.as_str() {
        "HashDigestLength" => match algo_name.to_uppercase().as_str() {
            "SHA256" => 32,
            "SHA1" => 20,
            "MD5" => 16,
            _ => 0,
        },
        "ObjectLength" => 128,
        "BlockLength" => match algo_name.to_uppercase().as_str() {
            "SHA256" => 64,
            "SHA1" => 64,
            "MD5" => 64,
            _ => 8,
        },
        _ => 0,
    };

    if pb_output != 0 && cb_output >= 4 {
        emu.maps.write_dword(pb_output, value);
    }
    if pcb_result != 0 {
        emu.maps.write_dword(pcb_result, 4);
    }

    log_red!(
        emu,
        "bcrypt!BCryptGetProperty obj=0x{:x} prop={} val={}",
        h_object,
        prop_name,
        value
    );

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 6);
}

fn BCryptDeriveKeyPBKDF2(emu: &mut emu::Emu) {
    let h_prf = arg(emu, 0) as u64;
    let pb_password = arg(emu, 1) as u64;
    let cb_password = arg(emu, 2) as usize;
    let pb_salt = arg(emu, 3) as u64;
    let cb_salt = arg(emu, 4) as usize;
    let iterations_lo = arg(emu, 5) as u64;
    let _iterations_hi = arg(emu, 6) as u64;
    let pb_derived_key = arg(emu, 7) as u64;
    let cb_derived_key = arg(emu, 8) as usize;
    let _flags = arg(emu, 9);

    log_red!(
        emu,
        "bcrypt!BCryptDeriveKeyPBKDF2 prf=0x{:x} passlen={} saltlen={} iters={} outlen={}",
        h_prf,
        cb_password,
        cb_salt,
        iterations_lo,
        cb_derived_key
    );

    if pb_derived_key != 0 && cb_derived_key > 0 {
        let password = if pb_password != 0 && cb_password > 0 {
            emu.maps
                .read_bytes(pb_password, cb_password.min(256))
                .to_vec()
        } else {
            vec![]
        };
        let salt = if pb_salt != 0 && cb_salt > 0 {
            emu.maps.read_bytes(pb_salt, cb_salt.min(256)).to_vec()
        } else {
            vec![]
        };
        let mut derived = vec![0u8; cb_derived_key];
        for (i, byte) in derived.iter_mut().enumerate() {
            let idx = i as u8;
            let p = *password.get(i % password.len().max(1)).unwrap_or(&0);
            let s = *salt.get(i % salt.len().max(1)).unwrap_or(&0);
            *byte = idx ^ p ^ s;
        }
        emu.maps.write_bytes(pb_derived_key, &derived);
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 10);
}

fn BCryptGenerateSymmetricKey(emu: &mut emu::Emu) {
    let h_algorithm = arg(emu, 0) as u64;
    let ph_key = arg(emu, 1) as u64;
    let _pb_key_object = arg(emu, 2) as u64;
    let _cb_key_object = arg(emu, 3) as u64;
    let pb_secret = arg(emu, 4) as u64;
    let cb_secret = arg(emu, 5) as usize;
    let _flags = arg(emu, 6);

    let algo_uri = helper::handler_get_uri(h_algorithm);
    let algo_name = algo_uri.strip_prefix("bcrypt://").unwrap_or(&algo_uri);
    let hndl = helper::handler_create(&format!("bcryptkey://{}", algo_name));

    let calg = match algo_name.to_uppercase().as_str() {
        "AES" => 0x00006611,
        "RC4" => 0x00006801,
        "3DES" => 0x00006603,
        _ => 0,
    };
    let key_bits = (cb_secret * 8) as u32;
    if calg != 0 {
        crypto_state::create_key(hndl, calg, key_bits);
    }
    if pb_secret != 0 && cb_secret > 0 {
        let key_data = emu.maps.read_bytes(pb_secret, cb_secret.min(256)).to_vec();
        crypto_state::set_key_data(hndl, &key_data);
    }

    if ph_key != 0 {
        emu.maps.write_dword(ph_key, hndl as u32);
    }

    log_red!(
        emu,
        "bcrypt!BCryptGenerateSymmetricKey algo={} key_len={} =0x{:x}",
        algo_name,
        cb_secret,
        hndl
    );

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 7);
}

fn BCryptDestroyKey(emu: &mut emu::Emu) {
    let h_key = arg(emu, 0) as u64;
    log_red!(emu, "bcrypt!BCryptDestroyKey key=0x{:x}", h_key);
    crypto_state::destroy_key(h_key);
    helper::handler_close(h_key);
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 1);
}

fn BCryptEncrypt(emu: &mut emu::Emu) {
    let h_key = arg(emu, 0) as u64;
    let pb_input = arg(emu, 1) as u64;
    let cb_input = arg(emu, 2) as usize;
    let _pb_padding = arg(emu, 3) as u64;
    let _pb_iv = arg(emu, 4) as u64;
    let _cb_iv = arg(emu, 5) as usize;
    let pb_output = arg(emu, 6) as u64;
    let cb_output = arg(emu, 7) as usize;
    let pcb_result = arg(emu, 8) as u64;
    let _flags = arg(emu, 9);

    log_red!(
        emu,
        "bcrypt!BCryptEncrypt key=0x{:x} in_len={} out_buf=0x{:x}",
        h_key,
        cb_input,
        pb_output
    );

    if pb_input != 0 && cb_input > 0 && pb_output != 0 && cb_output >= cb_input {
        let mut data = emu
            .maps
            .read_bytes(pb_input, cb_input.min(0x100_0000))
            .to_vec();
        crypto_state::encrypt_data(h_key, &mut data);
        emu.maps.write_bytes(pb_output, &data);
        if pcb_result != 0 {
            emu.maps.write_dword(pcb_result, data.len() as u32);
        }
    } else if pb_output == 0 && pcb_result != 0 {
        emu.maps.write_dword(pcb_result, cb_input as u32);
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 10);
}

fn BCryptDecrypt(emu: &mut emu::Emu) {
    let h_key = arg(emu, 0) as u64;
    let pb_input = arg(emu, 1) as u64;
    let cb_input = arg(emu, 2) as usize;
    let _pb_padding = arg(emu, 3) as u64;
    let _pb_iv = arg(emu, 4) as u64;
    let _cb_iv = arg(emu, 5) as usize;
    let pb_output = arg(emu, 6) as u64;
    let cb_output = arg(emu, 7) as usize;
    let pcb_result = arg(emu, 8) as u64;
    let _flags = arg(emu, 9);

    log_red!(
        emu,
        "bcrypt!BCryptDecrypt key=0x{:x} in_len={} out_buf=0x{:x}",
        h_key,
        cb_input,
        pb_output
    );

    if pb_input != 0 && cb_input > 0 && pb_output != 0 && cb_output >= cb_input {
        let mut data = emu
            .maps
            .read_bytes(pb_input, cb_input.min(0x100_0000))
            .to_vec();
        crypto_state::decrypt_data(h_key, &mut data);
        emu.maps.write_bytes(pb_output, &data);
        if pcb_result != 0 {
            emu.maps.write_dword(pcb_result, data.len() as u32);
        }
    } else if pb_output == 0 && pcb_result != 0 {
        emu.maps.write_dword(pcb_result, cb_input as u32);
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 10);
}

fn BCryptSetProperty(emu: &mut emu::Emu) {
    let h_object = arg(emu, 0) as u64;
    let psz_property = arg(emu, 1) as u64;
    let prop = emu.maps.read_wide_string(psz_property);
    log_red!(
        emu,
        "bcrypt!BCryptSetProperty obj=0x{:x} prop={}",
        h_object,
        prop
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 4);
}

fn BCryptImportKeyPair(emu: &mut emu::Emu) {
    let h_algorithm = arg(emu, 0) as u64;
    let _h_import_key = arg(emu, 1) as u64;
    let _psz_blob_type = arg(emu, 2) as u64;
    let ph_key = arg(emu, 3) as u64;

    let algo_uri = helper::handler_get_uri(h_algorithm);
    let algo_name = algo_uri.strip_prefix("bcrypt://").unwrap_or(&algo_uri);
    let hndl = helper::handler_create(&format!("bcryptkey://{}", algo_name));

    if ph_key != 0 {
        emu.maps.write_dword(ph_key, hndl as u32);
    }

    log_red!(
        emu,
        "bcrypt!BCryptImportKeyPair algo={} =0x{:x}",
        algo_name,
        hndl
    );

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 6);
}

fn BCryptExportKey(emu: &mut emu::Emu) {
    let _h_key = arg(emu, 0) as u64;
    let _h_export_key = arg(emu, 1) as u64;
    let psz_blob_type = arg(emu, 2) as u64;
    let pb_output = arg(emu, 3) as u64;
    let cb_output = arg(emu, 4) as usize;
    let pcb_result = arg(emu, 5) as u64;

    let blob_type = emu.maps.read_wide_string(psz_blob_type);
    log_red!(emu, "bcrypt!BCryptExportKey type={}", blob_type);

    if pb_output != 0 && cb_output > 0 {
        emu.maps.memset(pb_output, 0, cb_output.min(1024));
    }
    if pcb_result != 0 {
        emu.maps.write_dword(pcb_result, 0);
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 6);
}

fn BCryptSignHash(emu: &mut emu::Emu) {
    let _h_key = arg(emu, 0) as u64;
    let _padding_info = arg(emu, 1) as u64;
    let _pb_input = arg(emu, 2) as u64;
    let _cb_input = arg(emu, 3) as usize;
    let pb_output = arg(emu, 4) as u64;
    let cb_output = arg(emu, 5) as usize;
    let pcb_result = arg(emu, 6) as u64;

    log_red!(emu, "bcrypt!BCryptSignHash out=0x{:x}", pb_output);

    if pb_output != 0 && cb_output > 0 {
        for i in 0..cb_output.min(256) {
            emu.maps
                .write_byte(pb_output + i as u64, (i as u8).wrapping_add(0xAA));
        }
    }
    if pcb_result != 0 {
        emu.maps.write_dword(pcb_result, cb_output.min(256) as u32);
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 7);
}

fn BCryptVerifySignature(emu: &mut emu::Emu) {
    let h_key = arg(emu, 0) as u64;
    let padding_info = arg(emu, 1) as u64;
    let pb_hash = arg(emu, 2) as u64;
    let cb_hash = arg(emu, 3);
    let pb_signature = arg(emu, 4) as u64;
    let cb_signature = arg(emu, 5);
    let flags = arg(emu, 6);
    log_red!(
        emu,
        "bcrypt!BCryptVerifySignature key:0x{:x} hash:0x{:x}({}) sig:0x{:x}({}) flags:0x{:x}",
        h_key,
        pb_hash,
        cb_hash,
        pb_signature,
        cb_signature,
        flags
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 7);
}

fn BCryptGenerateKeyPair(emu: &mut emu::Emu) {
    let h_algorithm = arg(emu, 0) as u64;
    let ph_key = arg(emu, 1) as u64;
    let length = arg(emu, 2);

    let algo_uri = helper::handler_get_uri(h_algorithm);
    let algo_name = algo_uri.strip_prefix("bcrypt://").unwrap_or(&algo_uri);
    let hndl = helper::handler_create(&format!("bcryptkeypair://{}", algo_name));

    if ph_key != 0 {
        emu.maps.write_dword(ph_key, hndl as u32);
    }

    log_red!(
        emu,
        "bcrypt!BCryptGenerateKeyPair algo={} bits={} =0x{:x}",
        algo_name,
        length,
        hndl
    );

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 4);
}

fn BCryptFinalizeKeyPair(emu: &mut emu::Emu) {
    let h_key = arg(emu, 0) as u64;
    log_red!(emu, "bcrypt!BCryptFinalizeKeyPair key=0x{:x}", h_key);
    emu.set_last_error(0);
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 2);
}

fn BCryptHash(emu: &mut emu::Emu) {
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
        crypto_state::hash_data(temp_h, data);
        if let Some(digest) = crypto_state::get_hash_param(temp_h, crypto_state::HP_HASHVAL) {
            let wlen = digest.len().min(cb_output);
            emu.maps.write_bytes(pb_output, &digest[..wlen]);
        }
        crypto_state::destroy_hash(temp_h);
        helper::handler_close(temp_h);
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 7);
}

fn ProcessPrng(emu: &mut emu::Emu) {
    let buf = arg(emu, 0) as u64;
    let len = arg(emu, 1) as u64;

    log_red!(emu, "bcrypt!ProcessPrng buf=0x{:x} len={}", buf, len);

    for i in 0..len {
        emu.maps
            .write_byte(buf + i, (emu.pos.wrapping_add(i) & 0xff) as u8);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
