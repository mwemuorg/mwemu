use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn EnumFontFamiliesExA(emu: &mut emu::Emu) {
    enum_font_families_ex(emu, "EnumFontFamiliesExA");
}

fn enum_font_families_ex(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let logfont = api_try!(emu, call.arg(emu, 1));
    let proc_ptr = api_try!(emu, call.arg(emu, 2));
    let lparam = api_try!(emu, call.arg(emu, 3));
    let _flags = api_try!(emu, call.arg(emu, 4));
    log_red!(
        emu,
        "gdi32!{api} hdc:0x{:x} lf:0x{:x} proc:0x{:x} lparam:0x{:x}",
        hdc,
        logfont,
        proc_ptr,
        lparam
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn EnumFontFamiliesExW(emu: &mut emu::Emu) {
    enum_font_families_ex(emu, "EnumFontFamiliesExW");
}
