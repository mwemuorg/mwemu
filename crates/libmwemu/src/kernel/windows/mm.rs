//! Memory manager and HAL: physical-memory windows, MDL mapping, system
//! routine lookup, PCI configuration space.
//!
//! `MmMapIoSpace`, `MmCopyMemory`, `ZwMapViewOfSection(\Device\PhysicalMemory)`
//! and `HalGetBusDataByOffset` are the primitives a BYOVD exploit is built on,
//! so every one of them is recorded as a privileged operation with the
//! physical address the driver asked for. The window itself is a tracked
//! chunk: it is readable and writable so the driver keeps running, and a
//! stale use after `MmUnmapIoSpace` reports like any other use-after-free.

use super::structs::*;
use super::*;
use crate::kernel::heap::Region;

/// Model a physical-memory window: a pool-backed chunk that stands in for
/// `[phys, phys + size)`. Reads see zero unless the driver wrote first.
fn map_physical(emu: &mut Emu, phys: u64, size: u64, api: &str) -> u64 {
    let va = emu.kernel_alloc(Region::Pages, size.max(1), "physical-window", api, true);
    if va != 0 {
        emu.kernel_privop("map_io_space", phys, va, size);
        if let Some(k) = emu.kernel.as_mut() {
            k.win.io_mappings.insert(va, (phys, size));
        }
        log::info!("{}: phys 0x{:x} size 0x{:x} -> 0x{:x}", api, phys, size, va);
    }
    va
}

fn unmap_physical(emu: &mut Emu, va: u64, api: &str) {
    let phys = emu
        .kernel
        .as_mut()
        .and_then(|k| k.win.io_mappings.remove(&va))
        .map(|(p, _)| p)
        .unwrap_or(0);
    emu.kernel_privop("unmap_io_space", phys, va, 0);
    emu.kernel_free(va, api);
}

/// PCI configuration space lives in the generic register file, keyed so a
/// harness can preset a vendor/device id: `pci_key(bus, slot, offset)`.
pub fn pci_key(bus: u64, slot: u64, offset: u64) -> u64 {
    0x5043_0000_0000_0000 | ((bus & 0xff) << 24) | ((slot & 0xffff) << 8) | (offset & 0xff)
}

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    match symbol {
        // MmMapIoSpace(PhysicalAddress, NumberOfBytes, CacheType)
        "MmMapIoSpace" | "MmMapIoSpaceEx" => {
            let phys = emu.kernel_arg(0);
            let size = emu.kernel_arg(1);
            let va = map_physical(emu, phys, size, symbol);
            emu.set_kernel_ret(va);
        }
        "MmUnmapIoSpace" => {
            let va = emu.kernel_arg(0);
            unmap_physical(emu, va, symbol);
            emu.set_kernel_ret(0);
        }
        // MmMapVideoDisplay(PhysicalAddress, NumberOfBytes, CacheType)
        "MmMapVideoDisplay" => {
            let phys = emu.kernel_arg(0);
            let size = emu.kernel_arg(1);
            let va = map_physical(emu, phys, size, symbol);
            emu.set_kernel_ret(va);
        }
        "MmUnmapVideoDisplay" => {
            let va = emu.kernel_arg(0);
            unmap_physical(emu, va, symbol);
            emu.set_kernel_ret(0);
        }
        // MmCopyMemory(TargetAddress, SourceAddress (MM_COPY_ADDRESS by value),
        //              NumberOfBytes, Flags, *NumberOfBytesTransferred)
        "MmCopyMemory" => {
            let dst = emu.kernel_arg(0);
            let src = emu.kernel_arg(1);
            let len = emu.kernel_arg(2);
            let flags = emu.kernel_arg(3);
            let out = emu.kernel_arg(4);
            const MM_COPY_MEMORY_PHYSICAL: u64 = 1;
            if flags & MM_COPY_MEMORY_PHYSICAL != 0 {
                emu.kernel_privop("phys_read", src, dst, len);
                // Physical memory reads as zero.
                let n = crt::mapped_extent(emu, dst, len);
                emu.maps.write_bytes(dst, &vec![0u8; n as usize]);
            } else if emu.maps.is_mapped(src) {
                emu.maps.memcpy(dst, src, len as usize);
            } else {
                write_out_qword(emu, out, 0);
                emu.set_kernel_ret(STATUS_ACCESS_VIOLATION);
                return true;
            }
            write_out_qword(emu, out, len);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        // MmCopyVirtualMemory(FromProcess, FromAddress, ToProcess, ToAddress,
        //                     BufferSize, PreviousMode, *NumberOfBytesCopied)
        "MmCopyVirtualMemory" => {
            let src = emu.kernel_arg(1);
            let dst = emu.kernel_arg(3);
            let len = emu.kernel_arg(4);
            let out = emu.kernel_arg(6);
            emu.kernel_privop("process_copy", src, dst, len);
            if emu.maps.is_mapped(src) && emu.maps.is_mapped(dst) {
                emu.maps.memcpy(dst, src, len as usize);
                write_out_qword(emu, out, len);
                emu.set_kernel_ret(STATUS_SUCCESS);
            } else {
                write_out_qword(emu, out, 0);
                emu.set_kernel_ret(STATUS_ACCESS_VIOLATION);
            }
        }
        // MmGetPhysicalAddress(BaseAddress): a PHYSICAL_ADDRESS by value.
        // The emulated "physical" address of a window is the address the
        // driver asked for; anything else is identity-mapped into a low range.
        "MmGetPhysicalAddress" => {
            let va = emu.kernel_arg(0);
            let phys = emu
                .kernel
                .as_ref()
                .and_then(|k| {
                    k.win
                        .io_mappings
                        .iter()
                        .find(|(base, (_, size))| va >= **base && va < **base + *size)
                        .map(|(base, (p, _))| p + (va - base))
                })
                .unwrap_or(va & 0x0000_000f_ffff_ffff);
            emu.kernel_privop("virt_to_phys", va, phys, 0);
            emu.set_kernel_ret(phys);
        }
        "MmGetVirtualForPhysical" => {
            let phys = emu.kernel_arg(0);
            let va = emu
                .kernel
                .as_ref()
                .and_then(|k| {
                    k.win
                        .io_mappings
                        .iter()
                        .find(|(_, (p, size))| phys >= *p && phys < *p + *size)
                        .map(|(base, (p, _))| base + (phys - p))
                })
                .unwrap_or(phys);
            emu.kernel_privop("phys_to_virt", phys, va, 0);
            emu.set_kernel_ret(va);
        }
        "MmGetPhysicalMemoryRanges" => {
            // One range: [0, 4GB), terminated by a zero entry.
            let p = emu.kernel_alloc(Region::Slab, 0x20, "PHYSICAL_MEMORY_RANGE", symbol, true);
            if p != 0 {
                emu.maps.write_qword(p + 8, 0x1_0000_0000);
            }
            emu.set_kernel_ret(p);
        }
        "MmIsAddressValid" => {
            let va = emu.kernel_arg(0);
            let ok = emu.maps.is_mapped(va);
            emu.set_kernel_ret(ok as u64);
        }
        "MmIsNonPagedSystemAddressValid"
        | "MmIsThisAnNtAsSystem"
        | "MmIsDriverVerifying"
        | "MmIsDriverVerifyingByAddress"
        | "MmIsIoSpaceActive" => {
            emu.set_kernel_ret(0);
        }
        // MmGetSystemRoutineAddress(SystemRoutineName): resolve by name to an
        // interceptable stub, so a driver that resolves `ZwQuerySystemInformation`
        // at run time is intercepted exactly like one that imports it.
        "MmGetSystemRoutineAddress" => {
            let name = read_unicode_string(emu, emu.kernel_arg(0));
            let addr = if name.is_empty() {
                0
            } else {
                emu.kernel_resolve_import(&name).unwrap_or(0)
            };
            log::info!("MmGetSystemRoutineAddress: '{}' -> 0x{:x}", name, addr);
            emu.kernel_log_line(format!("MmGetSystemRoutineAddress({})", name));
            emu.set_kernel_ret(addr);
        }
        "MmGetSystemRoutineAddressEx" => {
            let name = read_unicode_string(emu, emu.kernel_arg(1));
            let addr = if name.is_empty() {
                0
            } else {
                emu.kernel_resolve_import(&name).unwrap_or(0)
            };
            emu.set_kernel_ret(addr);
        }

        // --- MDL ----------------------------------------------------------------
        // MmProbeAndLockPages(Mdl, AccessMode, Operation): the pages are
        // already "locked" by construction.
        "MmProbeAndLockPages" | "MmProbeAndLockProcessPages" | "MmProbeAndLockSelectedPages" => {
            let mdl = emu.kernel_arg(0);
            let flags = emu.maps.read_word(mdl + MDL_FLAGS).unwrap_or(0);
            emu.maps
                .write_word(mdl + MDL_FLAGS, flags | MDL_PAGES_LOCKED);
            emu.set_kernel_ret(0);
        }
        "MmUnlockPages" => {
            let mdl = emu.kernel_arg(0);
            let flags = emu.maps.read_word(mdl + MDL_FLAGS).unwrap_or(0);
            emu.maps
                .write_word(mdl + MDL_FLAGS, flags & !MDL_PAGES_LOCKED);
            emu.set_kernel_ret(0);
        }
        // MmMapLockedPagesSpecifyCache(Mdl, AccessMode, CacheType, RequestedAddress,
        //                              BugCheckOnFailure, Priority)
        "MmMapLockedPagesSpecifyCache"
        | "MmMapLockedPages"
        | "MmMapLockedPagesWithReservedMapping" => {
            let mdl = emu.kernel_arg(0);
            let start = emu.maps.read_qword(mdl + MDL_START_VA).unwrap_or(0);
            let off = emu.maps.read_dword(mdl + MDL_BYTE_OFFSET).unwrap_or(0) as u64;
            let va = start + off;
            let flags = emu.maps.read_word(mdl + MDL_FLAGS).unwrap_or(0);
            emu.maps
                .write_word(mdl + MDL_FLAGS, flags | MDL_MAPPED_TO_SYSTEM_VA);
            emu.maps.write_qword(mdl + MDL_MAPPED_SYSTEM_VA, va);
            // Mapping an MDL built over physical pages (MmAllocatePagesForMdl)
            // or over an MmMapIoSpace window is still the physical primitive.
            if start >= MM_SYSTEM_RANGE_START {
                emu.kernel_privop(
                    "map_locked_pages",
                    start,
                    va,
                    emu.maps.read_dword(mdl + MDL_BYTE_COUNT).unwrap_or(0) as u64,
                );
            }
            emu.set_kernel_ret(va);
        }
        "MmUnmapLockedPages" | "MmUnmapReservedMapping" => {
            emu.set_kernel_ret(0);
        }
        "MmGetSystemAddressForMdlSafe" | "MmGetSystemAddressForMdl" => {
            let mdl = emu.kernel_arg(0);
            let va = emu.maps.read_qword(mdl + MDL_MAPPED_SYSTEM_VA).unwrap_or(0);
            emu.set_kernel_ret(va);
        }
        // MmBuildMdlForNonPagedPool(Mdl): fills MappedSystemVa and the PFNs.
        "MmBuildMdlForNonPagedPool" => {
            let mdl = emu.kernel_arg(0);
            let start = emu.maps.read_qword(mdl + MDL_START_VA).unwrap_or(0);
            let off = emu.maps.read_dword(mdl + MDL_BYTE_OFFSET).unwrap_or(0) as u64;
            let count = emu.maps.read_dword(mdl + MDL_BYTE_COUNT).unwrap_or(0) as u64;
            io::build_mdl(emu, mdl, start + off, count);
            emu.set_kernel_ret(0);
        }
        // MmCreateMdl(Mdl, Base, Length) — legacy allocate-or-init.
        "MmCreateMdl" => {
            let mdl = emu.kernel_arg(0);
            let va = emu.kernel_arg(1);
            let len = emu.kernel_arg(2);
            let out = if mdl == 0 {
                io::alloc_mdl(emu, va, len, symbol)
            } else {
                io::build_mdl(emu, mdl, va, len);
                mdl
            };
            emu.set_kernel_ret(out);
        }
        "MmSizeOfMdl" => {
            let va = emu.kernel_arg(0);
            let len = emu.kernel_arg(1);
            let pages = ((va + len.max(1) - 1) >> 12) - (va >> 12) + 1;
            emu.set_kernel_ret(MDL_SIZE + pages * 8);
        }
        "MmMdlPageContentsState" => emu.set_kernel_ret(0),
        // MmAllocatePagesForMdl(LowAddress, HighAddress, SkipBytes, TotalBytes)
        "MmAllocatePagesForMdl"
        | "MmAllocatePagesForMdlEx"
        | "MmAllocateNodePagesForMdlEx"
        | "MmAllocatePartitionNodePagesForMdlEx" => {
            let total = emu.kernel_arg(3);
            let pages = emu.kernel_alloc(Region::Pages, total.max(1), "mdl-pages", symbol, true);
            let mdl = if pages != 0 {
                io::alloc_mdl(emu, pages, total, symbol)
            } else {
                0
            };
            // Freshly allocated pages are not mapped until MmMapLockedPages.
            if mdl != 0 {
                let flags = emu.maps.read_word(mdl + MDL_FLAGS).unwrap_or(0);
                emu.maps
                    .write_word(mdl + MDL_FLAGS, flags & !MDL_MAPPED_TO_SYSTEM_VA);
                emu.maps.write_qword(mdl + MDL_MAPPED_SYSTEM_VA, 0);
            }
            emu.set_kernel_ret(mdl);
        }
        "MmFreePagesFromMdl" | "MmFreePagesFromMdlEx" => {
            let mdl = emu.kernel_arg(0);
            let start = emu.maps.read_qword(mdl + MDL_START_VA).unwrap_or(0);
            if start != 0 {
                emu.kernel_free(start, symbol);
            }
            emu.set_kernel_ret(0);
        }
        "MmMapMemoryDumpMdl" | "MmMapMemoryDumpMdlEx" => emu.set_kernel_ret(0),
        // MmMapViewInSystemSpace(Section, *MappedBase, *ViewSize)
        "MmMapViewInSystemSpace" | "MmMapViewInSystemSpaceEx" => {
            let base_out = emu.kernel_arg(1);
            let size_ptr = emu.kernel_arg(2);
            let size = emu.maps.read_qword(size_ptr).unwrap_or(0x1000).max(0x1000);
            let va = emu.kernel_alloc(Region::Pages, size, "section-view", symbol, true);
            write_out_qword(emu, base_out, va);
            emu.maps.write_qword(size_ptr, size);
            emu.set_kernel_ret(if va != 0 {
                STATUS_SUCCESS
            } else {
                STATUS_NO_MEMORY
            });
        }
        "MmUnmapViewInSystemSpace" => {
            let va = emu.kernel_arg(0);
            emu.kernel_free(va, symbol);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "MmMapViewOfSection" => emu.set_kernel_ret(STATUS_NOT_SUPPORTED),
        "MmSecureVirtualMemory" | "MmSecureVirtualMemoryEx" => emu.set_kernel_ret(1),
        "MmUnsecureVirtualMemory"
        | "MmLockPagableSectionByHandle"
        | "MmUnlockPagableImageSection"
        | "MmPageEntireDriver"
        | "MmResetDriverPaging"
        | "MmSetAddressRangeModified"
        | "MmFlushImageSection"
        | "MmAdvanceMdl"
        | "MmPrefetchPages"
        | "MmProtectMdlSystemAddress"
        | "MmAddVerifierThunks"
        | "MmSetPermanentCacheAttribute"
        | "MmFlushIoBuffers" => {
            emu.set_kernel_ret(0);
        }
        "MmLockPagableDataSection" | "MmLockPagableCodeSection" | "MmLockPagableImageSection" => {
            let p = emu.kernel_arg(0);
            emu.set_kernel_ret(p);
        }
        "MmQuerySystemSize" => emu.set_kernel_ret(2), // MmLargeSystem
        "MmGetMdlPfnArray" => {
            let mdl = emu.kernel_arg(0);
            emu.set_kernel_ret(mdl + MDL_SIZE);
        }
        // ProbeForRead(Address, Length, Alignment) / ProbeForWrite: a user
        // address past MmUserProbeAddress raises STATUS_ACCESS_VIOLATION on
        // a real kernel, which the driver catches with __try. There is no SEH
        // dispatch in kernel mode here, so the violation ends the run loudly.
        "ProbeForRead" | "ProbeForWrite" => {
            let addr = emu.kernel_arg(0);
            let len = emu.kernel_arg(1);
            let align = emu.kernel_arg(2).max(1);
            if len != 0
                && (addr >= MM_USER_PROBE_ADDRESS
                    || addr.wrapping_add(len) < addr
                    || !addr.is_multiple_of(align))
            {
                log::error!(
                    "{}: 0x{:x} ({} bytes) is not a valid user address — STATUS_ACCESS_VIOLATION would be raised",
                    symbol,
                    addr,
                    len
                );
                emu.kernel_log_line(format!("{}: access violation on 0x{:x}", symbol, addr));
                emu.stop();
            }
            emu.set_kernel_ret(0);
        }

        // --- HAL -----------------------------------------------------------------
        // HalGetBusDataByOffset(BusDataType, BusNumber, SlotNumber, Buffer, Offset, Length)
        "HalGetBusDataByOffset" | "HalGetBusData" => {
            // ULONG arguments on the stack carry stale upper halves: truncate.
            let bus = emu.kernel_arg(1) as u32 as u64;
            let slot = emu.kernel_arg(2) as u32 as u64;
            let buf = emu.kernel_arg(3);
            let (off, len) = if symbol == "HalGetBusData" {
                (0, emu.kernel_arg(4) as u32 as u64)
            } else {
                (
                    emu.kernel_arg(4) as u32 as u64,
                    emu.kernel_arg(5) as u32 as u64,
                )
            };
            // Configuration space is 4 KB per function; a longer request is
            // the driver passing the caller's length straight through.
            let n = crt::mapped_extent(emu, buf, len.min(0x1000));
            for i in 0..n {
                let v = emu.kernel_get_register(pci_key(bus, slot, off + i)) as u8;
                emu.maps.write_byte(buf + i, v);
            }
            emu.kernel_privop("pci_read", pci_key(bus, slot, off) & 0xffff_ffff, buf, len);
            emu.set_kernel_ret(len);
        }
        "HalSetBusDataByOffset" | "HalSetBusData" => {
            let bus = emu.kernel_arg(1) as u32 as u64;
            let slot = emu.kernel_arg(2) as u32 as u64;
            let buf = emu.kernel_arg(3);
            let (off, len) = if symbol == "HalSetBusData" {
                (0, emu.kernel_arg(4) as u32 as u64)
            } else {
                (
                    emu.kernel_arg(4) as u32 as u64,
                    emu.kernel_arg(5) as u32 as u64,
                )
            };
            let mut first = 0u64;
            let n = crt::mapped_extent(emu, buf, len.min(0x1000));
            for i in 0..n {
                let v = emu.maps.read_byte(buf + i).unwrap_or(0) as u64;
                if i < 8 {
                    first |= v << (8 * i);
                }
                emu.kernel_set_register(pci_key(bus, slot, off + i), v);
            }
            emu.kernel_privop(
                "pci_write",
                pci_key(bus, slot, off) & 0xffff_ffff,
                first,
                len,
            );
            emu.set_kernel_ret(len);
        }
        // HalTranslateBusAddress(InterfaceType, BusNumber, BusAddress, *AddressSpace, *TranslatedAddress)
        "HalTranslateBusAddress" => {
            let bus_addr = emu.kernel_arg(2);
            let translated = emu.kernel_arg(4);
            write_out_qword(emu, translated, bus_addr);
            emu.set_kernel_ret(1);
        }
        "HalGetInterruptVector" => emu.set_kernel_ret(0x30),
        "HalGetAdapter"
        | "HalQuerySystemInformation"
        | "HalSetSystemInformation"
        | "HalAssignSlotResources"
        | "HalAdjustResourceList"
        | "HalQueryRealTimeClock"
        | "HalSetRealTimeClock"
        | "HalMakeBeep"
        | "HalRequestIpi"
        | "HalSendNMI"
        | "HalSendSoftwareInterrupt"
        | "HalEnumerateEnvironmentVariablesEx"
        | "HalGetEnvironmentVariable"
        | "HalSetEnvironmentVariable"
        | "HalAcquireDisplayOwnership"
        | "HalDisplayString"
        | "HalProcessorIdle"
        | "HalReadDmaCounter"
        | "HalAllocateHardwareCounters"
        | "HalFreeHardwareCounters"
        | "HalBugCheckSystem"
        | "KeFlushWriteBuffer"
        | "KeStallExecutionProcessor"
        | "HalGetBusDataByOffsetEx" => {
            emu.set_kernel_ret(
                if symbol.starts_with("HalQuery")
                    || symbol.starts_with("HalSetSystem")
                    || symbol == "HalGetAdapter"
                {
                    STATUS_NOT_IMPLEMENTED
                } else {
                    0
                },
            );
        }
        "HalReturnToFirmware" => {
            log::error!("HalReturnToFirmware: the driver asked to reboot");
            emu.kernel_log_line("HalReturnToFirmware".to_string());
            emu.stop();
        }
        _ => return false,
    }
    true
}

pub const SURFACE: &[&str] = &[
    "MmMapIoSpace",
    "MmMapIoSpaceEx",
    "MmUnmapIoSpace",
    "MmMapVideoDisplay",
    "MmUnmapVideoDisplay",
    "MmCopyMemory",
    "MmCopyVirtualMemory",
    "MmGetPhysicalAddress",
    "MmGetVirtualForPhysical",
    "MmGetPhysicalMemoryRanges",
    "MmIsAddressValid",
    "MmIsNonPagedSystemAddressValid",
    "MmIsThisAnNtAsSystem",
    "MmIsDriverVerifying",
    "MmGetSystemRoutineAddress",
    "MmGetSystemRoutineAddressEx",
    "MmProbeAndLockPages",
    "MmProbeAndLockProcessPages",
    "MmUnlockPages",
    "MmMapLockedPagesSpecifyCache",
    "MmMapLockedPages",
    "MmMapLockedPagesWithReservedMapping",
    "MmUnmapLockedPages",
    "MmUnmapReservedMapping",
    "MmGetSystemAddressForMdlSafe",
    "MmGetSystemAddressForMdl",
    "MmBuildMdlForNonPagedPool",
    "MmCreateMdl",
    "MmSizeOfMdl",
    "MmAllocatePagesForMdl",
    "MmAllocatePagesForMdlEx",
    "MmAllocateNodePagesForMdlEx",
    "MmFreePagesFromMdl",
    "MmFreePagesFromMdlEx",
    "MmMapViewInSystemSpace",
    "MmMapViewInSystemSpaceEx",
    "MmUnmapViewInSystemSpace",
    "MmSecureVirtualMemory",
    "MmUnsecureVirtualMemory",
    "MmLockPagableDataSection",
    "MmLockPagableCodeSection",
    "MmLockPagableSectionByHandle",
    "MmUnlockPagableImageSection",
    "MmPageEntireDriver",
    "MmResetDriverPaging",
    "MmQuerySystemSize",
    "MmGetMdlPfnArray",
    "MmAdvanceMdl",
    "MmProtectMdlSystemAddress",
    "ProbeForRead",
    "ProbeForWrite",
    "HalGetBusDataByOffset",
    "HalGetBusData",
    "HalSetBusDataByOffset",
    "HalSetBusData",
    "HalTranslateBusAddress",
    "HalGetInterruptVector",
    "HalGetAdapter",
    "HalQuerySystemInformation",
    "HalSetSystemInformation",
    "HalAssignSlotResources",
    "HalQueryRealTimeClock",
    "HalMakeBeep",
    "HalSendNMI",
    "HalReturnToFirmware",
    "KeStallExecutionProcessor",
    "KeFlushWriteBuffer",
];
