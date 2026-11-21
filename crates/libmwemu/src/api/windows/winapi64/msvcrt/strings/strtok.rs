use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn strtok(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let delim_ptr = api_try!(emu, call.arg(emu, 1));
    let delims = emu.maps.read_string(delim_ptr);
    if str_ptr != 0 {
        let s = emu.maps.read_string(str_ptr);
        if let Some(pos) = s.find(|c: char| delims.contains(c)) {
            emu.maps.write_byte(str_ptr + pos as u64, 0);
            log_red!(
                emu,
                "msvcrt!strtok '{}' delim:'{}' => '{}'",
                s,
                delims,
                &s[..pos]
            );
            call.set_return(emu, str_ptr);
        } else if !s.is_empty() {
            log_red!(emu, "msvcrt!strtok '{}' delim:'{}' => '{}'", s, delims, s);
            call.set_return(emu, str_ptr);
        } else {
            log_red!(emu, "msvcrt!strtok empty");
            emu.set_last_error(0);
            call.set_return(emu, 0);
        }
    } else {
        log_red!(emu, "msvcrt!strtok(NULL) => 0");
        call.set_return(emu, 0);
    }
}
