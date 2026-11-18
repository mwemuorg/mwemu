use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn FindStringOrdinal(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let flags = api_try!(emu, call.arg_u32(emu, 0));
    let src_ptr = api_try!(emu, call.arg(emu, 1));
    let src_len = api_try!(emu, call.arg_i32(emu, 2));
    let value_ptr = api_try!(emu, call.arg(emu, 3));
    let value_len = api_try!(emu, call.arg_i32(emu, 4));
    let ignore_case = api_try!(emu, call.arg(emu, 5)) != 0;
    let src = if src_ptr != 0 {
        emu.maps.read_wide_string(src_ptr)
    } else {
        String::new()
    };
    let value = if value_ptr != 0 {
        emu.maps.read_wide_string(value_ptr)
    } else {
        String::new()
    };
    let result = if ignore_case {
        src.to_lowercase().find(&value.to_lowercase())
    } else {
        src.find(&value)
    };
    let idx = result.map(|p| p as i64).unwrap_or(-1);
    log_red!(
        emu,
        "kernelbase!FindStringOrdinal '{}' in '{}' => {}",
        value,
        src,
        idx
    );
    emu.set_last_error(0);
    call.set_return(emu, idx as u64);
}
