use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub fn GetTickCount64(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!GetTickCount64");
    // ULONGLONG comes back in edx:eax
    let tick = emu.tick as u64;
    call.set_return(emu, tick & 0xffff_ffff);
    emu.regs_mut().rdx = tick >> 32;
    call.cleanup_args(emu, 0);
}
