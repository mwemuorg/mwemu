use std::fs;

use itertools::iproduct;

use crate::api::windows::common::memory::Call;
use crate::api::windows::common::{memory, sections};
use crate::emu::Emu;
use crate::maps::Maps;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::MemoryKind;
use crate::serialization::Serialization;
use crate::tests::helpers::{LOADER32, LOADER64, call_winapi32, call_winapi64};
use crate::windows::constants::{LIBS32_MIN, LIBS64_MIN};
use crate::{emu32, emu64};

const PAGE: u32 = 0x1000;
const GRANULARITY: u64 = 0x10000;

#[derive(Clone, Copy, Debug)]
enum Loader {
    Pe32,
    Pe64,
    Dll64,
}

const LOADERS: [Loader; 3] = [Loader::Pe32, Loader::Pe64, Loader::Dll64];

impl Loader {
    fn is_64(self) -> bool {
        !matches!(self, Self::Pe32)
    }

    fn emu(self) -> Emu {
        let mut emu = if self.is_64() { emu64() } else { emu32() };
        emu.cfg.console_enabled = false;
        emu.cfg.winver = None;
        emu
    }

    fn load(self, emu: &mut Emu, raw: &[u8], name: &str) -> u64 {
        let base = self.try_load(emu, raw, name);
        assert_ne!(base, 0, "{self:?}");
        base
    }

    fn try_load(self, emu: &mut Emu, raw: &[u8], name: &str) -> u64 {
        let path = std::env::temp_dir().join(format!("mwemu_alignment_{name}.dll"));
        fs::write(&path, raw).unwrap();
        let filename = path.to_str().unwrap();
        let base = match self {
            Self::Pe32 => emu.load_pe32(filename, false, 0).0 as u64,
            Self::Pe64 => emu.load_pe64(filename, false, 0).0,
            Self::Dll64 => emu.map_dll_pe64(filename).0,
        };
        fs::remove_file(path).unwrap();
        base
    }
}

fn write_u32(raw: &mut [u8], offset: usize, value: u32) {
    raw[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn image(
    loader: Loader,
    alignment: u32,
    rva: u32,
    pointer: u32,
    raw_size: u32,
    virtual_size: u32,
) -> Vec<u8> {
    let fixture = if loader.is_64() { LOADER64 } else { LOADER32 };
    let mut raw = vec![0; 0x3000];
    raw[..0x400].copy_from_slice(&fixture[..0x400]);
    let nt = u32::from_le_bytes(raw[0x3c..0x40].try_into().unwrap()) as usize;
    let opt = nt + 24;
    let optional_size = u16::from_le_bytes(raw[nt + 20..nt + 22].try_into().unwrap()) as usize;
    raw[nt + 6..nt + 8].copy_from_slice(&1u16.to_le_bytes());
    write_u32(&mut raw, opt + 16, rva);
    // An unavailable preferred base forces DLL allocation without relocations.
    write_u32(&mut raw, opt + if loader.is_64() { 24 } else { 28 }, 0);
    if loader.is_64() {
        write_u32(&mut raw, opt + 28, 0);
    }
    write_u32(&mut raw, opt + 32, alignment);
    write_u32(&mut raw, opt + 36, alignment.min(0x200));
    write_u32(&mut raw, opt + 56, 0x4000.max(rva + virtual_size));
    write_u32(&mut raw, opt + 60, 0x400);
    let directories = opt + if loader.is_64() { 112 } else { 96 };
    raw[directories..directories + 16 * 8].fill(0);
    let section = opt + optional_size;
    raw[section..0x400].fill(0);
    raw[section..section + 8].copy_from_slice(b".probe\0\0");
    write_u32(&mut raw, section + 8, virtual_size);
    write_u32(&mut raw, section + 12, rva);
    write_u32(&mut raw, section + 16, raw_size);
    write_u32(&mut raw, section + 20, pointer);
    write_u32(&mut raw, section + 36, 0xe0000020);
    for (i, byte) in raw.iter_mut().enumerate().skip(0x400) {
        *byte = ((i ^ (i >> 8)) % 251 + 1) as u8;
    }
    raw
}

#[test]
fn section_raw_offsets_follow_image_alignment() {
    for (loader, pointer, file_alignment) in
        iproduct!(LOADERS, [0x143, 0x801, 0x9ff], [0x200, 0x1000])
    {
        let mut raw = image(loader, PAGE, PAGE, pointer, 0x200, PAGE);
        let nt = u32::from_le_bytes(raw[0x3c..0x40].try_into().unwrap()) as usize;
        write_u32(&mut raw, nt + 24 + 36, file_alignment);
        let mut emu = loader.emu();
        let base = loader.load(&mut emu, &raw, "offsets");
        let mapped = emu.maps.get_mem_by_addr(base + PAGE as u64).unwrap();
        let offset = (pointer & !0x1ff) as usize;
        assert_eq!(
            &mapped.get_mem()[..32],
            &raw[offset..offset + 32],
            "{loader:?}, pointer={pointer:#x}, file alignment={file_alignment:#x}"
        );
    }
}

#[test]
fn section_initialized_extent_includes_sector_and_page_padding() {
    for (loader, (pointer, raw_size, expected)) in iproduct!(
        LOADERS,
        [
            (0x800, 0x201, 0x400),
            (0x801, 0x800, 0xa00),
            (0x9ff, 0x201, 0x400),
            (0x800, 0x1200, 0x1000)
        ]
    ) {
        let raw = image(loader, PAGE, PAGE, pointer, raw_size, 0x20);
        let mut emu = loader.emu();
        let base = loader.load(&mut emu, &raw, "extent");
        let mapped = emu.maps.get_mem_by_addr(base + PAGE as u64).unwrap();
        let offset = (pointer & !0x1ff) as usize;
        assert_eq!(mapped.size(), PAGE as usize, "{loader:?}");
        assert_eq!(
            &mapped.get_mem()[..expected],
            &raw[offset..offset + expected],
            "{loader:?}, pointer={pointer:#x}, size={raw_size:#x}"
        );
        assert!(mapped.get_mem()[expected..].iter().all(|&byte| byte == 0));
    }
}

#[test]
fn zero_raw_pointer_leaves_section_uninitialized() {
    for loader in LOADERS {
        let raw = image(loader, PAGE, PAGE, 0, 0x200, PAGE);
        let mut emu = loader.emu();
        let base = loader.load(&mut emu, &raw, "zero");
        let mapped = emu.maps.get_mem_by_addr(base + PAGE as u64).unwrap();
        assert!(mapped.get_mem().iter().all(|&byte| byte == 0), "{loader:?}");
    }
}

#[test]
fn low_alignment_preserves_exact_file_offsets() {
    for (loader, alignment) in iproduct!(LOADERS, [1, 0x10, 0x200]) {
        let raw = image(loader, alignment, 0x403, 0x403, 0x20, 0x20);
        let mut emu = loader.emu();
        let base = loader.load(&mut emu, &raw, "low");
        let mapped = emu.maps.get_mem_by_addr(base + 0x403).unwrap();
        assert_eq!(
            &mapped.get_mem()[..0x20],
            &raw[0x403..0x423],
            "{loader:?}, alignment={alignment:#x}"
        );
    }
}

#[test]
fn dll_allocation_uses_granularity_and_full_virtual_extent() {
    for loader in LOADERS {
        let mut emu = loader.emu();
        let bottom = if loader.is_64() {
            LIBS64_MIN
        } else {
            LIBS32_MIN
        };
        emu.maps
            .create_map("first", bottom, 0x1001, Permission::READ_WRITE)
            .unwrap();
        emu.maps
            .create_map("obstacle", bottom + 0x20000, 0x1001, Permission::READ_WRITE)
            .unwrap();
        // The file fits the first aligned gap, but the late section does not.
        let raw = image(loader, PAGE, 0x10000, 0x800, 0x200, PAGE);
        let base = loader.load(&mut emu, &raw, "placement");
        assert_eq!(base, bottom + 0x30000, "{loader:?}");
        let section = emu.maps.get_mem_by_addr(base + 0x10000).unwrap();
        assert_eq!(&section.get_mem()[..0x200], &raw[0x800..0xa00]);
    }
}

#[test]
fn consecutive_stub_images_use_allocation_granularity() {
    for is_64 in [false, true] {
        let mut emu = if is_64 { emu64() } else { emu32() };
        for name in ["first.dll", "second.dll", "third.dll"] {
            let base = if is_64 {
                emu.create_stub_pe64(name, &["Example"])
            } else {
                emu.create_stub_pe32(name, &["Example"]) as u64
            };
            assert_ne!(base, 0);
            assert_eq!(base % GRANULARITY, 0, "{name}, is_64={is_64}");
        }
    }
}

#[test]
fn pe_image_gaps_remain_owned_and_excluded_from_allocation() {
    for loader in LOADERS {
        let raw = image(loader, PAGE, 0x30000, 0x800, 0x200, PAGE);
        let mut emu = loader.emu();
        let base = loader.load(&mut emu, &raw, "owned");
        let gap = base + 0x10000;
        assert!(!emu.maps.is_mapped(gap));
        assert!(!emu.maps.is_range_free(gap, 1), "{loader:?}");
        assert!(
            emu.maps
                .create_map("intruder", gap, 0x1000, Permission::READ_WRITE)
                .is_err()
        );
        assert_eq!(
            emu.maps
                .find_free_range(0x1000, GRANULARITY, base, base + 0x50000),
            Some(base + 0x40000)
        );
        assert_eq!(
            emu.maps
                .find_free_range_top_down(0x1000, GRANULARITY, base, base + 0x31000),
            None
        );
        let info = emu.maps.query_memory(gap);
        assert_eq!(info.allocation_base, base);
        assert_eq!(info.kind, MemoryKind::Image);
        assert!(!info.state.unwrap().committed);
        assert_eq!(info.size, 0x20000);
        let next = if loader.is_64() {
            emu.create_stub_pe64("next.dll", &["Example"])
        } else {
            emu.create_stub_pe32("next.dll", &["Example"]) as u64
        };
        assert!(next >= base + 0x31000);
    }
}

#[test]
fn pe_image_load_failure_rolls_back_earlier_regions() {
    for loader in LOADERS {
        let raw = image(loader, PAGE, PAGE, 0x800, 0x200, PAGE);
        let mut emu = loader.emu();
        emu.maps
            .create_map(
                "mwemu_alignment_rollback.probe",
                0x100000,
                0x1000,
                Permission::READ_WRITE,
            )
            .unwrap();
        let count = emu.maps.maps.len();
        assert_eq!(loader.try_load(&mut emu, &raw, "rollback"), 0, "{loader:?}");
        assert_eq!(emu.maps.maps.len(), count);
        assert!(!emu.maps.exists_mapname("mwemu_alignment_rollback.pe"));
        assert_eq!(emu.maps.read_byte(0x100000), Some(0));
    }
}

#[test]
fn pe_image_stub_failure_rolls_back_earlier_regions() {
    for is64 in [false, true] {
        let mut emu = if is64 { emu64() } else { emu32() };
        emu.maps
            .create_map("broken.edata", 0x100000, 0x1000, Permission::READ_WRITE)
            .unwrap();
        let base = if is64 {
            emu.create_stub_pe64("broken.dll", &["Example"])
        } else {
            emu.create_stub_pe32("broken.dll", &["Example"]) as u64
        };
        assert_eq!(base, 0);
        assert_eq!(emu.maps.maps.len(), 1);
        assert!(emu.export_indexes.get_by_name("broken").is_none());
    }
}

fn sparse_image(loader: Loader) -> Vec<u8> {
    let mut raw = image(loader, PAGE, 0x30000, 0x800, 0x200, 4 * PAGE);
    let nt = u32::from_le_bytes(raw[0x3c..0x40].try_into().unwrap()) as usize;
    // Preserve a reserved tail beyond the final section as well as the middle gap.
    write_u32(&mut raw, nt + 24 + 56, 0x38000);
    raw
}

fn assert_image_owner(emu: &Emu, base: u64) {
    for offset in [0x1000, 0x10000, 0x35000] {
        let info = emu.maps.query_memory(base + offset);
        assert_eq!(info.allocation_base, base);
        assert_eq!(info.kind, MemoryKind::Image);
        assert!(!info.state.unwrap().committed);
        assert!(!emu.maps.is_range_free(base + offset, 1));
        assert_eq!(emu.maps.read_byte(base + offset), None);
    }
    assert_eq!(
        emu.maps.find_pe_image_info(base + 0x35000),
        Some((base, 0x38000))
    );
    assert_eq!(
        emu.maps
            .query_memory(base + 0x30000)
            .state
            .unwrap()
            .protection,
        0x40
    );
}

#[test]
fn pe_image_partial_unmap_keeps_owner_until_whole_view_is_released() {
    for loader in LOADERS {
        let mut emu = loader.emu();
        let base = loader.load(&mut emu, &sparse_image(loader), "lifetime");
        emu.maps.unmap_range(base + 0x31000, 0x1000);
        assert!(
            !emu.maps
                .query_memory(base + 0x31000)
                .state
                .unwrap()
                .committed
        );
        assert_eq!(emu.maps.read_byte(base + 0x32000), Some(0));
        assert!(!emu.maps.is_range_free(base + 0x31000, 0x1000));
        // Image identity must not depend on mutable guest headers.
        assert!(emu.maps.write_dword(base, 0));
        assert_eq!(
            emu.maps.find_pe_image_info(base + 0x35000),
            Some((base, 0x38000))
        );
        assert!(emu.maps.release_memory(base).is_err());
        sections::unmap(&mut emu, base + 0x10000).unwrap();
        assert!(emu.maps.is_range_free(base, 0x38000));
        assert!(emu.maps.find_pe_image_info(base).is_none());
        assert!(!emu.maps.is_mapped(base + 0x32000));
    }
}

#[test]
fn pe_image_query_and_allocation_apis_respect_reserved_holes() {
    for loader in LOADERS {
        let mut emu = loader.emu();
        let base = loader.load(&mut emu, &sparse_image(loader), "query");
        let gap = base + 0x10000;
        assert!(memory::allocate(&mut emu, gap, 0x1000, 0x3000, 4).is_err());
        assert!(
            emu.maps
                .protect_memory(gap, 0x1000, 4, Permission::READ_WRITE)
                .is_err()
        );
        let output = 0x5000000;
        emu.maps
            .create_map("query_output", output, 0x1000, Permission::READ_WRITE)
            .unwrap();
        let query: fn(&mut Emu) = |emu| memory::virtual_query(emu, false);
        let nt_query: fn(&mut Emu) = |emu| memory::nt_query(emu, Call::Api);
        let (length, state_offset) = if loader.is_64() { (48, 32) } else { (28, 16) };
        let result = if loader.is_64() {
            call_winapi64(&mut emu, query, &[gap, output, length])
        } else {
            call_winapi32(&mut emu, query, &[gap as u32, output as u32, length as u32]) as u64
        };
        assert_eq!(result, length);
        assert_eq!(
            memory::read_pointer(&emu, output + memory::pointer_size(&emu)),
            Some(base)
        );
        assert_eq!(emu.maps.read_dword(output + state_offset), Some(0x2000));
        assert_eq!(
            emu.maps.read_dword(output + state_offset + 8),
            Some(0x1000000)
        );
        let expected = emu.maps.read_bytes(output, length as usize).into_owned();
        emu.maps.fill_range(output, 0xcc, length as usize);
        let result = if loader.is_64() {
            call_winapi64(&mut emu, nt_query, &[u64::MAX, gap, 0, output, length, 0])
        } else {
            call_winapi32(
                &mut emu,
                nt_query,
                &[u32::MAX, gap as u32, 0, output as u32, length as u32, 0],
            ) as u64
        };
        assert_eq!(result, 0);
        assert_eq!(
            emu.maps.read_bytes(output, length as usize).as_ref(),
            expected
        );
    }
}

#[test]
fn pe_image_native_and_minidump_snapshots_keep_unbacked_gaps() {
    for loader in LOADERS {
        let mut emu = loader.emu();
        let base = loader.load(&mut emu, &sparse_image(loader), "snapshot");
        assert_image_owner(&emu, base);
        let native = Serialization::deserialize(&Serialization::serialize(&emu));
        assert_image_owner(&native, base);
        let directory = std::env::temp_dir().join(format!("mwemu_pe_owner_{loader:?}"));
        fs::create_dir_all(&directory).unwrap();
        let path = directory.join("image.mdmp");
        Serialization::dump_to_minidump(&emu, path.to_str().unwrap()).unwrap();
        assert!(
            fs::metadata(&path).unwrap().len() < 0x10000,
            "gaps must not create backing bytes"
        );
        let mut restored = Serialization::load_from_minidump(path.to_str().unwrap());
        assert_image_owner(&restored, base);
        assert_eq!(
            restored.maps.read_dword(base + 0x30000),
            emu.maps.read_dword(base + 0x30000)
        );
        sections::unmap(&mut restored, base + 0x35000).unwrap();
        assert!(restored.maps.is_range_free(base, 0x38000));
        for file in fs::read_dir(&directory).unwrap() {
            fs::remove_file(file.unwrap().path()).unwrap();
        }
        fs::remove_dir(directory).unwrap();
    }
}

#[test]
fn pe_image_unmap_removes_stub_exports_and_reuses_the_range() {
    for is64 in [false, true] {
        let mut emu = if is64 { emu64() } else { emu32() };
        let base = if is64 {
            emu.create_stub_pe64("unload.dll", &["Example"])
        } else {
            emu.create_stub_pe32("unload.dll", &["Example"]) as u64
        };
        assert!(emu.export_indexes.get_by_base(base).is_some());
        let unmap: fn(&mut Emu) = |emu| sections::nt_unmap(emu, Call::Api);
        let result = if is64 {
            call_winapi64(&mut emu, unmap, &[u64::MAX, base + 0x1001])
        } else {
            call_winapi32(&mut emu, unmap, &[u32::MAX, (base + 0x1001) as u32]) as u64
        };
        assert_eq!(result, 0);
        assert!(emu.export_indexes.get_by_base(base).is_none());
        assert!(emu.maps.is_range_free(base, 0x3000));
        let next = if is64 {
            emu.create_stub_pe64("replacement.dll", &["Example"])
        } else {
            emu.create_stub_pe32("replacement.dll", &["Example"]) as u64
        };
        assert_eq!(next, base);
    }
}

#[test]
fn pe_image_owner_and_backing_search_matches_exhaustive_candidates() {
    for owner in 1..20u64 {
        let mut maps = Maps::default();
        maps.map_image(owner, 3, &[]).unwrap();
        maps.create_map("backing", 24, 2, Permission::READ_WRITE)
            .unwrap();
        for (alignment, bottom, size) in iproduct!([1, 2, 4, 8, 16], 0..20, [1, 2, 3, 8]) {
            for top in bottom..32 {
                let candidates: Vec<_> = (bottom..top)
                    .filter(|&base| {
                        base % alignment == 0
                            && base + size <= top
                            && (base + size <= owner || base >= owner + 3)
                            && (base + size <= 24 || base >= 26)
                    })
                    .collect();
                assert_eq!(
                    maps.find_free_range(size, alignment, bottom, top),
                    candidates.first().copied()
                );
                assert_eq!(
                    maps.find_free_range_top_down(size, alignment, bottom, top),
                    candidates.last().copied()
                );
            }
        }
    }
}
