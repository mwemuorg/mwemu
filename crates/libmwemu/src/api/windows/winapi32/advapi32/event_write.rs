use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn EventWrite(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let reg_handle = api_try!(emu, call.arg_u32(emu, 0));
    let event_desc = api_try!(emu, call.arg_u32(emu, 1));
    let user_data_count = api_try!(emu, call.arg_u32(emu, 2));
    let user_data = api_try!(emu, call.arg_u32(emu, 3));
    let _activity_id = api_try!(emu, call.arg_u32(emu, 4));
    log_red!(
        emu,
        "advapi32!EventWrite handle: 0x{:x} desc: 0x{:x} count: {} data: 0x{:x}",
        reg_handle,
        event_desc,
        user_data_count,
        user_data
    );
    let val_ptr = api_try!(emu, call.arg(emu, 1));
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 5);
}
