use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetDateFormatA(emu: &mut Emu) {
    invoke(emu, false, "GetDateFormatA");
}

pub fn GetDateFormatW(emu: &mut Emu) {
    invoke(emu, true, "GetDateFormatW");
}

fn invoke(emu: &mut Emu, wide: bool, api: &str) {
    let call = WinApi(Stdcall);
    let argument_0 = api_try!(emu, call.arg(emu, 0));
    let argument_1 = api_try!(emu, call.arg(emu, 1));
    let output = api_try!(emu, call.arg(emu, 4));
    let capacity = api_try!(emu, call.arg_i32(emu, 5));
    log_red!(
        emu,
        "kernel32!{} locale:0x{:x} flags:0x{:x}",
        api,
        argument_0,
        argument_1
    );
    let return_value = strings::write_text(emu, output, capacity, "01/01/2024", wide);
    call.set_return(emu, return_value);
}
