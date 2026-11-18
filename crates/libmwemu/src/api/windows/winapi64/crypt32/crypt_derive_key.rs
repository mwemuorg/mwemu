use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptDeriveKey(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_prov = api_try!(emu, call.arg(emu, 0));
    let algid = api_try!(emu, call.arg_u32(emu, 1));
    let h_hash = api_try!(emu, call.arg(emu, 2));
    let flags = api_try!(emu, call.arg(emu, 3));
    let h_key_ptr = api_try!(emu, call.arg(emu, 4));
    log_red!(
        emu,
        "crypt32!CryptDeriveKey prov: 0x{:x} alg: 0x{:x} hash: 0x{:x}",
        h_prov,
        algid,
        h_hash
    );
    let h_key = helper::handler_create(&format!("derivedkey:0x{:x}", algid));
    if h_key_ptr != 0 {
        emu.maps.write_qword(h_key_ptr, h_key);
    }
    call.set_return(emu, 1); // TRUE
}
