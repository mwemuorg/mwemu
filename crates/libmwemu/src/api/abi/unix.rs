//! Unix function-hook conventions; syscall entry is a separate adapter.
use serde::{Deserialize, Serialize};

use crate::api::abi::{
    self, ARM64_REGISTER_ARGS, ArgumentFault, BYTE_BYTES, CallFrame, DWORD_BYTES, QWORD_BYTES,
    WORD_BYTES, set_integer_return,
};
use crate::emu::Emu;

const SYSTEM_V_REGISTER_ARGS: usize = 6;

/// System V AMD64 and the currently supported AAPCS64 register arguments.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnixApi;

/// Integer variadic arguments; floating-point and aggregate classification
/// still require a signature-aware adapter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VariadicConvention {
    SystemV,
    Darwin,
}

pub type ArgumentError = abi::ArgumentError<UnixApi>;

impl UnixApi {
    pub fn arg(self, emu: &Emu, index: usize) -> Result<u64, ArgumentError> {
        self.integer_arg(emu, index, QWORD_BYTES)
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

    pub fn arg_u64(self, emu: &Emu, index: usize) -> Result<u64, ArgumentError> {
        self.integer_arg(emu, index, QWORD_BYTES)
    }

    pub fn arg_i64(self, emu: &Emu, index: usize) -> Result<i64, ArgumentError> {
        self.integer_arg(emu, index, QWORD_BYTES)
            .map(|value| value as i64)
    }

    fn integer_arg(self, emu: &Emu, index: usize, size: usize) -> Result<u64, ArgumentError> {
        let frame = CallFrame::new(emu, self);
        if emu.cfg.arch.is_aarch64() && index < ARM64_REGISTER_ARGS {
            return Ok(emu.regs_aarch64().x[index]);
        }
        if !emu.cfg.arch.is_x64() {
            // Unix ARM64 stack packing/varargs need a signature-aware adapter.
            // Preserve the former register-only boundary rather than assume x64 slots.
            return Err(frame.error(index, size, None, ArgumentFault::UnsupportedConvention));
        }
        let regs = emu.regs();
        let value = match index {
            0 => Some(regs.rdi),
            1 => Some(regs.rsi),
            2 => Some(regs.rdx),
            3 => Some(regs.rcx),
            4 => Some(regs.r8),
            5 => Some(regs.r9),
            _ => None,
        };
        if let Some(value) = value {
            return Ok(value);
        }
        let offset = (index - SYSTEM_V_REGISTER_ARGS).checked_mul(QWORD_BYTES);
        frame.stack_arg(index, offset, size)
    }

    pub fn integer_vararg(
        self,
        emu: &Emu,
        first: usize,
        number: usize,
        convention: VariadicConvention,
    ) -> Result<u64, ArgumentError> {
        let frame = CallFrame::new(emu, self);
        let index = first.checked_add(number).ok_or_else(|| {
            frame.error(
                usize::MAX,
                QWORD_BYTES,
                None,
                ArgumentFault::AddressOverflow,
            )
        })?;
        if emu.cfg.arch.is_aarch64() {
            let slot = match convention {
                VariadicConvention::Darwin => Some(number),
                VariadicConvention::SystemV => index.checked_sub(ARM64_REGISTER_ARGS),
            };
            if let Some(slot) = slot {
                return frame.stack_arg(index, slot.checked_mul(QWORD_BYTES), QWORD_BYTES);
            }
        }
        self.arg(emu, index)
    }

    pub fn set_return(self, emu: &mut Emu, value: u64) {
        set_integer_return(emu, value);
    }
}
