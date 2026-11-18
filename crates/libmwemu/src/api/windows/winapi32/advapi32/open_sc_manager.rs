use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn OpenSCManagerA(emu: &mut emu::Emu) {
    open_sc_manager(emu, "OpenSCManagerA", false);
}

fn open_sc_manager(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let machine_ptr = api_try!(emu, call.arg(emu, 0));
    let database_ptr = api_try!(emu, call.arg_u32(emu, 1));
    let access = api_try!(emu, call.arg_u32(emu, 2));
    let machine = if machine_ptr != 0 {
        strings::read_text(emu, machine_ptr, wide)
    } else {
        String::new()
    };
    let hsc = helper::handler_create("scmanager");
    log_red!(
        emu,
        "advapi32!{api} machine:'{}' access:0x{:x} => 0x{:x}",
        machine,
        access,
        hsc
    );
    call.cleanup_args(emu, 3);
    call.set_return(emu, hsc);
}

pub fn OpenSCManagerW(emu: &mut emu::Emu) {
    open_sc_manager(emu, "OpenSCManagerW", true);
}
