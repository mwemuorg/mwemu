use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::window_table;

pub(super) fn EnumWindows(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let enum_func = api_try!(emu, call.arg(emu, 0));
    let lparam = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "user32!EnumWindows lpEnumFunc: 0x{:x} lParam: 0x{:x}",
        enum_func,
        lparam
    );

    // BOOL CALLBACK EnumWindowsProc(HWND, LPARAM); FALSE stops the walk and
    // makes EnumWindows return FALSE too.
    for hwnd in window_table::top_level_windows() {
        match emu.call64(enum_func, &[hwnd, lparam]) {
            Ok(keep_going) if keep_going as u32 != 0 => {}
            Ok(_) => {
                call.set_return(emu, 0);
                return;
            }
            Err(e) => {
                log::warn!(
                    "EnumWindows callback 0x{:x} failed: {}",
                    enum_func,
                    e.message
                );
                call.set_return(emu, 0);
                return;
            }
        }
    }
    call.set_return(emu, 1);
}
