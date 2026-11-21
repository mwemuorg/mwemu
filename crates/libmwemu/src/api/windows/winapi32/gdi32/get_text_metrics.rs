use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetTextMetricsA(emu: &mut emu::Emu) {
    get_text_metrics(emu, "GetTextMetricsA");
}

fn get_text_metrics(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg_u32(emu, 0));
    let tm = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    if tm != 0 {
        emu.maps.write_dword(tm, 16); // tmHeight
        emu.maps.write_dword(tm + 4, 13); // tmAscent
        emu.maps.write_dword(tm + 8, 3); // tmDescent
        emu.maps.write_dword(tm + 12, 0); // tmInternalLeading
        emu.maps.write_dword(tm + 16, 0); // tmExternalLeading
        emu.maps.write_dword(tm + 20, 7); // tmAveCharWidth
        emu.maps.write_dword(tm + 24, 14); // tmMaxCharWidth
    }

    log_red!(emu, "gdi32!{api} hdc:0x{:x} tm:0x{:x}", hdc, tm);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}

pub fn GetTextMetricsW(emu: &mut emu::Emu) {
    get_text_metrics(emu, "GetTextMetricsW");
}
