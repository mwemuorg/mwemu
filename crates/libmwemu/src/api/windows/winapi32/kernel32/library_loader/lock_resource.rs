use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn LockResource(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hResData = api_try!(emu, call.arg_u32(emu, 0)) as u64;

    call.cleanup_args(emu, 1);

    if helper::handler_exist(hResData) {
        let uri = helper::handler_get_uri(hResData);
        let ptr = match uri.split("_").next().and_then(|s| s.parse::<u64>().ok()) {
            Some(v) => v + emu.base,
            None => {
                log::warn!("LockResource: bad handler uri `{}`", uri);
                call.set_return(emu, 0);
                return;
            }
        };

        log_red!(
            emu,
            "** {} kernel32!LockResource {:x} {:x}",
            emu.pos,
            hResData,
            ptr
        );
        call.set_return(emu, ptr);
        return;
    }

    log_red!(
        emu,
        "** {} kernel32!LockResource {:x} not found",
        emu.pos,
        hResData
    );

    call.set_return(emu, 0);
}
