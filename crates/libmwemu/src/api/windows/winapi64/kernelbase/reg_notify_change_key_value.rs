use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn RegNotifyChangeKeyValue(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let watch_subtree = api_try!(emu, call.arg(emu, 1));
    let notify_filter = api_try!(emu, call.arg(emu, 2));
    let h_event = api_try!(emu, call.arg(emu, 3));
    let async_notify = api_try!(emu, call.arg(emu, 4));
    log_red!(
        emu,
        "kernelbase!RegNotifyChangeKeyValue hkey:0x{:x} filter:0x{:x} async:{}",
        hkey,
        notify_filter,
        async_notify
    );
    let val_ptr = api_try!(emu, call.arg(emu, 1));
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0); // ERROR_SUCCESS
}
