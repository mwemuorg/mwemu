use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn PathCchAppend(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let cch_path = api_try!(emu, call.arg(emu, 1)) as usize;
    let more_ptr = api_try!(emu, call.arg(emu, 2));
    let path = emu.maps.read_wide_string(path_ptr);
    let more = emu.maps.read_wide_string(more_ptr);
    let result_path = if more.starts_with('\\') || (more.len() >= 2 && more.as_bytes()[1] == b':') {
        more.clone()
    } else if path.ends_with('\\') {
        format!("{}{}", path, more)
    } else {
        format!("{}\\{}", path, more)
    };
    emu.maps.write_wide_string(path_ptr, &result_path);
    log_red!(
        emu,
        "kernelbase!{} '{}' + '{}' => '{}'",
        api,
        path,
        more,
        result_path
    );
    call.set_return(emu, 0); // S_OK
}
