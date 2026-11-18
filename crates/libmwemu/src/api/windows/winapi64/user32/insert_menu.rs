use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn InsertMenuA(emu: &mut emu::Emu) {
    insert_menu(emu, "InsertMenuA", false);
}

fn insert_menu(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hmenu = api_try!(emu, call.arg(emu, 0));
    let position = api_try!(emu, call.arg_u32(emu, 1));
    let flags = api_try!(emu, call.arg_u32(emu, 2));
    let id_new_item = api_try!(emu, call.arg(emu, 3));
    let new_item_ptr = api_try!(emu, call.arg(emu, 4));
    let text = if new_item_ptr != 0 && (flags & 0x0040) == 0 {
        strings::read_text(emu, new_item_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "user32!{api} menu:0x{:x} pos:{} flags:0x{:x} '{}'",
        hmenu,
        position,
        flags,
        text
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn InsertMenuW(emu: &mut emu::Emu) {
    insert_menu(emu, "InsertMenuW", true);
}
