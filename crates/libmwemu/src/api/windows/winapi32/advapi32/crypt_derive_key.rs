use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::crypto::*;

pub(super) fn CryptDeriveKey(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hprov = api_try!(emu, call.arg(emu, 0));
    let algid = api_try!(emu, call.arg_u32(emu, 1));
    let _hbase_data = api_try!(emu, call.arg_u32(emu, 2));
    let flags = api_try!(emu, call.arg_u32(emu, 3));
    let hkey_ptr = api_try!(emu, call.arg(emu, 4));

    let alg = get_cryptoalgorithm_name(algid);
    let mut alg_len = get_crypto_key_len(algid);

    let handle = helper::handler_create(alg);
    if alg_len == 0 {
        alg_len = ((flags >> 16) & 0xffff) as usize;
    }
    helper::handler_put_bytes(handle, &vec![0x41u8; alg_len]);
    crypto_state::create_key(handle, algid, alg_len as u32);

    log_red!(emu, "advapi32!CryptDeriveKey {} bits:{}", alg, alg_len);

    if hkey_ptr != 0 {
        emu.maps.write_dword(hkey_ptr, handle as u32);
    }

    call.cleanup_args(emu, 5);
    call.set_return(emu, 1);
}
