use crate::api::windows::common::crypto_state;
use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi64::kernel32;
use crate::windows::constants;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
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
        "ProcessPrng" => {
            let pb_data = emu.regs().rcx;
            let cb_data = emu.regs().rdx;
            log_red!(
                emu,
                "bcrypt!ProcessPrng buf: 0x{:x} len: {}",
                pb_data,
                cb_data
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for i in 0..cb_data {
                let byte = ((emu.pos.wrapping_add(i)) & 0xFF) as u8;
                emu.maps.write_byte(pb_data + i, byte);
            }
            emu.regs_mut().rax = 1; // BOOL TRUE
        }
        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}

pub fn BCryptOpenAlgorithmProvider(emu: &mut emu::Emu) {
    let ph_algorithm = emu.regs().rcx;
    let psz_alg_id = emu.regs().rdx;
    let flags = emu.regs().r9;

    let algo_name = emu.maps.read_wide_string(psz_alg_id);

    log_red!(
        emu,
        "bcrypt!BCryptOpenAlgorithmProvider algo: {} flags: 0x{:x}",
        algo_name,
        flags
    );

    let handle = helper::handler_create(&format!("bcrypt://{}", algo_name));
    if ph_algorithm != 0 {
        emu.maps.write_qword(ph_algorithm, handle);
    }
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

pub fn BCryptGenRandom(emu: &mut emu::Emu) {
    let pb_buffer = emu.regs().rdx;
    let cb_buffer = emu.regs().r8;
    let flags = emu.regs().r9;

    log_red!(
        emu,
        "bcrypt!BCryptGenRandom buf: 0x{:x} len: {} flags: 0x{:x}",
        pb_buffer,
        cb_buffer,
        flags
    );

    for i in 0..cb_buffer {
        let byte = ((emu.pos.wrapping_add(i)) & 0xFF) as u8;
        emu.maps.write_byte(pb_buffer + i, byte);
    }
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

pub fn BCryptCloseAlgorithmProvider(emu: &mut emu::Emu) {
    let h_algorithm = emu.regs().rcx;
    let flags = emu.regs().rdx;

    log_red!(
        emu,
        "bcrypt!BCryptCloseAlgorithmProvider handle: 0x{:x} flags: 0x{:x}",
        h_algorithm,
        flags
    );

    helper::handler_close(h_algorithm);
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn bcrypt_algo_to_calg(name: &str) -> u32 {
    match name.to_uppercase().as_str() {
        "SHA256" => 0x0000800c,
        "SHA1" => 0x00008004,
        "MD5" => 0x00008003,
        _ => 0,
    }
}

pub fn BCryptCreateHash(emu: &mut emu::Emu) {
    let h_algorithm = emu.regs().rcx;
    let ph_hash = emu.regs().rdx;
    let pb_hash_object = emu.regs().r8;
    let cb_hash_object = emu.regs().r9;
    let pb_secret = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let cb_secret = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);

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

pub fn BCryptHashData(emu: &mut emu::Emu) {
    let h_hash = emu.regs().rcx;
    let pb_input = emu.regs().rdx;
    let cb_input = emu.regs().r8 as usize;
    let flags = emu.regs().r9;

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
}

pub fn BCryptFinishHash(emu: &mut emu::Emu) {
    let h_hash = emu.regs().rcx;
    let pb_output = emu.regs().rdx;
    let cb_output = emu.regs().r8 as usize;
    let flags = emu.regs().r9;

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
}

pub fn BCryptDestroyHash(emu: &mut emu::Emu) {
    let h_hash = emu.regs().rcx;

    log_red!(emu, "bcrypt!BCryptDestroyHash hash=0x{:x}", h_hash);

    crypto_state::destroy_hash(h_hash);
    helper::handler_close(h_hash);
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

pub fn BCryptGetProperty(emu: &mut emu::Emu) {
    let h_object = emu.regs().rcx;
    let psz_property = emu.regs().rdx;
    let pb_output = emu.regs().r8;
    let cb_output = emu.regs().r9 as usize;
    let pcb_result = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);

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
        "bcrypt!BCryptGetProperty obj=0x{:x} prop={} => {}",
        h_object,
        prop_name,
        value
    );

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

pub fn BCryptDeriveKeyPBKDF2(emu: &mut emu::Emu) {
    let h_prf = emu.regs().rcx;
    let pb_password = emu.regs().rdx;
    let cb_password = emu.regs().r8 as usize;
    let pb_salt = emu.regs().r9;
    let cb_salt = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as usize;
    let iterations = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(1);
    let pb_derived_key = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    let cb_derived_key = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0) as usize;

    if pb_derived_key != 0 && cb_derived_key > 0 {
        let pwd = if pb_password != 0 && cb_password > 0 {
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
        let mut key = vec![0u8; cb_derived_key];
        for (i, b) in key.iter_mut().enumerate() {
            let pi = if !pwd.is_empty() {
                pwd[i % pwd.len()]
            } else {
                0x42
            };
            let si = if !salt.is_empty() {
                salt[i % salt.len()]
            } else {
                0x37
            };
            *b = pi ^ si ^ (i as u8);
        }
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

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

pub fn BCryptGenerateSymmetricKey(emu: &mut emu::Emu) {
    let h_algorithm = emu.regs().rcx;
    let ph_key = emu.regs().rdx;
    let _pb_key_object = emu.regs().r8;
    let _cb_key_object = emu.regs().r9;
    let pb_secret = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let cb_secret = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as usize;
    let _flags = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);

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
        emu.maps.write_qword(ph_key, hndl);
    }

    log_red!(
        emu,
        "bcrypt!BCryptGenerateSymmetricKey algo={} key_len={} =0x{:x}",
        algo_name,
        cb_secret,
        hndl
    );

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

pub fn BCryptDestroyKey(emu: &mut emu::Emu) {
    let h_key = emu.regs().rcx;

    log_red!(emu, "bcrypt!BCryptDestroyKey key=0x{:x}", h_key);

    crypto_state::destroy_key(h_key);
    helper::handler_close(h_key);
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

pub fn BCryptEncrypt(emu: &mut emu::Emu) {
    let h_key = emu.regs().rcx;
    let pb_input = emu.regs().rdx;
    let cb_input = emu.regs().r8 as usize;
    let _pb_padding = emu.regs().r9;
    let _pb_iv = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _cb_iv = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as usize;
    let pb_output = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    let cb_output = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0) as usize;
    let pcb_result = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0);
    let _flags = emu.maps.read_qword(emu.regs().rsp + 0x50).unwrap_or(0);

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
}

pub fn BCryptDecrypt(emu: &mut emu::Emu) {
    let h_key = emu.regs().rcx;
    let pb_input = emu.regs().rdx;
    let cb_input = emu.regs().r8 as usize;
    let _pb_padding = emu.regs().r9;
    let _pb_iv = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _cb_iv = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as usize;
    let pb_output = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    let cb_output = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0) as usize;
    let pcb_result = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0);
    let _flags = emu.maps.read_qword(emu.regs().rsp + 0x50).unwrap_or(0);

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
}

pub fn BCryptSetProperty(emu: &mut emu::Emu) {
    let h_object = emu.regs().rcx;
    let psz_property = emu.regs().rdx;

    let prop = emu.maps.read_wide_string(psz_property);
    log_red!(
        emu,
        "bcrypt!BCryptSetProperty obj=0x{:x} prop={}",
        h_object,
        prop
    );

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

pub fn BCryptImportKeyPair(emu: &mut emu::Emu) {
    let h_algorithm = emu.regs().rcx;
    let _h_import_key = emu.regs().rdx;
    let _psz_blob_type = emu.regs().r8;
    let ph_key = emu.regs().r9;

    let algo_uri = helper::handler_get_uri(h_algorithm);
    let algo_name = algo_uri.strip_prefix("bcrypt://").unwrap_or(&algo_uri);
    let hndl = helper::handler_create(&format!("bcryptkey://{}", algo_name));

    if ph_key != 0 {
        emu.maps.write_qword(ph_key, hndl);
    }

    log_red!(
        emu,
        "bcrypt!BCryptImportKeyPair algo={} =0x{:x}",
        algo_name,
        hndl
    );

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

pub fn BCryptExportKey(emu: &mut emu::Emu) {
    let _h_key = emu.regs().rcx;
    let _h_export_key = emu.regs().rdx;
    let psz_blob_type = emu.regs().r8;
    let pb_output = emu.regs().r9;
    let cb_output = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as usize;
    let pcb_result = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);

    let blob_type = emu.maps.read_wide_string(psz_blob_type);
    log_red!(emu, "bcrypt!BCryptExportKey type={}", blob_type);

    if pb_output != 0 && cb_output > 0 {
        emu.maps.memset(pb_output, 0, cb_output.min(1024));
    }
    if pcb_result != 0 {
        emu.maps.write_dword(pcb_result, 0);
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

pub fn BCryptSignHash(emu: &mut emu::Emu) {
    let _h_key = emu.regs().rcx;
    let pb_output = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let cb_output = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as usize;
    let pcb_result = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);

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
}

pub fn BCryptVerifySignature(emu: &mut emu::Emu) {
    let h_key = emu.regs().rcx;
    let _padding_info = emu.regs().rdx;
    let pb_hash = emu.regs().r8;
    let cb_hash = emu.regs().r9 as u32;
    let pb_signature = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let cb_signature = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as u32;
    let _flags = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0) as u32;

    log_red!(
        emu,
        "bcrypt!BCryptVerifySignature key:0x{:x} hash:0x{:x}({}) sig:0x{:x}({}) => valid",
        h_key,
        pb_hash,
        cb_hash,
        pb_signature,
        cb_signature
    );
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

pub fn BCryptGenerateKeyPair(emu: &mut emu::Emu) {
    let h_algorithm = emu.regs().rcx;
    let ph_key = emu.regs().rdx;
    let length = emu.regs().r8 as u32;

    let algo_uri = helper::handler_get_uri(h_algorithm);
    let algo_name = algo_uri.strip_prefix("bcrypt://").unwrap_or(&algo_uri);
    let hndl = helper::handler_create(&format!("bcryptkeypair://{}", algo_name));

    if ph_key != 0 {
        emu.maps.write_qword(ph_key, hndl);
    }

    log_red!(
        emu,
        "bcrypt!BCryptGenerateKeyPair algo={} bits={} =0x{:x}",
        algo_name,
        length,
        hndl
    );

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

pub fn BCryptFinalizeKeyPair(emu: &mut emu::Emu) {
    let h_key = emu.regs().rcx;

    log_red!(emu, "bcrypt!BCryptFinalizeKeyPair key=0x{:x}", h_key);

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

pub fn BCryptHash(emu: &mut emu::Emu) {
    let h_algorithm = emu.regs().rcx;
    let _pb_secret = emu.regs().rdx;
    let _cb_secret = emu.regs().r8 as usize;
    let pb_input = emu.regs().r9;
    let cb_input = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as usize;
    let pb_output = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let cb_output = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0) as usize;

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
}
