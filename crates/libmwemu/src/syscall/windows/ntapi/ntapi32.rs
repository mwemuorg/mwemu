use crate::api::abi::windows::WindowsAbi::WinSysenter;
use crate::emu::Emu;
use crate::syscall::windows::syscall32;

pub fn gateway(syscall: u64, _argv: u64, emu: &mut Emu) {
    syscall32::dispatch_service(emu, syscall, WinSysenter);
}
