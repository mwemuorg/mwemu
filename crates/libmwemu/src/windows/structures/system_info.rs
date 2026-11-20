use crate::maps::Maps;
use crate::maps::virtual_memory::{ALLOCATION_GRANULARITY, PAGE_SIZE};
use crate::windows::constants::memory::{USER32_ADDRESS_LIMIT, USER64_ADDRESS_LIMIT};

const PROCESSOR_ARCHITECTURE_INTEL: u16 = 0;
const PROCESSOR_ARCHITECTURE_AMD64: u16 = 9;
const PROCESSOR_INTEL_PENTIUM: u32 = 586;
const PROCESSOR_AMD_X8664: u32 = 8664;

// Match the guest environment's NUMBER_OF_PROCESSORS.
pub(crate) const PROCESSORS: u32 = 4;
pub(crate) const ACTIVE_PROCESSOR_MASK: u32 = (1 << PROCESSORS) - 1;
const PROCESSOR_LEVEL: u16 = 6;

const SYSTEM_INFO_SIZE_32: usize = 36;
const SYSTEM_INFO_SIZE_64: usize = 48;

fn system_info(wide_pointers: bool, native_amd64: bool) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(if wide_pointers {
        SYSTEM_INFO_SIZE_64
    } else {
        SYSTEM_INFO_SIZE_32
    });
    let (architecture, processor_type) = if native_amd64 {
        (PROCESSOR_ARCHITECTURE_AMD64, PROCESSOR_AMD_X8664)
    } else {
        (PROCESSOR_ARCHITECTURE_INTEL, PROCESSOR_INTEL_PENTIUM)
    };
    bytes.extend(architecture.to_le_bytes());
    bytes.extend(0u16.to_le_bytes());
    bytes.extend((PAGE_SIZE as u32).to_le_bytes());
    let limit = if wide_pointers {
        USER64_ADDRESS_LIMIT
    } else {
        USER32_ADDRESS_LIMIT
    };
    for value in [
        ALLOCATION_GRANULARITY,
        limit - ALLOCATION_GRANULARITY - 1,
        ACTIVE_PROCESSOR_MASK.into(),
    ] {
        if wide_pointers {
            bytes.extend(value.to_le_bytes());
        } else {
            bytes.extend((value as u32).to_le_bytes());
        }
    }
    bytes.extend(PROCESSORS.to_le_bytes());
    bytes.extend(processor_type.to_le_bytes());
    bytes.extend((ALLOCATION_GRANULARITY as u32).to_le_bytes());
    bytes.extend(PROCESSOR_LEVEL.to_le_bytes());
    bytes.extend(0u16.to_le_bytes());
    bytes
}

pub(crate) fn save_system_info(
    maps: &mut Maps,
    address: u64,
    wide_pointers: bool,
    native_amd64: bool,
) -> bool {
    let bytes = system_info(wide_pointers, native_amd64);
    maps.validate_write_range(address, bytes.len() as u64) && maps.write_bytes(address, &bytes)
}
