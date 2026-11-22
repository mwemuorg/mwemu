use std::ops::Range;

use goblin::pe::characteristic::{IMAGE_FILE_LARGE_ADDRESS_AWARE, IMAGE_FILE_RELOCS_STRIPPED};
use goblin::pe::dll_characteristic::{
    IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE, IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA,
};

use super::PeLoadError;
use crate::emu::Emu;
use crate::maps::virtual_memory::ALLOCATION_GRANULARITY;
use crate::windows::constants::errors::{ERROR_INVALID_ADDRESS, ERROR_NOT_ENOUGH_MEMORY};
use crate::windows::constants::memory::*;

// Keep low-entropy image bases within a 16 MiB band of the preferred address when space permits.
const LOW_ENTROPY_SPAN: u64 = 256 * ALLOCATION_GRANULARITY;
const SMALL_ADDRESS_LIMIT: u64 = 1 << 31;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ImageRole {
    Main,
    GuestDll,
    SystemLibrary,
}

impl ImageRole {
    pub(super) fn from_main_image(main: bool) -> Self {
        if main {
            Self::Main
        } else {
            Self::SystemLibrary
        }
    }
}

pub(super) struct GuestImageBase {
    pub preferred: u64,
    pub coff_flags: u16,
    pub dll_flags: u16,
    pub relocatable: bool,
    pub is_64: bool,
}

impl GuestImageBase {
    pub(super) fn choose(
        &self,
        emu: &mut Emu,
        name: &str,
        span: u64,
        role: ImageRole,
    ) -> Result<u64, PeLoadError> {
        let large_address_aware = self.coff_flags & IMAGE_FILE_LARGE_ADDRESS_AWARE != 0;
        let width_limit = if self.is_64 {
            USER64_ADDRESS_LIMIT
        } else {
            USER32_ADDRESS_LIMIT
        };
        // Main-image code executes below the API interception range. Guest DLLs are tracked by allocation.
        let limit = if role == ImageRole::Main {
            if self.is_64 { LIBS64_MIN } else { LIBS32_MIN }
        } else {
            width_limit
        };
        let limit = if role == ImageRole::Main && !large_address_aware {
            limit.min(SMALL_ADDRESS_LIMIT)
        } else {
            limit
        };
        let aslr = self.relocatable && self.dll_flags & IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE != 0;
        if !aslr && emu.pe_base_available(self.preferred, span, limit) {
            return Ok(self.preferred);
        }
        // An unspecified base is used by synthetic images that need no fixups.
        if !self.relocatable
            && (self.preferred != 0 || self.coff_flags & IMAGE_FILE_RELOCS_STRIPPED != 0)
        {
            return Err(PeLoadError::new(
                ERROR_INVALID_ADDRESS,
                format!(
                    "cannot map fixed PE image '{name}' at {:#x}",
                    self.preferred
                ),
            ));
        }
        let wide_addresses = self.is_64 && large_address_aware;
        let address_space = if wide_addresses {
            ALLOC64_MIN..ALLOC64_MAX
        } else {
            ALLOC32_MIN..ALLOC32_MAX
        };
        let found = if aslr {
            let high_entropy =
                wide_addresses && self.dll_flags & IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA != 0;
            emu.random_image_base(span, self.preferred, address_space, high_entropy, limit)
        } else {
            emu.alloc_pe_image(span, address_space.start, address_space.end)
        };
        found.ok_or_else(|| {
            PeLoadError::new(
                ERROR_NOT_ENOUGH_MEMORY,
                format!("cannot allocate PE image '{name}' ({span:#x} bytes)"),
            )
        })
    }
}

impl Emu {
    fn random_image_base(
        &mut self,
        span: u64,
        preferred: u64,
        fallback: Range<u64>,
        high_entropy: bool,
        limit: u64,
    ) -> Option<u64> {
        if span == 0 || span > self.maps.max_alloc_size {
            return None;
        }
        let (first, last) = if high_entropy {
            (fallback.start, fallback.end.checked_sub(span)?)
        } else {
            let max_base = limit.checked_sub(span)?;
            let anchor =
                (preferred.min(max_base) & !(LOW_ENTROPY_SPAN - 1)).max(ALLOCATION_GRANULARITY);
            (
                anchor,
                (anchor + LOW_ENTROPY_SPAN - ALLOCATION_GRANULARITY).min(max_base),
            )
        };
        let slots = last.checked_sub(first)? / ALLOCATION_GRANULARITY + 1;
        self.apply_cfg_seed();
        let start = first + (self.random_u64() % slots) * ALLOCATION_GRANULARITY;
        let end = last + span;
        let alternative = |bottom, top| {
            let base = self.alloc_pe_image(span, bottom, top)?;
            if base == preferred {
                self.alloc_pe_image(span, base.checked_add(ALLOCATION_GRANULARITY)?, top)
            } else {
                Some(base)
            }
        };
        alternative(start, end)
            .or_else(|| alternative(first, end))
            .or_else(|| alternative(fallback.start, fallback.end))
            .or_else(|| {
                self.pe_base_available(preferred, span, limit)
                    .then_some(preferred)
            })
    }
}
