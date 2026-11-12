use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::crypto::*;

pub(super) fn CryptAcquireContextA(emu: &mut emu::Emu) {
    crypt_acquire_context(emu, "CryptAcquireContextA");
}

fn crypt_acquire_context(emu: &mut emu::Emu, api: &str) {
    let out_handle = WinApiAbi::arg(emu, 0);
    let container = WinApiAbi::arg(emu, 1) as u32;
    let provider = WinApiAbi::arg(emu, 2) as u32;
    let prov_type = WinApiAbi::arg(emu, 3) as u32;
    let flags = WinApiAbi::arg(emu, 4) as u32;

    let uri = "cryptctx://".to_string();
    let hndl = helper::handler_create(&uri) as u32;
    emu.maps.write_dword(out_handle, hndl);

    let mut sflags = String::new();
    if flags & CRYPT_VERIFYCONTEXT == CRYPT_VERIFYCONTEXT {
        sflags.push_str("CRYPT_VERIFYCONTEXT ");
    }
    if flags & CRYPT_NEWKEYSET == CRYPT_NEWKEYSET {
        sflags.push_str("CRYPT_NEWKEYSET ");
    }
    if flags & CRYPT_DELETEKEYSET == CRYPT_DELETEKEYSET {
        sflags.push_str("CRYPT_DELETEKEYSET ");
    }
    if flags & CRYPT_MACHINE_KEYSET == CRYPT_MACHINE_KEYSET {
        sflags.push_str("CRYPT_MACHINE_KEYSET ");
    }
    if flags & CRYPT_SILENT == CRYPT_SILENT {
        sflags.push_str("CRYPT_SILENT ");
    }
    if flags & CRYPT_DEFAULT_CONTAINER_OPTIONAL == CRYPT_DEFAULT_CONTAINER_OPTIONAL {
        sflags.push_str("CRYPT_DEFAULT_CONTAINER_OPTIONAL ");
    }

    log_red!(
        emu,
        "advapi321!{api} =0x{:x} type: {} flags: `{}`",
        hndl,
        prov_type,
        &sflags
    );

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub(super) fn CryptAcquireContextW(emu: &mut emu::Emu) {
    crypt_acquire_context(emu, "CryptAcquireContextW");
}
