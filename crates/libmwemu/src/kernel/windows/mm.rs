//! Memory manager and HAL: physical-memory windows, MDL mapping, system
//! routine lookup, PCI configuration space.
//!
//! `MmMapIoSpace`, `MmCopyMemory`, `ZwMapViewOfSection(\Device\PhysicalMemory)`
//! and `HalGetBusDataByOffset` are the primitives a BYOVD exploit is built on,
//! so every one of them is recorded as a privileged operation with the
//! physical address the driver asked for. The window itself is a tracked
//! chunk: it is readable and writable so the driver keeps running, and a
//! stale use after `MmUnmapIoSpace` reports like any other use-after-free.

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::api::windows::policy::api_try;
use crate::kernel::{HandlerRegistry, kernel_handlers, return_one, return_zero};

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

/// `MmGetSystemRoutineAddress` semantics: a name the kernel exports resolves
/// to an interceptable stub (or data storage); anything else is NULL, so a
/// driver probing for an optional API takes its fallback path exactly as it
/// would on a kernel without that export. Only the modelled surface counts
/// as exported — an unmodelled real export would be a silent stub anyway.
fn resolve_system_routine(emu: &mut Emu, name: &str) -> u64 {
    if name.is_empty() {
        return 0;
    }
    if is_implemented(name) || data_symbol_size(name).is_some() {
        return emu.kernel_resolve_import(name).unwrap_or(0);
    }
    log::info!(
        "MmGetSystemRoutineAddress: '{}' is not in the modelled surface, returning NULL",
        name
    );
    0
}

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    dispatch_api!(symbol, emu, registry handlers(),
        handled(()) => true,
        fallback => false,
    )
}

pub const fn handlers() -> HandlerRegistry {
    kernel_handlers! {
        // MmMapIoSpace(PhysicalAddress, NumberOfBytes, CacheType)
        "MmMapIoSpace" | "MmMapIoSpaceEx" => MmMapIoSpace,
        "MmUnmapIoSpace" => MmUnmapIoSpace,
        // MmMapVideoDisplay(PhysicalAddress, NumberOfBytes, CacheType)
        "MmMapVideoDisplay" => MmMapIoSpace,
        "MmUnmapVideoDisplay" => MmUnmapIoSpace,
        // MmCopyMemory(TargetAddress, SourceAddress (MM_COPY_ADDRESS by value),
        //              NumberOfBytes, Flags, *NumberOfBytesTransferred)
        "MmCopyMemory" => MmCopyMemory,
        // MmCopyVirtualMemory(FromProcess, FromAddress, ToProcess, ToAddress,
        //                     BufferSize, PreviousMode, *NumberOfBytesCopied)
        "MmCopyVirtualMemory" => MmCopyVirtualMemory,
        // MmGetPhysicalAddress(BaseAddress): a PHYSICAL_ADDRESS by value.
        // The emulated "physical" address of a window is the address the
        // driver asked for; anything else is identity-mapped into a low range.
        "MmGetPhysicalAddress" => MmGetPhysicalAddress,
        "MmGetVirtualForPhysical" => MmGetVirtualForPhysical,
        "MmGetPhysicalMemoryRanges" => MmGetPhysicalMemoryRanges,
        "MmIsAddressValid" => MmIsAddressValid,
        "MmIsNonPagedSystemAddressValid"
        | "MmIsThisAnNtAsSystem"
        | "MmIsDriverVerifying"
        | "MmIsDriverVerifyingByAddress"
        | "MmIsIoSpaceActive" => return_zero,
        // MmGetSystemRoutineAddress(SystemRoutineName): resolve by name to an
        // interceptable stub, so a driver that resolves `ZwQuerySystemInformation`
        // at run time is intercepted exactly like one that imports it.
        "MmGetSystemRoutineAddress" => MmGetSystemRoutineAddress,
        "MmGetSystemRoutineAddressEx" => MmGetSystemRoutineAddressEx,

        // --- MDL ----------------------------------------------------------------
        // MmProbeAndLockPages(Mdl, AccessMode, Operation): the pages are
        // already "locked" by construction.
        "MmProbeAndLockPages" | "MmProbeAndLockProcessPages" | "MmProbeAndLockSelectedPages" => MmProbeAndLockPages,
        "MmUnlockPages" => MmUnlockPages,
        // MmMapLockedPagesSpecifyCache(Mdl, AccessMode, CacheType, RequestedAddress,
        //                              BugCheckOnFailure, Priority)
        "MmMapLockedPagesSpecifyCache"
        | "MmMapLockedPages"
        | "MmMapLockedPagesWithReservedMapping" => MmMapLockedPagesSpecifyCache,
        "MmUnmapLockedPages" | "MmUnmapReservedMapping" => return_zero,
        "MmGetSystemAddressForMdlSafe" | "MmGetSystemAddressForMdl" => MmGetSystemAddressForMdlSafe,
        // MmBuildMdlForNonPagedPool(Mdl): fills MappedSystemVa and the PFNs.
        "MmBuildMdlForNonPagedPool" => MmBuildMdlForNonPagedPool,
        // MmCreateMdl(Mdl, Base, Length) — legacy allocate-or-init.
        "MmCreateMdl" => MmCreateMdl,
        "MmSizeOfMdl" => MmSizeOfMdl,
        "MmMdlPageContentsState" => return_zero,
        // MmAllocatePagesForMdl(LowAddress, HighAddress, SkipBytes, TotalBytes)
        "MmAllocatePagesForMdl"
        | "MmAllocatePagesForMdlEx"
        | "MmAllocateNodePagesForMdlEx"
        | "MmAllocatePartitionNodePagesForMdlEx" => MmAllocatePagesForMdl,
        "MmFreePagesFromMdl" | "MmFreePagesFromMdlEx" => MmFreePagesFromMdl,
        "MmMapMemoryDumpMdl" | "MmMapMemoryDumpMdlEx" => return_zero,
        // MmMapViewInSystemSpace(Section, *MappedBase, *ViewSize)
        "MmMapViewInSystemSpace" | "MmMapViewInSystemSpaceEx" => MmMapViewInSystemSpace,
        "MmUnmapViewInSystemSpace" => MmUnmapViewInSystemSpace,
        "MmMapViewOfSection" => MmMapViewOfSection,
        "MmSecureVirtualMemory" | "MmSecureVirtualMemoryEx" => return_one,
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
        | "MmFlushIoBuffers" => return_zero,
        "MmLockPagableDataSection" | "MmLockPagableCodeSection" | "MmLockPagableImageSection" => MmLockPagableDataSection,
        "MmQuerySystemSize" => MmQuerySystemSize, // MmLargeSystem
        "MmGetMdlPfnArray" => MmGetMdlPfnArray,
        // ProbeForRead(Address, Length, Alignment) / ProbeForWrite: a user
        // address past MmUserProbeAddress raises STATUS_ACCESS_VIOLATION on
        // a real kernel, which the driver catches with __try. There is no SEH
        // dispatch in kernel mode here, so the violation ends the run loudly.
        "ProbeForRead" | "ProbeForWrite" => ProbeForRead,

        // --- HAL -----------------------------------------------------------------
        // HalGetBusDataByOffset(BusDataType, BusNumber, SlotNumber, Buffer, Offset, Length)
        "HalGetBusDataByOffset" | "HalGetBusData" => HalGetBusDataByOffset,
        "HalSetBusDataByOffset" | "HalSetBusData" => HalSetBusDataByOffset,
        // HalTranslateBusAddress(InterfaceType, BusNumber, BusAddress, *AddressSpace, *TranslatedAddress)
        "HalTranslateBusAddress" => HalTranslateBusAddress,
        "HalGetInterruptVector" => HalGetInterruptVector,
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
        | "HalGetBusDataByOffsetEx" => hal_stub,
        "HalReturnToFirmware" => HalReturnToFirmware,
    }
}

pub const SURFACE: &[&str] = handlers().names;

pub fn has_handler(symbol: &str) -> bool {
    handlers().find(symbol).is_some()
}

fn MmMapIoSpace(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let phys = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg(emu, 1));
    let va = map_physical(emu, phys, size, symbol);
    call.set_return(emu, va);
}

fn MmUnmapIoSpace(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let va = api_try!(emu, call.arg(emu, 0));
    unmap_physical(emu, va, symbol);
    call.set_return(emu, 0);
}

fn MmCopyMemory(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let len = api_try!(emu, call.arg(emu, 2));
    let flags = api_try!(emu, call.arg(emu, 3));
    let out = api_try!(emu, call.arg(emu, 4));
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
        call.set_return(emu, STATUS_ACCESS_VIOLATION);
        return;
    }
    write_out_qword(emu, out, len);
    call.set_return(emu, STATUS_SUCCESS);
}

fn MmCopyVirtualMemory(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let src = api_try!(emu, call.arg(emu, 1));
    let dst = api_try!(emu, call.arg(emu, 3));
    let len = api_try!(emu, call.arg(emu, 4));
    let out = api_try!(emu, call.arg(emu, 6));
    emu.kernel_privop("process_copy", src, dst, len);
    if emu.maps.is_mapped(src) && emu.maps.is_mapped(dst) {
        emu.maps.memcpy(dst, src, len as usize);
        write_out_qword(emu, out, len);
        call.set_return(emu, STATUS_SUCCESS);
    } else {
        write_out_qword(emu, out, 0);
        call.set_return(emu, STATUS_ACCESS_VIOLATION);
    }
}

fn MmGetPhysicalAddress(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let va = api_try!(emu, call.arg(emu, 0));
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
    call.set_return(emu, phys);
}

fn MmGetVirtualForPhysical(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let phys = api_try!(emu, call.arg(emu, 0));
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
    call.set_return(emu, va);
}

fn MmGetPhysicalMemoryRanges(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    // One range: [0, 4GB), terminated by a zero entry.
    let p = emu.kernel_alloc(Region::Slab, 0x20, "PHYSICAL_MEMORY_RANGE", symbol, true);
    if p != 0 {
        emu.maps.write_qword(p + 8, 0x1_0000_0000);
    }
    call.set_return(emu, p);
}

fn MmIsAddressValid(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let va = api_try!(emu, call.arg(emu, 0));
    let ok = emu.maps.is_mapped(va);
    call.set_return(emu, ok as u64);
}

fn MmGetSystemRoutineAddress(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let name = read_unicode_string(emu, api_try!(emu, call.arg(emu, 0)));
    let addr = resolve_system_routine(emu, &name);
    log::info!("MmGetSystemRoutineAddress: '{}' -> 0x{:x}", name, addr);
    emu.kernel_log_line(format!("MmGetSystemRoutineAddress({})", name));
    call.set_return(emu, addr);
}

fn MmGetSystemRoutineAddressEx(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let name = read_unicode_string(emu, api_try!(emu, call.arg(emu, 1)));
    let addr = resolve_system_routine(emu, &name);
    call.set_return(emu, addr);
}

fn MmProbeAndLockPages(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let mdl = api_try!(emu, call.arg(emu, 0));
    let flags = emu.maps.read_word(mdl + MDL_FLAGS).unwrap_or(0);
    emu.maps
        .write_word(mdl + MDL_FLAGS, flags | MDL_PAGES_LOCKED);
    call.set_return(emu, 0);
}

fn MmUnlockPages(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let mdl = api_try!(emu, call.arg(emu, 0));
    let flags = emu.maps.read_word(mdl + MDL_FLAGS).unwrap_or(0);
    emu.maps
        .write_word(mdl + MDL_FLAGS, flags & !MDL_PAGES_LOCKED);
    call.set_return(emu, 0);
}

fn MmMapLockedPagesSpecifyCache(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let mdl = api_try!(emu, call.arg(emu, 0));
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
    call.set_return(emu, va);
}

fn MmGetSystemAddressForMdlSafe(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let mdl = api_try!(emu, call.arg(emu, 0));
    let va = emu.maps.read_qword(mdl + MDL_MAPPED_SYSTEM_VA).unwrap_or(0);
    call.set_return(emu, va);
}

fn MmBuildMdlForNonPagedPool(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let mdl = api_try!(emu, call.arg(emu, 0));
    let start = emu.maps.read_qword(mdl + MDL_START_VA).unwrap_or(0);
    let off = emu.maps.read_dword(mdl + MDL_BYTE_OFFSET).unwrap_or(0) as u64;
    let count = emu.maps.read_dword(mdl + MDL_BYTE_COUNT).unwrap_or(0) as u64;
    io::build_mdl(emu, mdl, start + off, count);
    call.set_return(emu, 0);
}

fn MmCreateMdl(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let mdl = api_try!(emu, call.arg(emu, 0));
    let va = api_try!(emu, call.arg(emu, 1));
    let len = api_try!(emu, call.arg(emu, 2));
    let out = if mdl == 0 {
        io::alloc_mdl(emu, va, len, symbol)
    } else {
        io::build_mdl(emu, mdl, va, len);
        mdl
    };
    call.set_return(emu, out);
}

fn MmSizeOfMdl(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let va = api_try!(emu, call.arg(emu, 0));
    let len = api_try!(emu, call.arg(emu, 1));
    let pages = ((va + len.max(1) - 1) >> 12) - (va >> 12) + 1;
    call.set_return(emu, MDL_SIZE + pages * 8);
}

fn MmAllocatePagesForMdl(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let total = api_try!(emu, call.arg(emu, 3));
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
    call.set_return(emu, mdl);
}

fn MmFreePagesFromMdl(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let mdl = api_try!(emu, call.arg(emu, 0));
    let start = emu.maps.read_qword(mdl + MDL_START_VA).unwrap_or(0);
    if start != 0 {
        emu.kernel_free(start, symbol);
    }
    call.set_return(emu, 0);
}

fn MmMapViewInSystemSpace(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let base_out = api_try!(emu, call.arg(emu, 1));
    let size_ptr = api_try!(emu, call.arg(emu, 2));
    let size = emu.maps.read_qword(size_ptr).unwrap_or(0x1000).max(0x1000);
    let va = emu.kernel_alloc(Region::Pages, size, "section-view", symbol, true);
    write_out_qword(emu, base_out, va);
    emu.maps.write_qword(size_ptr, size);
    call.set_return(
        emu,
        if va != 0 {
            STATUS_SUCCESS
        } else {
            STATUS_NO_MEMORY
        },
    );
}

fn MmUnmapViewInSystemSpace(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let va = api_try!(emu, call.arg(emu, 0));
    emu.kernel_free(va, symbol);
    call.set_return(emu, STATUS_SUCCESS);
}

fn MmMapViewOfSection(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(emu, STATUS_NOT_SUPPORTED);
}

fn MmLockPagableDataSection(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let p = api_try!(emu, call.arg(emu, 0));
    call.set_return(emu, p);
}

fn MmQuerySystemSize(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(emu, 2);
}

fn MmGetMdlPfnArray(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let mdl = api_try!(emu, call.arg(emu, 0));
    call.set_return(emu, mdl + MDL_SIZE);
}

fn ProbeForRead(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let addr = api_try!(emu, call.arg(emu, 0));
    let len = api_try!(emu, call.arg(emu, 1));
    let align = api_try!(emu, call.arg(emu, 2)).max(1);
    if len != 0
        && (addr >= MM_USER_PROBE_ADDRESS
            || addr.wrapping_add(len) < addr
            || !addr.is_multiple_of(align))
    {
        emu.kernel_abort(format!(
            "{}(0x{:x}, {}) raised STATUS_ACCESS_VIOLATION — no SEH dispatch in kernel mode",
            symbol, addr, len
        ));
    }
    call.set_return(emu, 0);
}

fn HalGetBusDataByOffset(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    // ULONG arguments on the stack carry stale upper halves: truncate.
    let bus = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let slot = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    let buf = api_try!(emu, call.arg(emu, 3));
    let (off, len) = if symbol == "HalGetBusData" {
        (0, api_try!(emu, call.arg_u32(emu, 4)) as u64)
    } else {
        (
            api_try!(emu, call.arg_u32(emu, 4)) as u64,
            api_try!(emu, call.arg_u32(emu, 5)) as u64,
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
    call.set_return(emu, len);
}

fn HalSetBusDataByOffset(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let bus = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let slot = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    let buf = api_try!(emu, call.arg(emu, 3));
    let (off, len) = if symbol == "HalSetBusData" {
        (0, api_try!(emu, call.arg_u32(emu, 4)) as u64)
    } else {
        (
            api_try!(emu, call.arg_u32(emu, 4)) as u64,
            api_try!(emu, call.arg_u32(emu, 5)) as u64,
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
    call.set_return(emu, len);
}

fn HalTranslateBusAddress(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let bus_addr = api_try!(emu, call.arg(emu, 2));
    let translated = api_try!(emu, call.arg(emu, 4));
    write_out_qword(emu, translated, bus_addr);
    call.set_return(emu, 1);
}

fn HalGetInterruptVector(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(emu, 0x30);
}

fn hal_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(
        emu,
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

fn HalReturnToFirmware(emu: &mut Emu, _symbol: &str) {
    emu.kernel_abort("HalReturnToFirmware: the driver asked to reboot".to_string());
}
