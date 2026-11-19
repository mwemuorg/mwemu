use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::syscall::linux::constants::errors::{EFAULT, EINVAL, ENOMEM, EPERM};
use crate::syscall::linux::constants::syscalls::NR64_SIGALTSTACK;
use crate::syscall::linux::constants::system::{
    MINSIGSTKSZ, SS_AUTODISARM, SS_DISABLE, SS_ONSTACK,
};
use crate::threading::context::SignalStack;

const STACK64_SIZE: usize = 24;
const STACK64_FLAGS_OFFSET: u64 = 8;
const STACK64_SIZE_OFFSET: u64 = 16;

pub(super) fn dispatch(emu: &mut Emu) -> bool {
    match emu.regs().rax {
        NR64_SIGALTSTACK => sigaltstack(emu),
        _ => return false,
    }
    true
}

fn on_signal_stack(stack: SignalStack, sp: u64) -> bool {
    stack.flags & SS_AUTODISARM == 0 && sp > stack.pointer && sp - stack.pointer <= stack.size
}

fn read_stack(emu: &Emu, address: u64) -> Result<SignalStack, u64> {
    if !emu
        .maps
        .can_access_range(address, STACK64_SIZE, Permission::READ)
    {
        return Err(EFAULT);
    }
    Ok(SignalStack {
        pointer: emu.maps.read_qword(address).ok_or(EFAULT)?,
        flags: emu
            .maps
            .read_dword(address + STACK64_FLAGS_OFFSET)
            .ok_or(EFAULT)?,
        size: emu
            .maps
            .read_qword(address + STACK64_SIZE_OFFSET)
            .ok_or(EFAULT)?,
    })
}

fn update_stack(emu: &mut Emu, input: u64, output: u64) -> Result<(), u64> {
    let next = if input == 0 {
        None
    } else {
        Some(read_stack(emu, input)?)
    };
    let old = emu.threads[emu.current_thread_id].signal_stack;
    let on_stack = on_signal_stack(old, emu.regs().rsp);
    if let Some(mut next) = next {
        if on_stack {
            return Err(EPERM);
        }
        match next.flags & !SS_AUTODISARM {
            SS_DISABLE => {
                next.pointer = 0;
                next.size = 0;
            }
            0 | SS_ONSTACK if next.size >= MINSIGSTKSZ => {}
            0 | SS_ONSTACK => return Err(ENOMEM),
            _ => return Err(EINVAL),
        }
        emu.threads[emu.current_thread_id].signal_stack = next;
    }
    if output != 0 {
        let flags = (old.flags & SS_AUTODISARM)
            | if old.size == 0 {
                SS_DISABLE
            } else if on_stack {
                SS_ONSTACK
            } else {
                0
            };
        let mut bytes = [0; STACK64_SIZE];
        bytes[..8].copy_from_slice(&old.pointer.to_le_bytes());
        bytes[8..12].copy_from_slice(&flags.to_le_bytes());
        bytes[16..].copy_from_slice(&old.size.to_le_bytes());
        // Linux updates the stack before copying old_ss, even if that copy faults.
        if !emu.maps.write_bytes(output, &bytes) {
            return Err(EFAULT);
        }
    }
    Ok(())
}

// Stack registration/query only; signal delivery remains outside this model.
fn sigaltstack(emu: &mut Emu) {
    let input = emu.regs().rdi;
    let output = emu.regs().rsi;
    emu.regs_mut().rax = match update_stack(emu, input, output) {
        Ok(()) => 0,
        Err(error) => error,
    };
    log::trace!(
        "syscall sigaltstack(ss:0x{input:x} old_ss:0x{output:x}) ={}",
        emu.regs().rax as i64
    );
}
