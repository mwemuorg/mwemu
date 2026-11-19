use crate::api::abi::windows::{WindowsAbi, WindowsAbi::WinInterrupt};
use crate::api::windows::policy;
use crate::emu::Emu;
use crate::syscall::windows;

pub fn gateway(emu: &mut Emu) {
    unsupported(emu, emu.regs().get_eax(), WinInterrupt);
}

pub(crate) fn unsupported(emu: &mut Emu, number: u64, call: WindowsAbi) {
    // The INT 0x80 compatibility path has no established parameter layout.
    let name = if call == WinInterrupt {
        None
    } else {
        emu.syscall_name_by_real.get(&number).cloned()
    }
    .unwrap_or_else(|| format!("syscall 0x{number:x}"));
    windows::dispatch(emu, number, &name, |emu| {
        policy::unsupported_with_abi(emu, "ntdll", &name, emu.pc(), call);
    });
}
