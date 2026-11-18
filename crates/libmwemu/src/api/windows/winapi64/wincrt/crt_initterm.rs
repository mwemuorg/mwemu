use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_initterm(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let start = api_try!(emu, call.arg(emu, 0));
    let end = api_try!(emu, call.arg(emu, 1));
    let mut count = 0u32;
    if start != 0 && end > start {
        let mut ptr = start;
        while ptr < end {
            let func = emu.maps.read_qword(ptr).unwrap_or(0);
            if func != 0 {
                count += 1;
            }
            ptr += 8;
        }
    }
    log_red!(
        emu,
        "wincrt!_initterm 0x{:x} - 0x{:x} ({} entries)",
        start,
        end,
        count
    );
    call.set_return(emu, 0);
}
