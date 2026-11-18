use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn crt_ftol(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let val = emu.fpu_mut().get_st(0);
    log_red!(emu, "wincrt!_ftol {} => {}", val, val as i64);
    call.set_return(emu, val as i64 as u64);
}
