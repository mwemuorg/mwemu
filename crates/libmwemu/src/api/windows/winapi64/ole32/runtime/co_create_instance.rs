use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::ole32::read_guid_string;

pub fn CoCreateInstance(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let rclsid = api_try!(emu, call.arg(emu, 0));
    let _punk_outer = api_try!(emu, call.arg(emu, 1));
    let dw_cls_context = api_try!(emu, call.arg(emu, 2));
    let riid = api_try!(emu, call.arg(emu, 3));
    let ppv = api_try!(emu, call.arg(emu, 4));

    let clsid_str = if rclsid != 0 {
        read_guid_string(emu, rclsid)
    } else {
        String::from("{NULL}")
    };
    let riid_str = if riid != 0 {
        read_guid_string(emu, riid)
    } else {
        String::from("{NULL}")
    };

    log_red!(
        emu,
        "ole32!CoCreateInstance clsid:{} ctx:0x{:x} riid:{} ppv:0x{:x}",
        clsid_str,
        dw_cls_context,
        riid_str,
        ppv
    );

    if ppv != 0 {
        emu.maps.write_qword(ppv, 0);
    }

    emu.set_last_error(0);
    call.set_return(emu, 0x80040154); // REGDB_E_CLASSNOTREG
}
