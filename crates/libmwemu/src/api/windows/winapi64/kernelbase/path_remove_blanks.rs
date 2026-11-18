use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn PathRemoveBlanksA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    if path_ptr != 0 {
        let path = emu.maps.read_string(path_ptr);
        let trimmed = path.trim();
        emu.maps.write_string(path_ptr, trimmed);
        log_red!(
            emu,
            "kernelbase!PathRemoveBlanksA '{}' => '{}'",
            path,
            trimmed
        );
    }
}

pub fn PathRemoveBlanksW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    if path_ptr != 0 {
        let path = emu.maps.read_wide_string(path_ptr);
        let trimmed = path.trim().to_string();
        emu.maps.write_wide_string(path_ptr, &trimmed);
        log_red!(
            emu,
            "kernelbase!PathRemoveBlanksW '{}' => '{}'",
            path,
            trimmed
        );
    }
}
