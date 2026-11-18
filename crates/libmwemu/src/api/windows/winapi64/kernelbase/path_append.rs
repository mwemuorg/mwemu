use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn PathAppendA(emu: &mut emu::Emu) {
    path_append(emu, "PathAppendA", false);
}

pub fn PathAppendW(emu: &mut emu::Emu) {
    path_append(emu, "PathAppendW", true);
}

fn path_append(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    if wide {
        let path_ptr = api_try!(emu, call.arg(emu, 0));
        let more_ptr = api_try!(emu, call.arg(emu, 1));
        let path = if path_ptr != 0 {
            strings::read_text(emu, path_ptr, wide)
        } else {
            String::new()
        };
        let more = if more_ptr != 0 {
            strings::read_text(emu, more_ptr, wide)
        } else {
            String::new()
        };
        let result = if path.is_empty() {
            more.clone()
        } else if more.is_empty() {
            path.clone()
        } else {
            let sep = if path.ends_with('\\') || path.ends_with('/') {
                ""
            } else {
                "\\"
            };
            format!("{}{}{}", path, sep, more)
        };
        if path_ptr != 0 {
            strings::write_string(emu, path_ptr, &result, wide);
        }
        log_red!(
            emu,
            "kernelbase!{api} '{}' + '{}' => '{}'",
            path,
            more,
            result
        );
    } else {
        let p0 = api_try!(emu, call.arg(emu, 0));
        let p1 = api_try!(emu, call.arg(emu, 1));
        let s = if p0 != 0 && p0 > 0xffff {
            strings::read_text(emu, p0, wide)
        } else {
            format!("0x{:x}", p0)
        };
        log_red!(emu, "kernelbase!{api} '{}'  p1:0x{:x}", s, p1);
    }

    emu.set_last_error(0);
    call.set_return(emu, 1);
}
