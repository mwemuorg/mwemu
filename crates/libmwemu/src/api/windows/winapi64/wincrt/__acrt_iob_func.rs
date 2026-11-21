use super::allocate_memory;
use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn __acrt_iob_func(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let index = api_try!(emu, call.arg(emu, 0));
    let iob_addr = allocate_memory(emu, 96).unwrap_or(0);
    if iob_addr != 0 {
        emu.maps.memset(iob_addr, 0, 96);
        let fd = match index {
            0 => 0u32, // stdin
            1 => 1,    // stdout
            2 => 2,    // stderr
            _ => index as u32,
        };
        emu.maps.write_dword(iob_addr + 16, fd); // _file field offset
    }
    log_red!(
        emu,
        "wincrt!__acrt_iob_func index:{} => 0x{:x}",
        index,
        iob_addr
    );
    call.set_return(emu, iob_addr);
}
