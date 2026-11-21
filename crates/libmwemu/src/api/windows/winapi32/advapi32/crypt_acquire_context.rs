use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::crypto::*;

pub(super) fn CryptAcquireContextA(emu: &mut emu::Emu) {
    crypt_acquire_context(emu, "CryptAcquireContextA");
}

fn crypt_acquire_context(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let out_handle = api_try!(emu, call.arg(emu, 0));
    let container = api_try!(emu, call.arg_u32(emu, 1));
    let provider = api_try!(emu, call.arg_u32(emu, 2));
    let prov_type = api_try!(emu, call.arg_u32(emu, 3));
    let flags = api_try!(emu, call.arg_u32(emu, 4));

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

    call.cleanup_args(emu, 5);
    call.set_return(emu, 1);
}

pub(super) fn CryptAcquireContextW(emu: &mut emu::Emu) {
    crypt_acquire_context(emu, "CryptAcquireContextW");
}
