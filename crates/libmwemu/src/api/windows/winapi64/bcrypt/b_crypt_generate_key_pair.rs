use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn BCryptGenerateKeyPair(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_algorithm = api_try!(emu, call.arg(emu, 0));
    let ph_key = api_try!(emu, call.arg(emu, 1));
    let length = api_try!(emu, call.arg_u32(emu, 2));

    let algo_uri = helper::handler_get_uri(h_algorithm);
    let algo_name = algo_uri.strip_prefix("bcrypt://").unwrap_or(&algo_uri);
    let hndl = helper::handler_create(&format!("bcryptkeypair://{}", algo_name));

    if ph_key != 0 {
        emu.maps.write_qword(ph_key, hndl);
    }

    log_red!(
        emu,
        "bcrypt!BCryptGenerateKeyPair algo={} bits={} =0x{:x}",
        algo_name,
        length,
        hndl
    );

    call.set_return(emu, STATUS_SUCCESS);
}
