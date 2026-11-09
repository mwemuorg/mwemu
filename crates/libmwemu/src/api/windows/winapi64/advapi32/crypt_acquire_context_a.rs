use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::CRYPT_DEFAULT_CONTAINER_OPTIONAL;
use crate::windows::constants::CRYPT_DELETEKEYSET;
use crate::windows::constants::CRYPT_MACHINE_KEYSET;
use crate::windows::constants::CRYPT_NEWKEYSET;
use crate::windows::constants::CRYPT_SILENT;
use crate::windows::constants::CRYPT_VERIFYCONTEXT;

pub fn CryptAcquireContextA(emu: &mut emu::Emu) {
    let out_handle = emu.regs().rcx;
    let container_ptr = emu.regs().rdx;
    let provider_ptr = emu.regs().r8;
    let prov_type = emu.regs().r9 as u32;
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0) as u32;

    let uri = "cryptctx://".to_string();
    let hndl = helper::handler_create(&uri);
    if out_handle != 0 {
        emu.maps.write_qword(out_handle, hndl);
    }

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
        "advapi32!CryptAcquireContextA =0x{:x} type: {} flags: `{}`",
        hndl,
        prov_type,
        &sflags
    );

    emu.regs_mut().rax = 1;
}
