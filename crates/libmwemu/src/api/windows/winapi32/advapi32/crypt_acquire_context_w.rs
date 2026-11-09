use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::CRYPT_DEFAULT_CONTAINER_OPTIONAL;
use crate::windows::constants::CRYPT_DELETEKEYSET;
use crate::windows::constants::CRYPT_MACHINE_KEYSET;
use crate::windows::constants::CRYPT_NEWKEYSET;
use crate::windows::constants::CRYPT_SILENT;
use crate::windows::constants::CRYPT_VERIFYCONTEXT;

pub(super) fn CryptAcquireContextW(emu: &mut emu::Emu) {
    let out_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let container = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let provider = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let prov_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

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
        "advapi321!CryptAcquireContextW =0x{:x} type: {} flags: `{}`",
        hndl,
        prov_type,
        &sflags
    );

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
