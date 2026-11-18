use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn PathCchStripPrefix(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let cch_path = api_try!(emu, call.arg(emu, 1)) as usize;
    let path = emu.maps.read_wide_string(path_ptr);
    let result_path = if let Some(stripped) = path.strip_prefix("\\\\?\\") {
        stripped.to_string()
    } else {
        path.clone()
    };
    emu.maps.write_wide_string(path_ptr, &result_path);
    log_red!(
        emu,
        "kernelbase!PathCchStripPrefix '{}' => '{}'",
        path,
        result_path
    );
    call.set_return(emu, 0); // S_OK
}
