use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn ClearEventLogA(emu: &mut emu::Emu) {
    clear_event_log(emu, "ClearEventLogA", false);
}

fn clear_event_log(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let event_log = api_try!(emu, call.arg(emu, 0));
    let backup_ptr = api_try!(emu, call.arg(emu, 1));
    let backup = if backup_ptr != 0 && backup_ptr > 0xffff {
        strings::read_text(emu, backup_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "advapi32!{api} handle:0x{:x} backup:'{}'",
        event_log,
        backup
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn ClearEventLogW(emu: &mut emu::Emu) {
    clear_event_log(emu, "ClearEventLogW", true);
}
