//! Page protection shared by kernel32, kernelbase, NT API and syscalls.
use crate::emu::Emu;
use crate::maps::Maps;
use crate::maps::virtual_memory::{MemoryError, page_range as checked_page_range};
use crate::winapi::common::memory::permissions;
use crate::windows::constants::errors::*;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ProtectError {
    InvalidParameter,
    InvalidProtection,
    Unmapped,
}

impl ProtectError {
    pub(crate) fn win32_error(&self) -> u64 {
        match self {
            Self::InvalidParameter | Self::InvalidProtection => ERROR_INVALID_PARAMETER,
            Self::Unmapped => ERROR_INVALID_ADDRESS,
        }
    }

    pub(crate) fn nt_status(&self) -> u64 {
        match self {
            Self::InvalidParameter => STATUS_INVALID_PARAMETER,
            Self::InvalidProtection => 0xc0000045,
            Self::Unmapped => STATUS_CONFLICTING_ADDRESSES,
        }
    }
}

fn memory_error(error: MemoryError) -> ProtectError {
    match error {
        MemoryError::InvalidParameter => ProtectError::InvalidParameter,
        _ => ProtectError::Unmapped,
    }
}

pub(crate) fn page_range(address: u64, size: u64) -> Result<(u64, u64), ProtectError> {
    let (base, length) = checked_page_range(address, size).map_err(memory_error)?;
    Ok((base, base + length))
}

pub(crate) fn validate(
    maps: &Maps,
    address: u64,
    size: u64,
    protection: u32,
) -> Result<u32, ProtectError> {
    let permission = permissions(protection).ok_or(ProtectError::InvalidProtection)?;
    let (base, end) = page_range(address, size)?;
    maps.validate_protection(base, end - base, permission)
        .map_err(memory_error)
}

pub(crate) fn protect(
    maps: &mut Maps,
    address: u64,
    size: u64,
    protection: u32,
) -> Result<u32, ProtectError> {
    let permission = permissions(protection).ok_or(ProtectError::InvalidProtection)?;
    let (base, end) = page_range(address, size)?;
    maps.protect_memory(base, end - base, protection, permission)
        .map_err(memory_error)
}

pub(crate) fn protect_winapi(
    emu: &mut Emu,
    address: u64,
    size: u64,
    protection: u32,
    old_pointer: u64,
) -> bool {
    if !emu.maps.validate_write_range(old_pointer, 4) {
        emu.set_last_error(ERROR_INVALID_PARAMETER);
        emu.regs_mut().rax = 0;
        return false;
    }
    match validate(&emu.maps, address, size, protection) {
        Ok(old) => {
            // All output/range checks precede mutation, including when OldProtect aliases the range.
            emu.maps.write_dword(old_pointer, old);
            protect(&mut emu.maps, address, size, protection).expect("validated protection range");
            emu.regs_mut().rax = 1;
            true
        }
        Err(error) => {
            emu.set_last_error(error.win32_error());
            emu.regs_mut().rax = 0;
            false
        }
    }
}
