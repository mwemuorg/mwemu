use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _EH_prolog(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let handler = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "msvcrt!_EH_prolog handler:0x{:x}", handler);
    emu.stack_push32(emu.regs().get_ebp() as u32);
}
