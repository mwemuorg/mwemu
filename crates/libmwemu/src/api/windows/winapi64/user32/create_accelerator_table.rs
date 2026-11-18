use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateAcceleratorTableA(emu: &mut emu::Emu) {
    create_accelerator_table(emu, "CreateAcceleratorTableA");
}

fn create_accelerator_table(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let accel_ptr = api_try!(emu, call.arg(emu, 0));
    let num_entries = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(
        emu,
        "user32!{api} accel:0x{:x} count:{}",
        accel_ptr,
        num_entries
    );
    let hndl = helper::handler_create(&format!("user32://{api}"));
    call.set_return(emu, hndl);
}

pub fn CreateAcceleratorTableW(emu: &mut emu::Emu) {
    create_accelerator_table(emu, "CreateAcceleratorTableW");
}
