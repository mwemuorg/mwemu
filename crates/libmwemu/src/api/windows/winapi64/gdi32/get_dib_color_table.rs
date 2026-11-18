use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetDIBColorTable(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let start = api_try!(emu, call.arg(emu, 1));
    let entries = api_try!(emu, call.arg(emu, 2));
    let colors = api_try!(emu, call.arg(emu, 3));
    if colors != 0 && entries > 0 {
        let n = entries.min(256) as usize;
        for i in 0..n {
            let offset = colors + (i as u64) * 4;
            let gray = (i * 255 / n.max(1)) as u32;
            let rgbq = gray | (gray << 8) | (gray << 16);
            emu.maps.write_dword(offset, rgbq);
        }
    }
    log_red!(
        emu,
        "gdi32!GetDIBColorTable hdc:0x{:x} start:{} entries:{} colors:0x{:x}",
        hdc,
        start,
        entries,
        colors
    );
    emu.set_last_error(0);
    call.set_return(emu, entries.min(256));
}
