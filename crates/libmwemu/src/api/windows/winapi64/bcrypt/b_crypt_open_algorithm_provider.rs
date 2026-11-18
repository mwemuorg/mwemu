use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn BCryptOpenAlgorithmProvider(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ph_algorithm = api_try!(emu, call.arg(emu, 0));
    let psz_alg_id = api_try!(emu, call.arg(emu, 1));
    let flags = api_try!(emu, call.arg(emu, 3));

    let algo_name = emu.maps.read_wide_string(psz_alg_id);

    log_red!(
        emu,
        "bcrypt!BCryptOpenAlgorithmProvider algo: {} flags: 0x{:x}",
        algo_name,
        flags
    );

    let handle = helper::handler_create(&format!("bcrypt://{}", algo_name));
    if ph_algorithm != 0 {
        emu.maps.write_qword(ph_algorithm, handle);
    }
    call.set_return(emu, STATUS_SUCCESS);
}
