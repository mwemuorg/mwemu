use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn FindNLSStringEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let locale_ptr = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg_u32(emu, 1));
    let src_ptr = api_try!(emu, call.arg(emu, 2));
    let src_count = api_try!(emu, call.arg_i32(emu, 3));
    let value_ptr = api_try!(emu, call.arg(emu, 4));
    let locale = if locale_ptr != 0 && locale_ptr > 0xffff {
        emu.maps.read_wide_string(locale_ptr)
    } else {
        String::new()
    };
    let src = emu.maps.read_wide_string(src_ptr);
    let value = if value_ptr != 0 {
        emu.maps.read_wide_string(value_ptr)
    } else {
        String::new()
    };
    let result = if let Some(pos) = src.to_lowercase().find(&value.to_lowercase()) {
        pos as i64
    } else {
        -1i64
    };
    log_red!(
        emu,
        "kernelbase!FindNLSStringEx locale:'{}' src:'{}' find:'{}' => {}",
        locale,
        src,
        value,
        result
    );
    emu.set_last_error(0);
    call.set_return(emu, result as u64);
}
