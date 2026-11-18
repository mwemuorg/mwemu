use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn InternetDial(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let connect_ptr = api_try!(emu, call.arg(emu, 1));
    let conn_id_ptr = api_try!(emu, call.arg(emu, 2));
    let flags = api_try!(emu, call.arg_u32(emu, 3));
    let connect = if connect_ptr != 0 {
        emu.maps.read_string(connect_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!InternetDial connect:'{}' flags:0x{:x}",
        connect,
        flags
    );
    if conn_id_ptr != 0 {
        emu.maps.write_dword(conn_id_ptr, 1);
    }
    call.set_return(emu, 0);
}

pub fn InternetDialA(emu: &mut emu::Emu) {
    internet_dial(emu, "InternetDialA", false);
}

fn internet_dial(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let connect_ptr = api_try!(emu, call.arg(emu, 1));
    let conn_id_ptr = api_try!(emu, call.arg(emu, 2));
    let flags = api_try!(emu, call.arg_u32(emu, 3));
    let connect = if connect_ptr != 0 {
        strings::read_text(emu, connect_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!{api} connect:'{}' flags:0x{:x}",
        connect,
        flags
    );
    if conn_id_ptr != 0 {
        emu.maps.write_dword(conn_id_ptr, 1);
    }
    call.set_return(emu, 0);
}

pub fn InternetDialW(emu: &mut emu::Emu) {
    internet_dial(emu, "InternetDialW", true);
}
