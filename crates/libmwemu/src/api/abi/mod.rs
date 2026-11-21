//! Shared frame access and scalar result storage; platform adapters own conventions.
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::emu::Emu;

pub mod unix;
pub mod windows;

const BYTE_BYTES: usize = size_of::<u8>();
const WORD_BYTES: usize = size_of::<u16>();
const DWORD_BYTES: usize = size_of::<u32>();
const QWORD_BYTES: usize = size_of::<u64>();
const ARM64_REGISTER_ARGS: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArgumentFault {
    AddressOverflow,
    Unreadable,
    UnsupportedConvention,
}

/// A failed frame read is not a zero/NULL argument or an API error result.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgumentError<A> {
    pub abi: A,
    pub index: usize,
    pub address: Option<u64>,
    pub size: usize,
    pub fault: ArgumentFault,
}

impl<A: fmt::Debug> fmt::Display for ArgumentError<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} argument {} ({} bytes): {:?}",
            self.abi, self.index, self.size, self.fault
        )?;
        if let Some(address) = self.address {
            write!(f, " at {address:#x}")?;
        }
        Ok(())
    }
}

impl<A: fmt::Debug> std::error::Error for ArgumentError<A> {}

/// Attach adapter context to checked memory reads without choosing a platform policy.
struct CallFrame<'a, A> {
    emu: &'a Emu,
    abi: A,
}

impl<'a, A: Copy> CallFrame<'a, A> {
    fn new(emu: &'a Emu, abi: A) -> Self {
        Self { emu, abi }
    }

    fn error(
        &self,
        index: usize,
        size: usize,
        address: Option<u64>,
        fault: ArgumentFault,
    ) -> ArgumentError<A> {
        ArgumentError {
            abi: self.abi,
            index,
            address,
            size,
            fault,
        }
    }

    fn stack_arg(
        &self,
        index: usize,
        offset: Option<usize>,
        size: usize,
    ) -> Result<u64, ArgumentError<A>> {
        let emu = self.emu;
        let overflow = || self.error(index, size, None, ArgumentFault::AddressOverflow);
        let address = emu
            .sp()
            .checked_add(offset.ok_or_else(overflow)? as u64)
            .ok_or_else(overflow)?;
        let last = address
            .checked_add((size - 1) as u64)
            .ok_or_else(overflow)?;
        if !emu.cfg.arch.is_64bits() && last > u32::MAX as u64 {
            return Err(overflow());
        }
        let value = match size {
            BYTE_BYTES => emu.maps.read_byte(address).map(u64::from),
            WORD_BYTES => emu.maps.read_word(address).map(u64::from),
            DWORD_BYTES => emu.maps.read_dword(address).map(u64::from),
            QWORD_BYTES => emu.maps.read_qword(address),
            _ => None,
        };
        if let Some(value) = value {
            return Ok(value);
        }
        // The scalar fast path can reject values spanning several adjacent maps.
        // Check every byte without padding gaps or bypassing read permissions.
        let mut bytes = [0; QWORD_BYTES];
        for (offset, byte) in bytes[..size].iter_mut().enumerate() {
            let address = address + offset as u64;
            *byte = emu
                .maps
                .read_byte(address)
                .ok_or_else(|| self.error(index, size, Some(address), ArgumentFault::Unreadable))?;
        }
        Ok(u64::from_le_bytes(bytes))
    }
}

pub(crate) fn set_integer_return(emu: &mut Emu, value: u64) {
    if emu.cfg.arch.is_aarch64() {
        emu.regs_aarch64_mut().x[0] = value;
    } else {
        emu.regs_mut().rax = value;
    }
}

pub(crate) fn set_u64_return(emu: &mut Emu, value: u64) {
    if emu.cfg.arch.is_64bits() {
        set_integer_return(emu, value);
    } else {
        emu.regs_mut().rax = value as u32 as u64;
        emu.regs_mut().rdx = value >> u32::BITS;
    }
}

pub(crate) fn set_f64_return(emu: &mut Emu, value: f64) {
    if emu.cfg.arch.is_aarch64() {
        emu.regs_aarch64_mut().v[0] = value.to_bits() as u128;
    } else if emu.cfg.arch.is_64bits() {
        emu.regs_mut().xmm0 = value.to_bits() as u128;
    } else {
        // C returns push x87 ST0; _CI* intrinsics instead replace their input.
        emu.fpu_mut().push_f64(value);
    }
}
