use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetFullPathNameA(emu: &mut emu::Emu) {
    get_full_path_name(emu, "GetFullPathNameA", false);
}

pub fn GetFullPathNameW(emu: &mut emu::Emu) {
    get_full_path_name(emu, "GetFullPathNameW", true);
}

fn get_full_path_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let file_ptr = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg_u32(emu, 1));
    let buff = api_try!(emu, call.arg(emu, 2));
    let file_part_ptr = api_try!(emu, call.arg(emu, 3));
    let filename = strings::read_text(emu, file_ptr, wide);
    let full_path = if filename.contains('\\') || filename.contains('/') {
        filename.clone()
    } else {
        format!("{}\\{}", emu.cfg.cwd_path, filename)
    };
    log_red!(emu, "kernel32!{api} '{}' => '{}'", filename, full_path);
    if wide {
        let needed = full_path.chars().count() as u32 + 1;
        if size == 0 || buff == 0 {
            call.set_return(emu, needed as u64);
            return;
        }
        if size < needed {
            call.set_return(emu, needed as u64);
            return;
        }
        strings::write_string(emu, buff, &full_path, wide);
        if file_part_ptr != 0
            && let Some(pos) = full_path.rfind('\\')
        {
            let offset = (pos + 1) as u64 * 2;
            emu.maps.write_qword(file_part_ptr, buff + offset);
        }
        call.set_return(emu, (needed - 1) as u64);
    } else {
        let needed = full_path.len() as u32 + 1;
        if size == 0 || buff == 0 {
            call.set_return(emu, needed as u64);
            return;
        }
        if size < needed {
            call.set_return(emu, needed as u64);
            return;
        }
        strings::write_string(emu, buff, &full_path, wide);
        if file_part_ptr != 0
            && let Some(pos) = full_path.rfind('\\')
        {
            let offset = pos as u64 + 1;
            emu.maps.write_qword(file_part_ptr, buff + offset);
        }
        call.set_return(emu, (needed - 1) as u64);
    }
}
