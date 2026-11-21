use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::policy;

pub fn __argc(emu: &mut Emu) {
    policy::unsupported_with_abi(emu, "kernelbase", "__argc", emu.pc(), WinApi(Cdecl));
}
