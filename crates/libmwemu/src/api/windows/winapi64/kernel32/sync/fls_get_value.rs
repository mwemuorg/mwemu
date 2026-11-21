use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn FlsGetValue(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let idx = api_try!(emu, call.arg(emu, 0));
    if idx as usize > emu.fls().len() {
        emu.set_last_error(0);
        call.set_return(emu, 0);
    } else {
        let return_value = emu.fls()[idx as usize];
        call.set_return(emu, return_value);
    }

    log_red!(
        emu,
        "kernel32!FlsGetValue idx: {} =0x{:x}",
        idx,
        emu.regs().get_eax() as u32
    );
}
