use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn AccessCheck(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p_sd = api_try!(emu, call.arg(emu, 0));
    let client_token = api_try!(emu, call.arg(emu, 1));
    let desired_access = api_try!(emu, call.arg(emu, 2));
    let generic_mapping = api_try!(emu, call.arg(emu, 3));
    let priv_set = api_try!(emu, call.arg(emu, 4));
    let priv_set_len = api_try!(emu, call.arg(emu, 5));
    let granted_access = api_try!(emu, call.arg(emu, 6));
    let access_status = api_try!(emu, call.arg(emu, 7));
    log_red!(
        emu,
        "kernelbase!AccessCheck sd:0x{:x} token:0x{:x} desired:0x{:x}",
        p_sd,
        client_token,
        desired_access
    );
    if granted_access != 0 {
        emu.maps.write_dword(granted_access, desired_access as u32);
    }
    if access_status != 0 {
        emu.maps.write_dword(access_status, 1);
    }
    if priv_set_len != 0 {
        emu.maps.write_dword(priv_set_len, 0);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
