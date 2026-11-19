use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetCurrencyFormatEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _locale_ptr = api_try!(emu, call.arg(emu, 0));
    let _flags = api_try!(emu, call.arg(emu, 1));
    let value_ptr = api_try!(emu, call.arg(emu, 2));
    let _format_ptr = api_try!(emu, call.arg(emu, 3));
    let buf_ptr = api_try!(emu, call.arg(emu, 4));
    let buf_size = api_try!(emu, call.arg_i32(emu, 5));
    let value = if value_ptr != 0 {
        emu.maps.read_wide_string(value_ptr)
    } else {
        "0".to_string()
    };
    let formatted = format!("${}", value);
    if buf_ptr != 0 && buf_size > 0 {
        emu.maps.write_wide_string(buf_ptr, &formatted);
    }
    log_red!(
        emu,
        "kernelbase!GetCurrencyFormatEx '{}' => '{}'",
        value,
        formatted
    );
    emu.set_last_error(0);
    call.set_return(emu, (formatted.len() + 1) as u64);
}
