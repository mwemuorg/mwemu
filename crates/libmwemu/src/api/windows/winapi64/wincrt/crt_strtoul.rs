use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_strtoul(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let nptr = api_try!(emu, call.arg(emu, 0));
    let endptr = api_try!(emu, call.arg(emu, 1));
    let base = api_try!(emu, call.arg_u32(emu, 2));
    let s = emu.maps.read_string(nptr);
    let trimmed = s.trim();
    let val = u64::from_str_radix(
        trimmed.trim_start_matches("0x").trim_start_matches("0X"),
        if base == 0 {
            if trimmed.starts_with("0x") || trimmed.starts_with("0X") {
                16
            } else {
                10
            }
        } else {
            base
        },
    )
    .unwrap_or(0);
    if endptr != 0 {
        emu.maps.write_qword(endptr, nptr + s.len() as u64);
    }
    log_red!(emu, "wincrt!strtoul '{}' base:{} => {}", s, base, val);
    call.set_return(emu, val);
}
