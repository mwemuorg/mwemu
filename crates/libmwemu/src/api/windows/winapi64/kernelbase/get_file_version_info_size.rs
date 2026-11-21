use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn GetFileVersionInfoSizeA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lptstr_filename = api_try!(emu, call.arg(emu, 0));
    let lpdw_handle = api_try!(emu, call.arg(emu, 1)) as usize;

    let filename = if lptstr_filename > 0 {
        emu.maps.read_string(lptstr_filename)
    } else {
        "unknown".to_string()
    };

    log_red!(
        emu,
        "** {} kernelbase!GetFileVersionInfoSizeA filename: {} lpdw_handle: 0x{:x}",
        emu.pos,
        filename,
        lpdw_handle
    );

    if filename == "comctl32.dll" {
        let dll_path = emu.cfg.get_maps_folder("comctl32.dll");
        match std::fs::metadata(dll_path) {
            Ok(metadata) => {
                call.set_return(emu, metadata.len());
            }
            Err(e) => {
                log::warn!("GetFileVersionInfoSizeA: cannot stat comctl32.dll: {}", e);
                call.set_return(emu, 0);
            }
        }
    } else {
        log::warn!("GetFileVersionInfoSizeA: unsupported file {}", filename);
        call.set_return(emu, 0);
    }
}

pub fn GetFileVersionInfoSizeW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let p2 = api_try!(emu, call.arg(emu, 2));
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernelbase!GetFileVersionInfoSizeW '{}'  p1:0x{:x} p2:0x{:x}",
        s,
        p1,
        p2
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
