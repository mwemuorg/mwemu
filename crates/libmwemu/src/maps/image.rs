//! Atomic creation of sparse PE images and their complete address-space owner.

use crate::maps::Maps;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::{Allocation, MemoryKind, PageState, protection_for};

pub(crate) struct ImageRegion<'a> {
    pub name: String,
    pub offset: u64,
    pub size: u64,
    pub permission: Permission,
    pub data: &'a [u8],
}

impl Maps {
    /// No recursive loader work occurs here: publish ownership only after every
    /// backing region succeeds, before the caller binds imports or runs guest code.
    pub(crate) fn map_image(
        &mut self,
        base: u64,
        size: u64,
        regions: &[ImageRegion<'_>],
    ) -> Result<(), String> {
        if !self.is_range_free(base, size) {
            return Err(format!("image range unavailable: 0x{base:x} + 0x{size:x}"));
        }
        let mut allocation = Allocation::new(
            base,
            size,
            protection_for(Permission::READ_WRITE),
            MemoryKind::Image,
            PageState::reserved(),
        );
        for region in regions {
            if region.size == 0 {
                continue;
            }
            if region
                .offset
                .checked_add(region.size)
                .is_none_or(|end| end > size)
                || region.data.len() as u64 > region.size
            {
                self.unmap_range(base, size);
                return Err(format!("section {} exceeds image range", region.name));
            }
            let address = base + region.offset;
            let map = match self.create_map(&region.name, address, region.size, region.permission) {
                Ok(map) => map,
                Err(error) => {
                    self.unmap_range(base, size);
                    return Err(error);
                }
            };
            map.memcpy(region.data, region.data.len());
            allocation.set_state(
                address,
                region.size,
                PageState::committed(protection_for(region.permission), region.permission),
            );
        }
        self.register_allocation(allocation);
        Ok(())
    }
}
