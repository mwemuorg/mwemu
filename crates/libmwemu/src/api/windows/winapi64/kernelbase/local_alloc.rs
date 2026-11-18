use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64;

pub(super) fn LocalAlloc(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    // LocalAlloc(uFlags, uBytes) is equivalent to HeapAlloc(GetProcessHeap(), uFlags, uBytes)
    // rcx = uFlags, rdx = uBytes -> rcx = 0 (global heap), rdx = uFlags, r8 = uBytes
    let flags = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg(emu, 1));
    emu.regs_mut().rcx = 0;
    emu.regs_mut().rdx = flags;
    emu.regs_mut().r8 = size;
    winapi64::kernel32::HeapAlloc(emu);
}
