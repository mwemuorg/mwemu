use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn SHGetFolderPathAndSubDirA(emu: &mut emu::Emu) {
    sh_get_folder_path_and_sub_dir(emu, "SHGetFolderPathAndSubDirA", false);
}

pub fn SHGetFolderPathAndSubDirW(emu: &mut emu::Emu) {
    sh_get_folder_path_and_sub_dir(emu, "SHGetFolderPathAndSubDirW", true);
}

fn sh_get_folder_path_and_sub_dir(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let csidl = api_try!(emu, call.arg_i32(emu, 1));
    let htoken = api_try!(emu, call.arg(emu, 2));
    let dw_flags = api_try!(emu, call.arg(emu, 3));
    let psz_sub_dir = api_try!(emu, call.arg(emu, 4));
    let psz_path = api_try!(emu, call.arg(emu, 5));
    let sub_dir = if psz_sub_dir != 0 {
        strings::read_text(emu, psz_sub_dir, wide)
    } else {
        String::new()
    };
    let path = format!("C:\\Users\\user\\AppData\\{}", sub_dir);
    if wide {
        if psz_path != 0 {
            for (i, ch) in path.encode_utf16().enumerate() {
                emu.maps.write_word(psz_path + (i as u64) * 2, ch);
            }
            emu.maps.write_word(psz_path + (path.len() as u64) * 2, 0);
        }
    } else {
        if psz_path != 0 {
            strings::write_string(emu, psz_path, &path, wide);
        }
    }

    log_red!(
        emu,
        "shell32!{api} csidl:0x{:x} sub:'{}' -> '{}'",
        csidl,
        sub_dir,
        path
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
