use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn CryptImportKey(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _hprov = api_try!(emu, call.arg(emu, 0));
    let data_ptr = api_try!(emu, call.arg(emu, 1));
    let data_len = api_try!(emu, call.arg_u32(emu, 2));
    let _hpubkey = api_try!(emu, call.arg(emu, 3));
    let _flags = api_try!(emu, call.arg_u32(emu, 4));
    let hkey_ptr = api_try!(emu, call.arg(emu, 5));

    let algid = if data_len >= 12 {
        emu.maps.read_dword(data_ptr + 8).unwrap_or(0x6801)
    } else {
        0x6801
    };

    log_red!(
        emu,
        "advapi32!CryptImportKey data:0x{:x} len:{} alg:0x{:x}",
        data_ptr,
        data_len,
        algid
    );

    if hkey_ptr != 0 {
        let key_handle = helper::handler_create("crypt_key");
        crypto_state::create_key(key_handle, algid, data_len * 8);
        emu.maps.write_dword(hkey_ptr, key_handle as u32);
    }

    call.cleanup_args(emu, 6);
    call.set_return(emu, 1);
}
