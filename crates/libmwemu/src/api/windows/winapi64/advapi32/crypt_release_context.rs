use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptReleaseContext(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hprov = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(emu, "advapi32!CryptReleaseContext hProv=0x{:x}", hprov);

    helper::handler_close(hprov);

    call.set_return(emu, 1);
}
