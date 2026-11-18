use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn PathCchCombine(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let out_ptr = api_try!(emu, call.arg(emu, 0));
    let cch_out = api_try!(emu, call.arg(emu, 1)) as usize;
    let dir_ptr = api_try!(emu, call.arg(emu, 2));
    let file_ptr = api_try!(emu, call.arg(emu, 3));
    let dir = emu.maps.read_wide_string(dir_ptr);
    let file = emu.maps.read_wide_string(file_ptr);
    let result_path = if file.starts_with('\\') || (file.len() >= 2 && file.as_bytes()[1] == b':') {
        file.clone()
    } else if dir.ends_with('\\') {
        format!("{}{}", dir, file)
    } else {
        format!("{}\\{}", dir, file)
    };
    emu.maps.write_wide_string(out_ptr, &result_path);
    log_red!(
        emu,
        "kernelbase!{} '{}' + '{}' => '{}'",
        api,
        dir,
        file,
        result_path
    );
    call.set_return(emu, 0); // S_OK
}
