use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::policy::api_try;

fn enumerate(emu: &mut Emu, api: &str, children: bool) {
    let call = WinApi(Stdcall);
    let parent = if children {
        api_try!(emu, call.arg(emu, 0))
    } else {
        0
    };
    let callback = api_try!(emu, call.arg(emu, usize::from(children)));
    let application_data = api_try!(emu, call.arg(emu, usize::from(children) + 1));
    log_red!(
        emu,
        "user32!{api} parent={parent:#x} callback={callback:#x} application_data={application_data:#x}"
    );
    let completed = callback != 0
        && emu.guest_windows.enumerate(parent).into_iter().all(|hwnd| {
            if !emu.guest_windows.contains(hwnd) {
                return true;
            }
            let result = if emu.cfg.arch.is_64bits() {
                emu.call64(callback, &[hwnd, application_data])
                    .map(|value| value as u32)
            } else {
                emu.call32(callback, &[hwnd as u32, application_data as u32])
            };
            result.is_ok_and(|more| more != 0)
        });
    if !emu.api_policy.stopped() {
        call.set_return(emu, completed.into());
        call.cleanup_args(emu, 2 + usize::from(children));
    }
}

pub fn EnumWindows(emu: &mut Emu) {
    enumerate(emu, "EnumWindows", false);
}

pub fn EnumChildWindows(emu: &mut Emu) {
    enumerate(emu, "EnumChildWindows", true);
}
