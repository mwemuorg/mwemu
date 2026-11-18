use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptGenKey(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_prov = api_try!(emu, call.arg(emu, 0));
    let algid = api_try!(emu, call.arg_u32(emu, 1));
    let flags = api_try!(emu, call.arg(emu, 2));
    let h_key_ptr = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "crypt32!CryptGenKey prov: 0x{:x} alg: 0x{:x} flags: 0x{:x}",
        h_prov,
        algid,
        flags
    );
    let h_key = helper::handler_create(&format!("cryptkey:0x{:x}", algid));
    if h_key_ptr != 0 {
        emu.maps.write_qword(h_key_ptr, h_key);
    }
    call.set_return(emu, 1); // TRUE
}
