use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::crypto::*;

pub fn CryptCreateHash(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hprov = api_try!(emu, call.arg_u32(emu, 0));
    let algid = api_try!(emu, call.arg_u32(emu, 1));
    let hkey = api_try!(emu, call.arg_u32(emu, 2));
    let flags = api_try!(emu, call.arg_u32(emu, 3));
    let ptr_hash = api_try!(emu, call.arg(emu, 4));

    let alg_name = get_cryptoalgorithm_name(algid);

    log_red!(emu, "kernel32!CryptCreateHash alg:{}", alg_name);

    call.cleanup_args(emu, 5);

    emu.maps.write_dword(
        ptr_hash,
        helper::handler_create(&format!("alg://{}", alg_name)) as u32,
    );
    call.set_return(emu, 1);
}
