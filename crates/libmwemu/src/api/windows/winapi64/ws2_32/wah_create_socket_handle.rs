use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WahCreateSocketHandle(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let table_ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ws2_32!WahCreateSocketHandle ptr:0x{:x}", table_ptr);
    let handle = helper::handler_create("wah_handle");
    if table_ptr != 0 {
        emu.maps.write_qword(table_ptr, handle);
    }
    call.set_return(emu, 0);
}
