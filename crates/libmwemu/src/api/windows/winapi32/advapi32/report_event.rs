use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn ReportEventA(emu: &mut emu::Emu) {
    report_event(emu, "ReportEventA");
}

fn report_event(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let hevt_log = api_try!(emu, call.arg(emu, 0));
    let wtype = api_try!(emu, call.arg_u32(emu, 1));
    let category = api_try!(emu, call.arg_u32(emu, 2));
    let event_id = api_try!(emu, call.arg_u32(emu, 3));
    let _user_sid = api_try!(emu, call.arg_u32(emu, 4));
    let num_strings = api_try!(emu, call.arg_u32(emu, 5));
    let _data_size = api_try!(emu, call.arg_u32(emu, 6));
    let _strings_ptr = api_try!(emu, call.arg_u32(emu, 7));
    let _raw_data = api_try!(emu, call.arg_u32(emu, 8));
    log_red!(
        emu,
        "advapi32!{api} log:0x{:x} type:{} cat:{} id:0x{:x} nstr:{}",
        hevt_log,
        wtype,
        category,
        event_id,
        num_strings
    );
    call.cleanup_args(emu, 9);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn ReportEventW(emu: &mut emu::Emu) {
    report_event(emu, "ReportEventW");
}
