use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;

pub fn _ftol(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let val = emu.fpu_mut().get_st(0);
    let result = val as i64;
    emu.fpu_mut().set_st(0, result as f64);
    log_red!(emu, "msvcrt!_ftol {} => {}", val, result);
    emu.set_last_error(0);
    call.set_return(emu, result as u64);
}
