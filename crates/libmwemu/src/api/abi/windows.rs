//! Windows function-hook and syscall entry conventions.
use serde::{Deserialize, Serialize};

use crate::api::abi::{
    self, ARM64_REGISTER_ARGS, ArgumentFault, BYTE_BYTES, CallFrame, DWORD_BYTES, QWORD_BYTES,
    WORD_BYTES, set_f64_return, set_integer_return, set_u64_return,
};
use crate::arch::Arch;
use crate::emu::Emu;

use CallingConvention::{Stdcall, Thiscall};
use WindowsAbi::{WinApi, WinInterrupt, WinSyscall, WinSysenter};

const X64_REGISTER_ARGS: usize = 4;
const SYSENTER_RETURN_SLOTS: usize = 2;

/// Source-level function convention; x64/ARM64 use the native Windows ABI.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CallingConvention {
    Stdcall,
    Cdecl,
    /// Non-variadic member call with implicit `this` at argument zero.
    Thiscall,
}

// Call sites import these variants beside Unix ABI variants.
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowsAbi {
    /// Function hook after the dispatcher has consumed the return address.
    WinApi(CallingConvention),
    /// x64 SYSCALL interception before the instruction clobbers RCX.
    WinSyscall,
    /// x86 KiFastSystemCall: EDX points to the two return addresses before arguments.
    WinSysenter,
    /// Existing Windows INT 0x80 compatibility route, without a parameter ABI.
    WinInterrupt,
}

pub type ArgumentError = abi::ArgumentError<WindowsAbi>;

impl WindowsAbi {
    /// Native integer/pointer argument: DWORD slot on x86, parameter position on x64.
    /// Thiscall reserves index zero for `this`; its explicit x86 slots start at one.
    pub fn arg(self, emu: &Emu, index: usize) -> Result<u64, ArgumentError> {
        let size = if emu.cfg.arch.is_64bits() {
            QWORD_BYTES
        } else {
            DWORD_BYTES
        };
        self.integer_arg(emu, index, size)
    }

    pub fn arg_u8(self, emu: &Emu, index: usize) -> Result<u8, ArgumentError> {
        self.integer_arg(emu, index, BYTE_BYTES)
            .map(|value| value as u8)
    }

    pub fn arg_i8(self, emu: &Emu, index: usize) -> Result<i8, ArgumentError> {
        self.integer_arg(emu, index, BYTE_BYTES)
            .map(|value| value as i8)
    }

    pub fn arg_u16(self, emu: &Emu, index: usize) -> Result<u16, ArgumentError> {
        self.integer_arg(emu, index, WORD_BYTES)
            .map(|value| value as u16)
    }

    pub fn arg_i16(self, emu: &Emu, index: usize) -> Result<i16, ArgumentError> {
        self.integer_arg(emu, index, WORD_BYTES)
            .map(|value| value as i16)
    }

    pub fn arg_u32(self, emu: &Emu, index: usize) -> Result<u32, ArgumentError> {
        self.integer_arg(emu, index, DWORD_BYTES)
            .map(|value| value as u32)
    }

    pub fn arg_i32(self, emu: &Emu, index: usize) -> Result<i32, ArgumentError> {
        self.integer_arg(emu, index, DWORD_BYTES)
            .map(|value| value as i32)
    }

    /// A 64-bit integer occupies two consecutive DWORD slots on x86.
    pub fn arg_u64(self, emu: &Emu, index: usize) -> Result<u64, ArgumentError> {
        self.integer_arg(emu, index, QWORD_BYTES)
    }

    pub fn arg_i64(self, emu: &Emu, index: usize) -> Result<i64, ArgumentError> {
        self.integer_arg(emu, index, QWORD_BYTES)
            .map(|value| value as i64)
    }

    pub fn args<const N: usize>(self, emu: &Emu) -> Result<[u64; N], ArgumentError> {
        let mut values = [0; N];
        for (index, value) in values.iter_mut().enumerate() {
            *value = self.arg(emu, index)?;
        }
        Ok(values)
    }

    /// FP reads need both the x64 parameter position and the x86 DWORD slot.
    pub fn arg_f64(self, emu: &Emu, index: usize, x86_word: usize) -> Result<f64, ArgumentError> {
        self.fp_bits(emu, index, x86_word, QWORD_BYTES)
            .map(f64::from_bits)
    }

    pub fn arg_f32(self, emu: &Emu, index: usize, x86_word: usize) -> Result<f32, ArgumentError> {
        self.fp_bits(emu, index, x86_word, DWORD_BYTES)
            .map(|bits| f32::from_bits(bits as u32))
    }

    pub fn set_return(self, emu: &mut Emu, value: u64) {
        set_integer_return(emu, value);
    }

    /// Return in EDX:EAX on x86, otherwise in the integer return register.
    pub fn set_return_u64(self, emu: &mut Emu, value: u64) {
        set_u64_return(emu, value);
    }

    pub fn set_return_f64(self, emu: &mut Emu, value: f64) {
        set_f64_return(emu, value);
    }

    /// Discard x86 callee-owned DWORD slots; exclude the register-passed `this` pointer.
    /// The hook dispatcher has already consumed the return address.
    pub fn cleanup_args(self, emu: &mut Emu, words: usize) {
        if matches!(self, WinApi(Stdcall | Thiscall)) && !emu.cfg.arch.is_64bits() {
            // RET's immediate adjusts ESP without reading the discarded arguments.
            let bytes = (words as u32).wrapping_mul(DWORD_BYTES as u32);
            let sp = (emu.sp() as u32).wrapping_add(bytes);
            emu.set_sp(sp as u64);
        }
    }

    fn integer_arg(self, emu: &Emu, index: usize, size: usize) -> Result<u64, ArgumentError> {
        let frame = CallFrame::new(emu, self);
        if self == WinInterrupt
            || (self == WinSyscall && !emu.cfg.arch.is_x64())
            || (self == WinSysenter && emu.cfg.arch != Arch::X86)
        {
            return Err(frame.error(index, size, None, ArgumentFault::UnsupportedConvention));
        }
        if self == WinSysenter {
            let offset = index
                .checked_add(SYSENTER_RETURN_SLOTS)
                .and_then(|slot| slot.checked_mul(DWORD_BYTES));
            return frame.memory_arg(index, emu.regs().get_edx(), offset, size.max(DWORD_BYTES));
        }
        if emu.cfg.arch.is_aarch64() {
            if index < ARM64_REGISTER_ARGS {
                return Ok(emu.regs_aarch64().x[index]);
            }
            let offset = (index - ARM64_REGISTER_ARGS).checked_mul(QWORD_BYTES);
            return frame.stack_arg(index, offset, size);
        }
        if !emu.cfg.arch.is_64bits() {
            let slot = if self == WinApi(Thiscall) {
                if index == 0 {
                    if size > DWORD_BYTES {
                        return Err(frame.error(
                            index,
                            size,
                            None,
                            ArgumentFault::UnsupportedConvention,
                        ));
                    }
                    return Ok(emu.regs().get_ecx());
                }
                index - 1
            } else {
                index
            };
            // Narrow x86 parameters are passed as promoted DWORDs.
            return frame.stack_arg(index, slot.checked_mul(DWORD_BYTES), size.max(DWORD_BYTES));
        }
        let regs = emu.regs();
        let value = match index {
            0 => Some(regs.rcx),
            1 => Some(regs.rdx),
            2 => Some(regs.r8),
            3 => Some(regs.r9),
            _ => None,
        };
        if let Some(value) = value {
            return Ok(value);
        }
        // The engine intercepts SYSCALL before RCX changes and retains the return
        // address on the stack; API hooks have already consumed that address.
        let slot = if self == WinSyscall {
            index.checked_add(1)
        } else {
            Some(index)
        };
        frame.stack_arg(index, slot.and_then(|n| n.checked_mul(QWORD_BYTES)), size)
    }

    fn fp_bits(
        self,
        emu: &Emu,
        index: usize,
        x86_word: usize,
        size: usize,
    ) -> Result<u64, ArgumentError> {
        let frame = CallFrame::new(emu, self);
        if !matches!(self, WinApi(_))
            || emu.cfg.arch.is_aarch64()
            || (self == WinApi(Thiscall) && index == 0)
        {
            // Independent FP register allocation cannot be inferred from a parameter index.
            return Err(frame.error(index, size, None, ArgumentFault::UnsupportedConvention));
        }
        if !emu.cfg.arch.is_64bits() {
            frame.stack_arg(x86_word, x86_word.checked_mul(DWORD_BYTES), size)
        } else if index < X64_REGISTER_ARGS {
            Ok(match index {
                0 => emu.regs().xmm0 as u64,
                1 => emu.regs().xmm1 as u64,
                2 => emu.regs().xmm2 as u64,
                _ => emu.regs().xmm3 as u64,
            })
        } else {
            frame.stack_arg(index, index.checked_mul(QWORD_BYTES), size)
        }
    }
}
