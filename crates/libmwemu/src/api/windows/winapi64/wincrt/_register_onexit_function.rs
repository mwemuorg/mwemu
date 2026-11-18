use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn _register_onexit_function(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let table = api_try!(emu, call.arg(emu, 0));
    let callback = api_try!(emu, call.arg(emu, 1));

    log_red!(
        emu,
        "wincrt!_register_onexit_function table:0x{:x} callback:0x{:x}",
        table,
        callback
    );

    if table != 0 {
        let last = emu.maps.read_qword(table + 8).unwrap_or(0);
        let end = emu.maps.read_qword(table + 16).unwrap_or(0);
        if last < end && last != 0 {
            emu.maps.write_qword(last, callback);
            emu.maps.write_qword(table + 8, last + 8);
        }
    }

    call.set_return(emu, 0);
}
