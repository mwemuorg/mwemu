use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SizeofResource(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hModule = api_try!(emu, call.arg(emu, 0));
    let hResInfo = api_try!(emu, call.arg(emu, 1));

    if helper::handler_exist(hResInfo) {
        let uri = helper::handler_get_uri(hResInfo);
        let size = match uri.split("_").last().and_then(|s| s.parse::<usize>().ok()) {
            Some(v) => v,
            None => {
                log::warn!("SizeofResource: bad handler uri `{}`", uri);
                emu.set_last_error(0);
                call.set_return(emu, 0);
                return;
            }
        };
        log::trace!(
            "** {} kernel32!SizeofResource {:x} {:x} size: {}",
            emu.pos,
            hModule,
            hResInfo,
            size
        );
        call.set_return(emu, size as u64);
        return;
    }

    log_red!(
        emu,
        "** {} kernel32!SizeofResource {:x} {:x} not found",
        emu.pos,
        hModule,
        hResInfo
    );

    call.set_return(emu, 0);
}
