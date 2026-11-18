use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SetParent(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd_child = api_try!(emu, call.arg(emu, 0));
    let hwnd_parent = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "user32!SetParent child:0x{:x} parent:0x{:x}",
        hwnd_child,
        hwnd_parent
    );
    let val_ptr = api_try!(emu, call.arg(emu, 1));
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    call.set_return(emu, hwnd_parent);
}
