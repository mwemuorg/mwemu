use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn GetLogicalDrives(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernelbase!GetLogicalDrives => 0x4");
    let _h = helper::handler_create("logical_drives");
    emu.set_last_error(0);
    call.set_return(emu, 0x4);
}
