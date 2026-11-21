use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn EventWriteTransfer(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let reg_handle = api_try!(emu, call.arg_u32(emu, 0));
    let event_desc = api_try!(emu, call.arg_u32(emu, 1));
    let activity_id = api_try!(emu, call.arg_u32(emu, 2));
    let related_id = api_try!(emu, call.arg_u32(emu, 3));
    let user_data_count = api_try!(emu, call.arg_u32(emu, 4));
    let user_data = api_try!(emu, call.arg_u32(emu, 5));
    let _reserved = api_try!(emu, call.arg_u32(emu, 6));
    log_red!(
        emu,
        "advapi32!EventWriteTransfer handle: 0x{:x} desc: 0x{:x} activity: 0x{:x} count: {}",
        reg_handle,
        event_desc,
        activity_id,
        user_data_count
    );
    let val_ptr = api_try!(emu, call.arg(emu, 1));
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 7);
}
