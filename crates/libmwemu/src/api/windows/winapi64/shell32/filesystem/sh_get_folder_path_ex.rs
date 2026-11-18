use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SHGetFolderPathEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let rfid = api_try!(emu, call.arg(emu, 0));
    let dw_flags = api_try!(emu, call.arg(emu, 1));
    let htoken = api_try!(emu, call.arg(emu, 2));
    let psz_path = api_try!(emu, call.arg(emu, 3));
    let cch_path = api_try!(emu, call.arg_u32(emu, 5));
    let path = "C:\\Users\\user";
    if psz_path != 0 {
        for (i, ch) in path.encode_utf16().enumerate() {
            emu.maps.write_word(psz_path + (i as u64) * 2, ch);
        }
        emu.maps.write_word(psz_path + (path.len() as u64) * 2, 0);
    }
    log_red!(emu, "shell32!SHGetFolderPathEx -> '{}'", path);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
