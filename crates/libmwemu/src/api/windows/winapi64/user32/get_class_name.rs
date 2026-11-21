use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetClassNameA(emu: &mut emu::Emu) {
    get_class_name(emu, "GetClassNameA", false);
}

fn get_class_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let buf = api_try!(emu, call.arg(emu, 1));
    let max_count = api_try!(emu, call.arg(emu, 2)) as usize;
    let class_name = "mwemu_class";
    if buf != 0 && max_count > 0 {
        strings::write_string(emu, buf, class_name, wide);
    }
    log_red!(emu, "user32!{api} hwnd:0x{:x} => '{}'", hwnd, class_name);

    emu.set_last_error(0);
    call.set_return(emu, class_name.len() as u64);
}

pub fn GetClassNameW(emu: &mut emu::Emu) {
    get_class_name(emu, "GetClassNameW", true);
}
