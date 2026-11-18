use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SetMapMode(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let mode = api_try!(emu, call.arg(emu, 1));
    let mode_name = match mode {
        1 => "MM_TEXT",
        2 => "MM_LOMETRIC",
        3 => "MM_HIMETRIC",
        4 => "MM_LOENGLISH",
        5 => "MM_HIENGLISH",
        6 => "MM_TWIPS",
        7 => "MM_ISOTROPIC",
        8 => "MM_ANISOTROPIC",
        _ => "UNKNOWN",
    };
    log_red!(
        emu,
        "gdi32!SetMapMode hdc:0x{:x} mode:{} ({})",
        hdc,
        mode,
        mode_name
    );
    emu.set_last_error(0);
    call.set_return(emu, 1); // previous mode: MM_TEXT
}
