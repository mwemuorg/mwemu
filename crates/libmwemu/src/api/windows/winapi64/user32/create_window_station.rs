use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateWindowStationA(emu: &mut emu::Emu) {
    create_window_station(emu, "CreateWindowStationA", false);
}

fn create_window_station(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let ex_style = api_try!(emu, call.arg(emu, 0));
    let class_ptr = api_try!(emu, call.arg(emu, 1));
    let title_ptr = api_try!(emu, call.arg(emu, 2));
    let style = api_try!(emu, call.arg(emu, 3));
    let class_name = if class_ptr > 0xffff {
        strings::read_text(emu, class_ptr, wide)
    } else {
        format!("atom_{}", class_ptr)
    };
    let title = if title_ptr != 0 {
        strings::read_text(emu, title_ptr, wide)
    } else {
        String::new()
    };
    let hndl = helper::handler_create(&format!("window_{}", class_name));
    log_red!(
        emu,
        "user32!{api} class:'{}' title:'{}' => 0x{:x}",
        class_name,
        title,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, hndl);
}

pub fn CreateWindowStationW(emu: &mut emu::Emu) {
    create_window_station(emu, "CreateWindowStationW", true);
}
