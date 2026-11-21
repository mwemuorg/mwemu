use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn LCMapStringA(emu: &mut emu::Emu) {
    lc_map_string(emu, "LCMapStringA", false);
}

pub fn LCMapStringW(emu: &mut emu::Emu) {
    lc_map_string(emu, "LCMapStringW", true);
}

fn lc_map_string(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _locale = api_try!(emu, call.arg_u32(emu, 0));
    let flags = api_try!(emu, call.arg_u32(emu, 1));
    let src_ptr = api_try!(emu, call.arg(emu, 2));
    if wide {
        let src_len = api_try!(emu, call.arg_i32(emu, 3));
        let dst_ptr = api_try!(emu, call.arg(emu, 4));
        let dst_len = api_try!(emu, call.arg_i32(emu, 5));
        let src = if src_ptr != 0 {
            strings::read_text(emu, src_ptr, wide)
        } else {
            String::new()
        };
        let mapped = if flags & 0x00000100 != 0 {
            src.to_uppercase()
        } else if flags & 0x00000200 != 0 {
            src.to_lowercase()
        } else {
            src.clone()
        };
        let result_len = mapped.encode_utf16().count() + 1;
        if dst_ptr != 0 && dst_len > 0 {
            strings::write_string(emu, dst_ptr, &mapped, wide);
        }
        log_red!(
            emu,
            "kernelbase!{api} flags:0x{:x} '{}' => '{}'",
            flags,
            src,
            mapped
        );
        emu.set_last_error(0);
        call.set_return(emu, result_len as u64);
    } else {
        let _src_len = api_try!(emu, call.arg_i32(emu, 3));
        let dst_ptr = api_try!(emu, call.arg(emu, 4));
        let dst_len = api_try!(emu, call.arg_i32(emu, 5));
        let src = if src_ptr != 0 {
            strings::read_text(emu, src_ptr, wide)
        } else {
            String::new()
        };
        let mapped = if flags & 0x00000100 != 0 {
            src.to_uppercase()
        } else if flags & 0x00000200 != 0 {
            src.to_lowercase()
        } else {
            src.clone()
        };
        let result_len = mapped.len() + 1;
        if dst_ptr != 0 && dst_len > 0 {
            strings::write_string(emu, dst_ptr, &mapped, wide);
        }
        log_red!(
            emu,
            "kernelbase!{api} flags:0x{:x} '{}' => '{}'",
            flags,
            src,
            mapped
        );
        emu.set_last_error(0);
        call.set_return(emu, result_len as u64);
    }
}
