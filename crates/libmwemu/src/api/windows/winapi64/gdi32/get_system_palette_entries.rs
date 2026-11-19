use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetSystemPaletteEntries(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let start = api_try!(emu, call.arg_u32(emu, 1));
    let entries = api_try!(emu, call.arg_u32(emu, 2));
    let pe = api_try!(emu, call.arg(emu, 3));
    if pe != 0 && entries > 0 {
        for i in 0..entries.min(256) {
            let offset = pe + (i as u64) * 4;
            let gray = ((start + i) * 255 / 256) as u8;
            let entry = (gray as u32) | ((gray as u32) << 8) | ((gray as u32) << 16);
            emu.maps.write_dword(offset, entry);
        }
    }
    log_red!(
        emu,
        "gdi32!GetSystemPaletteEntries hdc:0x{:x} start:{} entries:{} pe:0x{:x}",
        hdc,
        start,
        entries,
        pe
    );
    emu.set_last_error(0);
    call.set_return(emu, entries.min(256) as u64);
}
