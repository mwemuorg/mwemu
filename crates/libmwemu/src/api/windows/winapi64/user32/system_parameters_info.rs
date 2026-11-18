use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn SystemParametersInfoA(emu: &mut emu::Emu) {
    system_parameters_info(emu, "SystemParametersInfoA", false);
}

pub fn SystemParametersInfoW(emu: &mut emu::Emu) {
    system_parameters_info(emu, "SystemParametersInfoW", true);
}

fn system_parameters_info(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let ui_action = api_try!(emu, call.arg_u32(emu, 0));
    let ui_param = api_try!(emu, call.arg_u32(emu, 1));
    let pv_param = api_try!(emu, call.arg(emu, 2));
    let _f_win_ini = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "user32!{api} action:0x{:x} param:{} pvParam:0x{:x}",
        ui_action,
        ui_param,
        pv_param
    );
    if wide {
        if pv_param != 0 {
            match ui_action {
                0x0030 => {
                    emu.maps.write_dword(pv_param, 0);
                    emu.maps.write_dword(pv_param + 4, 0);
                    emu.maps.write_dword(pv_param + 8, 1920);
                    emu.maps.write_dword(pv_param + 12, 1040);
                }
                0x0048 => {
                    emu.maps.write_dword(pv_param, ui_param);
                }
                _ => {}
            }
        }
    } else {
        if pv_param != 0 {
            match ui_action {
                0x0030 => {
                    // SPI_GETWORKAREA - fill RECT
                    emu.maps.write_dword(pv_param, 0);
                    emu.maps.write_dword(pv_param + 4, 0);
                    emu.maps.write_dword(pv_param + 8, 1920);
                    emu.maps.write_dword(pv_param + 12, 1040);
                }
                0x0048 => {
                    // SPI_GETNONCLIENTMETRICS
                    emu.maps.write_dword(pv_param, ui_param);
                }
                _ => {}
            }
        }
    }

    call.set_return(emu, 1);
}
