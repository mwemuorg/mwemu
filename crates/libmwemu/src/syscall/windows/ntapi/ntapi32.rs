use crate::emu;

pub fn gateway(syscall: u64, argv: u64, emu: &mut emu::Emu) {
    if !emu.call_syscall_hook(syscall) {
        return;
    }
    match syscall {
        0xdc => {
            log::trace!("/!\\ direct syscall: NtAlpcSendWaitReceivePort");
            emu.regs_mut().rax = 0;
        }

        0x10f => {
            log::trace!("/!\\ direct syscall: NtOpenFile {:x}", argv);
            emu.regs_mut().rax = 0;
        }

        _ => {
            let output = if let Some(decoded) = emu.last_decoded {
                emu.format_instruction(&decoded)
            } else {
                String::from("???")
            };
            log::trace!(
                "{}{} 0x{:x}: {}{}",
                emu.colors.red,
                emu.pos,
                emu.pc(),
                output,
                emu.colors.nc
            );
            unimplemented!();
        }
    }
}
