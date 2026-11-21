use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetProcessVersion(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let process_id = api_try!(emu, call.arg(emu, 0));
    let _ = emu.maps.is_mapped(process_id);
    let version = 0x000A_0000u64;
    log_red!(
        emu,
        "kernelbase!GetProcessVersion pid:{} => 0x{:x}",
        process_id,
        version
    );
    emu.set_last_error(0);
    call.set_return(emu, version);
}
