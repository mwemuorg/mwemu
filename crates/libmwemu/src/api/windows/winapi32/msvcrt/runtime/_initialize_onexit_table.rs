use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _initialize_onexit_table(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let table_ptr = api_try!(emu, call.arg(emu, 0));
    if table_ptr != 0 {
        emu.maps.write_dword(table_ptr, 0); // _first
        emu.maps.write_dword(table_ptr + 4, 0); // _last
        emu.maps.write_dword(table_ptr + 8, 0); // _end
    }
    log_red!(emu, "msvcrt!_initialize_onexit_table tbl:0x{:x}", table_ptr);
    call.set_return(emu, 0);
}
