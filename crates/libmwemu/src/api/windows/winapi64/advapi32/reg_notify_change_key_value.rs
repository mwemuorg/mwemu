use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn RegNotifyChangeKeyValue(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let watch_subtree = api_try!(emu, call.arg(emu, 1)) != 0;
    let filter = api_try!(emu, call.arg(emu, 2));
    let event = api_try!(emu, call.arg(emu, 3));
    let _async = api_try!(emu, call.arg(emu, 4)) != 0;
    log_red!(
        emu,
        "advapi32!RegNotifyChangeKeyValue hkey:0x{:x} subtree:{} filter:0x{:x}",
        hkey,
        watch_subtree,
        filter
    );
    let val_ptr = api_try!(emu, call.arg(emu, 1));
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
