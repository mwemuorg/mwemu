use crate::emu;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi32::kernel32;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "PkiInitializeCriticalSection" => PkiInitializeCriticalSection(emu),
        "CryptStringToBinaryA" => CryptStringToBinaryA(emu),
        "CryptDecodeObjectEx" => CryptDecodeObjectEx(emu),
        "CertOpenStore" => CertOpenStore(emu),
        "CertOpenSystemStoreA" => CertOpenSystemStoreA(emu),
        "CertOpenSystemStoreW" => CertOpenSystemStoreW(emu),
        "CertCloseStore" => CertCloseStore(emu),
        "CertFindCertificateInStore" => CertFindCertificateInStore(emu),
        "CertFreeCertificateContext" => CertFreeCertificateContext(emu),

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

            log::error!(
                "attempt to call unimplemented crypt32 API 0x{:x} {}",
                addr,
                api
            );
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

fn PkiInitializeCriticalSection(emu: &mut emu::Emu) {
    let addr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    log_red!(
        emu,
        "crypt32!Pki_InitializeCriticalSection flags: {:x} addr: 0x{:x}",
        flags,
        addr
    );

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn CryptStringToBinaryA(emu: &mut emu::Emu) {
    let string = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let num_chars = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let inout_sz = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let skip = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
    let out_flags = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);

    let dflags = match flags {
        0x00000000 => "CRYPT_STRING_BASE64HEADER",
        0x00000001 => "CRYPT_STRING_BASE64",
        0x00000002 => "CRYPT_STRING_BINARY",
        0x00000003 => "CRYPT_STRING_BASE64REQUESTHEADER",
        0x00000004 => "CRYPT_STRING_HEX",
        0x00000005 => "CRYPT_STRING_HEXASCII",
        0x00000006 => "CRYPT_STRING_BASE64_ANY",
        0x00000007 => "CRYPT_STRING_ANY",
        0x00000008 => "CRYPT_STRING_HEX_ANY",
        0x00000009 => "CRYPT_STRING_BASE64X509CRLHEADER",
        0x0000000a => "CRYPT_STRING_HEXADDR",
        0x0000000b => "CRYPT_STRING_HEXASCIIADDR",
        0x0000000c => "CRYPT_STRING_HEXRAW",
        0x20000000 => "CRYPT_STRING_STRICT",
        _ => "incorrect flag",
    };

    log_red!(
        emu,
        "crypt32!CryptStringToBinaryA str: 0x{:x} len: {} ptr: {} len: {} {}",
        string,
        num_chars,
        ptr,
        inout_sz,
        dflags
    );

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn CryptDecodeObjectEx(emu: &mut emu::Emu) {
    let encoding_type = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let struct_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let encoded = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let cb_encoded = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let decode_para = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
    let pv_struct_info = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);
    let pcb_struct_info = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0);

    log_red!(
        emu,
        "crypt32!CryptDecodeObjectEx enc_type: 0x{:x} struct_type: 0x{:x} encoded: 0x{:x} len: {} flags: 0x{:x}",
        encoding_type,
        struct_type,
        encoded,
        cb_encoded,
        flags
    );

    if pcb_struct_info != 0 {
        emu.maps.write_dword(pcb_struct_info as u64, 0);
    }

    for _ in 0..8 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn CertOpenStore(emu: &mut emu::Emu) {
    let store_provider = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let encoding_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let hcrypt_prov = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let pv_para = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    log_red!(
        emu,
        "crypt32!CertOpenStore provider: 0x{:x} enc: 0x{:x} flags: 0x{:x}",
        store_provider,
        encoding_type,
        flags
    );

    for _ in 0..5 {
        emu.stack_pop32(false);
    }

    let handle = helper::handler_create("cert_store");
    emu.regs_mut().rax = handle;
}

fn CertOpenSystemStoreA(emu: &mut emu::Emu) {
    let hprov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let subsystem = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    let name = emu.maps.read_string(subsystem as u64);

    log_red!(emu, "crypt32!CertOpenSystemStoreA subsystem: {}", name);

    for _ in 0..2 {
        emu.stack_pop32(false);
    }

    let handle = helper::handler_create(&format!("cert_sys_store://{}", name));
    emu.regs_mut().rax = handle;
}

fn CertOpenSystemStoreW(emu: &mut emu::Emu) {
    let hprov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let subsystem = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    let name = emu.maps.read_wide_string(subsystem as u64);

    log_red!(emu, "crypt32!CertOpenSystemStoreW subsystem: {}", name);

    for _ in 0..2 {
        emu.stack_pop32(false);
    }

    let handle = helper::handler_create(&format!("cert_sys_store://{}", name));
    emu.regs_mut().rax = handle;
}

fn CertCloseStore(emu: &mut emu::Emu) {
    let h_store = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    log_red!(
        emu,
        "crypt32!CertCloseStore handle: 0x{:x} flags: 0x{:x}",
        h_store,
        flags
    );

    helper::handler_close(h_store as u64);

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn CertFindCertificateInStore(emu: &mut emu::Emu) {
    let h_store = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let encoding_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let find_flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let find_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let find_para = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let prev_cert = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);

    log_red!(
        emu,
        "crypt32!CertFindCertificateInStore store: 0x{:x} find_type: 0x{:x}",
        h_store,
        find_type
    );

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 0;
}

fn CertFreeCertificateContext(emu: &mut emu::Emu) {
    let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(
        emu,
        "crypt32!CertFreeCertificateContext ctx: 0x{:x}",
        cert_ctx
    );

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
