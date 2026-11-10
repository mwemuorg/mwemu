//! Allocation ownership, zeroing and transactional reallocation for Windows heaps.
use crate::emu::Emu;
use crate::emu::object_handle::HeapHandle;
use crate::exception::types::ExceptionType;
use crate::kernel::heap::Region;
use crate::maps::mem64::Permission;
use crate::winapi::common::memory::Call;
use crate::winapi::common::trace;
use crate::windows::constants;

pub(crate) fn alloc_context(emu: &Emu, handle: u64) -> Option<(usize, u64)> {
    let context = if handle == 0 {
        (0, 0)
    } else {
        emu.handle_management.heap_alloc_context(handle)?
    };
    if context.0 != 0 && !emu.maps.is_mapped(emu.heap_arenas.get(context.0)?.base()) {
        return None;
    }
    Some(context)
}

fn guarded_size(emu: &Emu, arena: usize, address: u64) -> Option<usize> {
    let chunk = emu.kernel.as_ref()?.heap.chunk_at(address)?;
    (chunk.addr == address && !chunk.is_freed() && chunk.heap_owner.unwrap_or(0) == arena)
        .then_some(chunk.req_size as usize)
}

pub(crate) fn owned_size(emu: &Emu, handle: u64, address: u64) -> Option<usize> {
    let (arena, _) = alloc_context(emu, handle)?;
    if emu.cfg.memory_guard {
        return guarded_size(emu, arena, address);
    }
    let heap = emu.heap_arenas.get(arena)?;
    if !emu.maps.is_mapped(heap.base()) {
        return None;
    }
    let size = heap.requested_size(address)?;
    emu.maps
        .can_access_range(address, size, Permission::NONE)
        .then_some(size)
}

pub(crate) fn heap_allocation_size(emu: &Emu, address: u64) -> Option<usize> {
    if emu.cfg.memory_guard {
        return guarded_size(emu, 0, address);
    }
    emu.heap_arenas
        .iter()
        .filter(|h| emu.maps.is_mapped(h.base()))
        .find_map(|h| h.requested_size(address))
}

pub(crate) fn heap_allocate(emu: &mut Emu, handle: u64, size: u64) -> Option<u64> {
    allocate(emu, handle, size, 0)
}

pub(crate) fn allocate(emu: &mut Emu, handle: u64, size: u64, flags: u64) -> Option<u64> {
    let (arena, maximum) = alloc_context(emu, handle)?;
    let size = usize::try_from(size.max(1)).ok()?;
    if maximum != 0 && size as u64 > maximum {
        return None;
    }
    if size as u64 > emu.maps.max_alloc_size {
        return None;
    }
    if emu.cfg.memory_guard {
        let address = emu.kernel_alloc(
            Region::Slab,
            size as u64,
            "heap",
            "HeapAlloc",
            flags & constants::HEAP_ZERO_MEMORY != 0,
        );
        if address == 0 {
            return None;
        }
        emu.kernel.as_mut()?.heap.set_heap_owner(address, arena);
        return Some(address);
    }
    let padded = size.max(usize::try_from(emu.cfg.heap_alloc_min_size).ok()?);
    if padded as u64 > emu.maps.max_alloc_size {
        return None;
    }
    let address = emu.heap_arena_mut(arena)?.allocate(padded)?;
    emu.heap_arena_mut(arena)?.record_request(address, size);
    let valid = emu
        .maps
        .can_access_range(address, size, Permission::READ_WRITE);
    if !valid
        || (flags & constants::HEAP_ZERO_MEMORY != 0 && !emu.maps.fill_range(address, 0, size))
    {
        emu.heap_arena_mut(arena)?.free(address);
        return None;
    }
    Some(address)
}

pub(crate) fn heap_free(emu: &mut Emu, handle: u64, address: u64) -> bool {
    let Some((arena, _)) = alloc_context(emu, handle) else {
        return false;
    };
    if emu.cfg.memory_guard {
        let valid = guarded_size(emu, arena, address).is_some();
        // Retain diagnostics for invalid/double frees without releasing another heap's block.
        let other_owner = emu
            .kernel
            .as_ref()
            .and_then(|k| k.heap.chunk_at(address))
            .is_some_and(|c| c.heap_owner.unwrap_or(0) != arena);
        if !other_owner {
            emu.kernel_free(address, "HeapFree");
        }
        return valid;
    }
    let Some(heap) = emu.heap_arena_mut(arena) else {
        return false;
    };
    if !heap.check_fragment_exists(address) {
        return false;
    }
    heap.free(address);
    true
}

pub(crate) fn heap_reallocate(emu: &mut Emu, handle: u64, address: u64, size: u64) -> Option<u64> {
    reallocate(emu, handle, address, size, 0)
}

pub(crate) fn reallocate(
    emu: &mut Emu,
    handle: u64,
    address: u64,
    size: u64,
    flags: u64,
) -> Option<u64> {
    let (arena, maximum) = alloc_context(emu, handle)?;
    let size = usize::try_from(size.max(1)).ok()?;
    if maximum != 0 && size as u64 > maximum {
        return None;
    }
    if size as u64 > emu.maps.max_alloc_size {
        return None;
    }
    let old_size = owned_size(emu, handle, address)?;
    let copied = old_size.min(size);
    if emu.cfg.memory_guard {
        if flags & constants::HEAP_REALLOC_IN_PLACE_ONLY != 0 {
            return None;
        }
        let new_address = allocate(emu, handle, size as u64, flags)?;
        if !emu.maps.copy_range(new_address, address, copied) {
            heap_free(emu, handle, new_address);
            return None;
        }
        heap_free(emu, handle, address);
        return Some(new_address);
    }
    // Plan using independent metadata. Failure leaves the live arena and bytes intact.
    let mut candidate = emu.heap_arena_mut(arena)?.clone();
    let padded = size.max(usize::try_from(emu.cfg.heap_alloc_min_size).ok()?);
    if padded as u64 > emu.maps.max_alloc_size {
        return None;
    }
    let result = candidate.reallocate(address, padded)?;
    candidate.record_request(result.new_addr, size);
    if flags & constants::HEAP_REALLOC_IN_PLACE_ONLY != 0 && result.new_addr != address {
        return None;
    }
    if !emu
        .maps
        .can_access_range(result.new_addr, size, Permission::READ_WRITE)
    {
        return None;
    }
    if result.new_addr != address && !emu.maps.copy_range(result.new_addr, address, copied) {
        return None;
    }
    if flags & constants::HEAP_ZERO_MEMORY != 0 && size > old_size {
        // The destination was validated before copying; zero only the newly requested tail.
        assert!(
            emu.maps
                .fill_range(result.new_addr + old_size as u64, 0, size - old_size)
        );
    }
    *emu.heap_arena_mut(arena)? = candidate;
    Some(result.new_addr)
}

pub(crate) fn fail_allocation(emu: &mut Emu, flags: u64) {
    if flags & constants::HEAP_GENERATE_EXCEPTIONS != 0 {
        emu.exception(ExceptionType::HeapNoMemory);
    }
}

fn api_handle(emu: &Emu, handle: u64) -> bool {
    handle != 0 && emu.handle_management.heap_alloc_context(handle).is_some()
}

pub(crate) fn alloc_api(emu: &mut Emu) {
    let call = Call::Api;
    let handle = call.arg(emu, 0);
    let flags = call.arg(emu, 1);
    let size = call.arg(emu, 2);
    let value = if api_handle(emu, handle) {
        allocate(emu, handle, size, flags).unwrap_or(0)
    } else {
        0
    };
    if value == 0 {
        fail_allocation(emu, flags);
    }
    call.finish(emu, value, 3);
}

pub(crate) fn free_api(emu: &mut Emu) {
    let call = Call::Api;
    let handle = call.arg(emu, 0);
    let address = call.arg(emu, 2);
    let success = api_handle(emu, handle) && heap_free(emu, handle, address);
    call.finish(emu, u64::from(success), 3);
}

pub(crate) fn realloc_api(emu: &mut Emu) {
    let call = Call::Api;
    let handle = call.arg(emu, 0);
    let flags = call.arg(emu, 1);
    let address = call.arg(emu, 2);
    let size = call.arg(emu, 3);
    let value = if api_handle(emu, handle) {
        reallocate(emu, handle, address, size, flags).unwrap_or(0)
    } else {
        0
    };
    if value == 0 {
        fail_allocation(emu, flags);
    }
    call.finish(emu, value, 4);
}

pub(crate) fn size_api(emu: &mut Emu) {
    let call = Call::Api;
    let handle = call.arg(emu, 0);
    let address = call.arg(emu, 2);
    let size = if api_handle(emu, handle) {
        owned_size(emu, handle, address).map(|s| s as u64)
    } else {
        None
    };
    call.finish(emu, size.unwrap_or(u64::MAX), 3);
}

pub(crate) fn create_api(emu: &mut Emu) {
    let call = Call::Api;
    let options = call.arg(emu, 0) as u32;
    let initial = call.arg(emu, 1);
    let maximum = call.arg(emu, 2);
    let value = (|| {
        let initial = usize::try_from(initial).ok()?;
        let maximum = usize::try_from(maximum).ok()?;
        if maximum != 0 && initial > maximum {
            return None;
        }
        let arena = emu.create_heap_arena(initial, maximum, options)?;
        let base = emu.heap_arenas[arena].base();
        let key = emu
            .handle_management
            .insert_heap_handle(HeapHandle::new(options, initial, maximum, arena));
        emu.handle_management.set_heap_base_addr(key, base);
        Some(base)
    })()
    .unwrap_or(0);
    call.finish(emu, value, 3);
}

pub(crate) fn destroy_api(emu: &mut Emu, native: bool) {
    let call = Call::Api;
    let handle = call.arg(emu, 0);
    let success = emu
        .handle_management
        .resolve_heap_key(handle)
        .is_some_and(|key| {
            if emu.handle_management.is_process_heap(key) {
                return false;
            }
            let Some(heap) = emu.handle_management.remove_heap_handle(key) else {
                return false;
            };
            emu.destroy_heap_arena(heap.arena);
            true
        });
    let value = if native {
        if success { 0 } else { handle }
    } else {
        u64::from(success)
    };
    call.finish(emu, value, 1);
}

pub(crate) fn local_size_api(emu: &mut Emu) {
    let call = Call::Api;
    let address = call.arg(emu, 0);
    let size = owned_size(emu, 0, address);
    if size.is_none() {
        emu.set_last_error(constants::ERROR_INVALID_HANDLE);
    }
    call.finish(emu, size.unwrap_or(0) as u64, 1);
}

pub(crate) fn local_alloc_api(emu: &mut Emu) {
    let call = Call::Api;
    let flags = call.arg(emu, 0);
    let size = call.arg(emu, 1);
    let flags = if flags & 0x40 != 0 {
        constants::HEAP_ZERO_MEMORY
    } else {
        0
    };
    let value = allocate(emu, 0, size, flags).unwrap_or(0);
    call.finish(emu, value, 2);
}

pub(crate) fn local_free_api(emu: &mut Emu) {
    let call = Call::Api;
    let address = call.arg(emu, 0);
    let value = if address == 0 || heap_free(emu, 0, address) {
        0
    } else {
        address
    };
    call.finish(emu, value, 1);
}

pub(crate) fn local_realloc_api(emu: &mut Emu) {
    let call = Call::Api;
    let address = call.arg(emu, 0);
    let size = call.arg(emu, 1);
    let flags = call.arg(emu, 2);
    let flags = if flags & 0x40 != 0 {
        constants::HEAP_ZERO_MEMORY
    } else {
        0
    };
    let value = reallocate(emu, 0, address, size, flags).unwrap_or(0);
    call.finish(emu, value, 3);
}

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    let (handler, parameters, returns_value): (fn(&mut Emu), &str, bool) = match api {
        "HeapAlloc" | "RtlAllocateHeap" => (alloc_api, "heap flags size", true),
        "HeapFree" | "RtlFreeHeap" => (free_api, "heap flags address", true),
        "HeapReAlloc" | "RtlReAllocateHeap" => (realloc_api, "heap flags address size", true),
        "HeapSize" | "RtlSizeHeap" => (size_api, "heap flags address", true),
        "HeapCreate" => (create_api, "options initial maximum", true),
        "HeapDestroy" => (|emu| destroy_api(emu, false), "heap", true),
        "RtlDestroyHeap" => (|emu| destroy_api(emu, true), "heap", true),
        "GlobalSize" | "LocalSize" => (local_size_api, "address", true),
        "GlobalAlloc" | "LocalAlloc" => (local_alloc_api, "flags size", true),
        "GlobalFree" | "LocalFree" => (local_free_api, "address", true),
        "GlobalReAlloc" | "LocalReAlloc" => (local_realloc_api, "address size flags", true),
        _ => return false,
    };
    trace::api(emu, api, parameters, returns_value, handler);
    true
}
