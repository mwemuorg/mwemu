use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn PathCchRenameExtension(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let cch_path = api_try!(emu, call.arg(emu, 1)) as usize;
    let ext_ptr = api_try!(emu, call.arg(emu, 2));
    let path = emu.maps.read_wide_string(path_ptr);
    let ext = emu.maps.read_wide_string(ext_ptr);
    let base = if let Some(pos) = path.rfind('.') {
        if path[pos..].contains('\\') {
            path.clone()
        } else {
            path[..pos].to_string()
        }
    } else {
        path.clone()
    };
    let dot_ext = if ext.starts_with('.') {
        ext.clone()
    } else {
        format!(".{}", ext)
    };
    let result_path = format!("{}{}", base, dot_ext);
    emu.maps.write_wide_string(path_ptr, &result_path);
    log_red!(
        emu,
        "kernelbase!PathCchRenameExtension '{}' + '{}' => '{}'",
        path,
        ext,
        result_path
    );
    call.set_return(emu, 0); // S_OK
}
