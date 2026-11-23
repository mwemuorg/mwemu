//! Reader for native snapshots written before per-range memory protection.

use std::cell::RefCell;
use std::collections::BTreeMap;

use ahash::AHashMap;
use serde::Deserialize;
use slab::Slab;

use crate::maps::Maps;
use crate::maps::mem64::{Mem64, Permission};
use crate::maps::tlb::TLB;

#[derive(Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(super) struct LegacySerializableMaps<M = LegacyMem64> {
    maps: LegacyMaps<M>,
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
struct LegacyMaps<M> {
    banzai: bool,
    mem_slab: Slab<M>,
    maps: BTreeMap<u64, usize>,
    name_map: AHashMap<String, usize>,
    is_64bits: bool,
    tlb: RefCell<TLB>,
    max_alloc_size: u64,
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(super) struct LegacyMem64 {
    mem_name: String,
    base_addr: u64,
    bottom_addr: u64,
    permission: Permission,
    mem: Vec<u8>,
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(super) struct RangeMem64 {
    mem_name: String,
    base_addr: u64,
    bottom_addr: u64,
    permission: Permission,
    permission_overrides: BTreeMap<u64, Permission>,
    mem: Vec<u8>,
}

impl From<LegacyMem64> for Mem64 {
    fn from(old: LegacyMem64) -> Self {
        Mem64::new(
            old.mem_name,
            old.base_addr,
            old.bottom_addr,
            old.mem,
            old.permission,
        )
    }
}

impl From<RangeMem64> for Mem64 {
    fn from(old: RangeMem64) -> Self {
        let mut mem = Mem64::new(
            old.mem_name,
            old.base_addr,
            old.bottom_addr,
            old.mem,
            old.permission,
        );
        let boundaries: Vec<_> = old.permission_overrides.into_iter().collect();
        for (index, &(offset, permission)) in boundaries.iter().enumerate() {
            let end = boundaries
                .get(index + 1)
                .map_or(old.bottom_addr, |&(next, _)| next);
            mem.set_permission_range(offset, end - offset, permission);
        }
        mem
    }
}

impl<M: Into<Mem64>> From<LegacySerializableMaps<M>> for Maps {
    fn from(value: LegacySerializableMaps<M>) -> Self {
        let old = value.maps;
        let mem_slab = old
            .mem_slab
            .into_iter()
            .map(|(id, mem)| (id, mem.into()))
            .collect();
        let mut maps = Maps::new(
            mem_slab,
            old.maps,
            old.name_map,
            old.is_64bits,
            old.banzai,
            old.tlb,
        );
        maps.max_alloc_size = old.max_alloc_size;
        maps
    }
}

#[cfg(test)]
mod tests {
    use crate::emu32;
    use crate::maps::mem64::Permission;
    use crate::serialization::NATIVE_V2_MAGIC;
    use crate::serialization::Serialization;
    use crate::serialization::emu::SerializableEmu;
    use crate::serialization::legacy_maps::{LegacySerializableMaps, RangeMem64};

    #[test]
    fn reads_native_snapshots_with_the_previous_memory_layout() {
        let mut emu = emu32();
        emu.regs_mut().rax = 0x1234;
        emu.maps
            .create_map("removed", 0x400000, 0x1000, Permission::READ_WRITE)
            .unwrap();
        emu.maps
            .create_map("kept", 0x500000, 0x1000, Permission::READ_WRITE)
            .unwrap();
        assert!(emu.maps.write_dword(0x500100, 0x12345678));
        emu.maps.dealloc(0x400000);
        let key = emu.maps.maps[&0x500000];
        let kept = emu.maps.get_mem_mut("kept");
        assert!(kept.set_permission_range(0x500800, 0x800, Permission::READ));

        // JSON projects the current map state onto each historical wire layout.
        let state = serde_json::to_value(SerializableEmu::from(&emu)).unwrap();
        let v1: SerializableEmu<LegacySerializableMaps> =
            serde_json::from_value(state.clone()).unwrap();
        let v2: SerializableEmu<LegacySerializableMaps<RangeMem64>> =
            serde_json::from_value(state).unwrap();
        let mut tagged = NATIVE_V2_MAGIC.to_vec();
        tagged.extend(bitcode::serialize(&v2).unwrap());

        for (bytes, upper_permission) in [
            (bitcode::serialize(&v1).unwrap(), Permission::READ_WRITE),
            (tagged, Permission::READ),
        ] {
            let restored = Serialization::try_deserialize(&bytes).unwrap();
            assert_eq!(restored.regs().rax, 0x1234);
            assert_eq!(restored.maps.maps[&0x500000], key);
            let kept = restored.maps.get_mem("kept");
            assert_eq!(kept.get_base(), 0x500000);
            assert_eq!(kept.permission_at(0x5007ff), Permission::READ_WRITE);
            assert_eq!(kept.permission_at(0x500800), upper_permission);
            assert_eq!(restored.maps.read_dword(0x500100), Some(0x12345678));
            assert!(!restored.maps.is_mapped(0x400000));
        }
    }
}
