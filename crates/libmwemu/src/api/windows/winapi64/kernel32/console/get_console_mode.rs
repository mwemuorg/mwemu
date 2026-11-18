use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetConsoleMode(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_console_handle = api_try!(emu, call.arg(emu, 0));
    let lp_mode = api_try!(emu, call.arg(emu, 1)) as usize;
    log_red!(
        emu,
        "** {} kernel32!GetConsoleMode {:x} {:x}",
        emu.pos,
        h_console_handle,
        lp_mode
    );
    // TODO: implement this
    emu.maps.write_dword(lp_mode as u64, 0x00000003); //TODO: not sure what this is
    call.set_return(emu, 1);
}
