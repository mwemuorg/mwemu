use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn _initialize_onexit_table(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let table = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "wincrt!_initialize_onexit_table 0x{:x}", table);

    if table != 0 {
        emu.maps.write_qword(table, 0); // _first
        emu.maps.write_qword(table + 8, 0); // _last
        emu.maps.write_qword(table + 16, 0); // _end
    }

    call.set_return(emu, 0);
}
