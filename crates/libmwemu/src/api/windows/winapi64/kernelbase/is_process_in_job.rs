use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn IsProcessInJob(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let process = api_try!(emu, call.arg(emu, 0));
    let job = api_try!(emu, call.arg(emu, 1));
    let result_ptr = api_try!(emu, call.arg(emu, 2));
    if result_ptr != 0 {
        emu.maps.write_dword(result_ptr, 0);
    }
    log_red!(
        emu,
        "kernelbase!IsProcessInJob proc:0x{:x} job:0x{:x} => false",
        process,
        job
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
