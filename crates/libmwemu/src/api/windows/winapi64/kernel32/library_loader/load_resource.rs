use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
pub fn LoadResource(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hModule = api_try!(emu, call.arg(emu, 0));
    let hResInfo = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "** {} kernel32!LoadResource {:x} {:x}",
        emu.pos,
        hModule,
        hResInfo
    );
    emu.set_last_error(0);
    call.set_return(emu, hResInfo);
}
