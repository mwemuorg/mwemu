use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn LoadResource(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hModule = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let hResInfo = api_try!(emu, call.arg_u32(emu, 1)) as u64;

    log_red!(
        emu,
        "** {} kernel32!LoadResource {:x} {:x}",
        emu.pos,
        hModule,
        hResInfo
    );

    call.cleanup_args(emu, 2);

    call.set_return(emu, hResInfo);
}
