use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetThreadPreferredUILanguages(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let flags = api_try!(emu, call.arg_u32(emu, 0));
    let num_langs_ptr = api_try!(emu, call.arg(emu, 1));
    let buff = api_try!(emu, call.arg(emu, 2));
    let out_sz = api_try!(emu, call.arg(emu, 3));

    emu.maps.write_dword(num_langs_ptr, 0);
    log_red!(emu, "kernel32!GetThreadPreferredUILanguages");

    emu.maps.write_dword(out_sz, 0);
    emu.maps.write_dword(num_langs_ptr, 0);

    call.cleanup_args(emu, 4);

    call.set_return(emu, 1);
}
