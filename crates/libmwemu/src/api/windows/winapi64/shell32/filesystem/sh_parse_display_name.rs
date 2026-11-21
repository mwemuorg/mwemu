use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SHParseDisplayName(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let psz_name = api_try!(emu, call.arg(emu, 0));
    let pbc = api_try!(emu, call.arg(emu, 1));
    let ppidl = api_try!(emu, call.arg(emu, 2));
    let sfgao_in = api_try!(emu, call.arg(emu, 3));
    let psfgao_out = api_try!(emu, call.arg(emu, 4));
    let name = emu.maps.read_wide_string(psz_name);
    let hndl = helper::handler_create(&format!("pidl://{}", name));
    if ppidl != 0 {
        emu.maps.write_qword(ppidl, hndl);
    }
    if psfgao_out != 0 {
        emu.maps.write_dword(psfgao_out, sfgao_in as u32);
    }
    log_red!(emu, "shell32!SHParseDisplayName '{}' =0x{:x}", name, hndl);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
