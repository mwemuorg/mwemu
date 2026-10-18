use crate::api::windows::common::ntdll;
use crate::emu;
use crate::emu::object_handle::SectionHandle;
use crate::maps::mem64::Permission;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
use crate::windows::constants;
use crate::windows::structures;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "NtAllocateVirtualMemory" => NtAllocateVirtualMemory(emu),
        "NtQueryVirtualMemory" => NtQueryVirtualMemory(emu),
        "RtlZeroMemory" => RtlZeroMemory(emu),
        "NtProtectVirtualMemory" => NtProtectVirtualMemory(emu),
        "memset" => memset(emu),
        "memcpy" => memcpy(emu),
        "CheckRemoteDebuggerPresent" => CheckRemoteDebuggerPresent(emu),
        "NtFreeVirtualMemory" => NtFreeVirtualMemory(emu),
        "NtCreateSection" => NtCreateSection(emu),
        "NtMapViewOfSection" => NtMapViewOfSection(emu),
        "NtUnmapViewOfSection" => NtUnmapViewOfSection(emu),
        "RtlMoveMemory" => RtlMoveMemory(emu),
        "RtlCopyMemory" => RtlCopyMemory(emu),
        "NtFlushInstructionCache" => NtFlushInstructionCache(emu),
        _ => return false,
    }
    true
}

fn RtlZeroMemory(emu: &mut emu::Emu) {
    let dest = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("bad RtlZeroMemory address pointer parameter") as u64;
    let length = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("bad RtlZeroMemory address length parameter") as u64;

    ntdll::rtl_zero_memory(emu, dest, length);
}

fn NtAllocateVirtualMemory(emu: &mut emu::Emu) {
    let addr_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("bad NtAllocateVirtualMemory address pointer parameter") as u64;
    let size_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("bad NtAllocateVirtualMemory size pointer parameter") as u64;
    let addr = emu
        .maps
        .read_dword(addr_ptr)
        .expect("bad NtAllocateVirtualMemory address parameter") as u64;
    let size = emu
        .maps
        .read_dword(size_ptr)
        .expect("bad NtAllocateVirtualMemory size parameter") as u64;

    let protection_offset = 20;
    let protection_addr = emu.regs().rsp + protection_offset;
    let protect_value = emu
        .maps
        .read_dword(protection_addr)
        .expect("Failed to read Protection argument at NtAllocateVirtualMemory");

    let (can_read, can_write, can_execute) = ntdll::protect_to_rwx(protect_value);

    let do_alloc: bool = if addr == 0 {
        true
    } else {
        emu.maps.is_mapped(addr)
    };

    if size == 0 {
        {
            log::warn!("NtAllocateVirtualMemory mapping zero bytes.");
            return;
        }
    }

    let alloc_addr: u64 = if do_alloc {
        match emu.maps.alloc(size) {
            Some(a) => a,
            None => {
                log::warn!(
                    "/!\\ out of memory   cannot allocate forntdll!NtAllocateVirtualMemory "
                );
                return;
            }
        }
    } else {
        addr
    };

    log_red!(
        emu,
        "ntdll!NtAllocateVirtualMemory  addr: 0x{:x} sz: {} alloc: 0x{:x}",
        addr,
        size,
        alloc_addr
    );

    emu.maps
        .create_map(
            format!("valloc_{:x}", alloc_addr).as_str(),
            alloc_addr,
            size,
            Permission::from_flags(can_read, can_write, can_execute),
        )
        .expect("ntdll!NtAllocateVirtualMemory cannot create map");

    if !emu.maps.write_dword(addr_ptr, alloc_addr as u32) {
        {
            log::warn!("NtAllocateVirtualMemory: cannot write on address pointer");
            return;
        }
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}

fn NtQueryVirtualMemory(emu: &mut emu::Emu) {
    let handle = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!NtQueryVirtualMemory: error reading handle") as u64;
    let addr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll!NtQueryVirtualMemory: error reading address") as u64;

    log_red!(emu, "ntdll!NtQueryVirtualMemory addr: 0x{:x}", addr);

    if handle != 0xffffffff {
        log::trace!("\tusing handle of remote process {:x}", handle);

        if !helper::handler_exist(handle) {
            log::trace!("\nhandler doesnt exist.");
        }
    }

    let out_meminfo_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("ntdll_NtQueryVirtualMemory: error reading out pointer to meminfo")
        as u64;

    if !emu.maps.is_mapped(addr) {
        log::trace!(
            "/!\\ ntdll!NtQueryVirtualMemory: querying non maped addr: 0x{:x}",
            addr
        );
        for _ in 0..6 {
            emu.stack_pop32(false);
        }
        emu.regs_mut().rax = constants::STATUS_INVALID_PARAMETER;
        return;
    }

    let base = emu.maps.get_addr_base(addr).unwrap_or(0);

    let mut mem_info = structures::MemoryBasicInformation::load(out_meminfo_ptr, &emu.maps);
    mem_info.base_address = base as u32;
    mem_info.allocation_base = base as u32;
    mem_info.allocation_protect = constants::PAGE_EXECUTE | constants::PAGE_READWRITE;
    mem_info.state = constants::MEM_COMMIT;
    mem_info.typ = constants::MEM_PRIVATE;
    mem_info.save(out_meminfo_ptr, &mut emu.maps);

    for _ in 0..6 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn NtProtectVirtualMemory(emu: &mut emu::Emu) {
    let sz = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!NtProtectVirtualMemory error reading sz param") as u64;
    let status = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll!NtProtectVirtualMemory error reading status param") as u64;
    let page_number =
        emu.maps
            .read_dword(emu.regs().get_esp() + 8)
            .expect("ntdll!NtProtectVirtualMemory error reading page_number param") as u64;
    let page = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("ntdll!NtProtectVirtualMemory error reading page param") as u64;
    let old_prot_ptr =
        emu.maps
            .read_dword(emu.regs().get_esp() + 16)
            .expect("ntdll!NtProtectVirtualMemory error reading old prot param") as u64;

    log_red!(emu, "ntdll!NtProtectVirtualMemory sz: {}", sz);

    for _ in 0..5 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn CheckRemoteDebuggerPresent(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll! CheckRemoteDebuggerPresenterror reading hndl param") as u64;
    let bool_ptr =
        emu.maps
            .read_dword(emu.regs().get_esp() + 4)
            .expect("ntdll!CheckRemoteDebuggerPresent reading bool ptr param") as u64;

    log_red!(emu, "ntdll!CheckRemoteDebuggerPresent");

    emu.maps.write_dword(bool_ptr, 0);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}

fn memset(emu: &mut emu::Emu) {
    let ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!memset error reading ptr") as u64;
    let byte = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll!memset error reading byte");
    let count = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ntdll!memset error reading count");

    ntdll::memset(emu, ptr, byte as u64, count as u64);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = ptr;
}

fn memcpy(emu: &mut emu::Emu) {
    let dst_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!strcat error reading dst") as u64;
    let src_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll!strcat error reading src") as u64;
    let count = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ntdll!strcat error reading src") as usize;

    ntdll::memcpy(emu, dst_ptr, src_ptr, count);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
}

fn NtFreeVirtualMemory(emu: &mut emu::Emu) {
    let base_addr_ptr = arg(emu, 1) as u64;
    let base_addr = emu.maps.read_dword(base_addr_ptr).unwrap_or(0) as u64;
    log_red!(emu, "ntdll!NtFreeVirtualMemory base: 0x{:x}", base_addr);
    if base_addr != 0 {
        emu.maps.dealloc(base_addr);
    }
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 4);
}
fn NtCreateSection(emu: &mut emu::Emu) {
    let section_handle_ptr = arg(emu, 0) as u64;
    let max_size_ptr = arg(emu, 3) as u64;
    let protection = arg(emu, 4);
    let max_size = if max_size_ptr != 0 {
        emu.maps.read_qword(max_size_ptr).unwrap_or(0x10000)
    } else {
        0x10000
    };
    log_red!(
        emu,
        "ntdll!NtCreateSection max_size: 0x{:x} prot: 0x{:x}",
        max_size,
        protection
    );
    let section = SectionHandle {
        name: format!("section_{:x}", emu.pos),
        max_size,
        protection,
        mapped_addr: None,
        mapped_size: 0,
    };
    let key = emu.handle_management.insert_section_handle(section);
    emu.maps.write_dword(section_handle_ptr, key as u32);
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 7);
}
fn NtMapViewOfSection(emu: &mut emu::Emu) {
    let section_handle = arg(emu, 0);
    let base_addr_ptr = arg(emu, 2) as u64;
    let view_size_ptr = arg(emu, 6) as u64;
    let view_size = emu.maps.read_dword(view_size_ptr).unwrap_or(0) as u64;
    let alloc_size = if view_size == 0 { 0x10000 } else { view_size };
    log_red!(
        emu,
        "ntdll!NtMapViewOfSection handle: {} view_size: 0x{:x}",
        section_handle,
        alloc_size
    );
    let mapped = emu.maps.alloc(alloc_size).and_then(|base| {
        emu.maps
            .create_map(
                &format!("section_map_{:x}", emu.pos),
                base,
                alloc_size,
                Permission::from_flags(true, true, false),
            )
            .ok()
            .map(|_| base)
    });
    let Some(base) = mapped else {
        log::warn!("ntdll!NtMapViewOfSection: cannot map the view");
        emu.regs_mut().rax = constants::STATUS_NO_MEMORY;
        stdcall_ret(emu, 10);
        return;
    };
    emu.maps.write_dword(base_addr_ptr, base as u32);
    if view_size_ptr != 0 {
        emu.maps.write_dword(view_size_ptr, alloc_size as u32);
    }
    if let Some(sh) = emu.handle_management.get_mut_section_handle(section_handle) {
        sh.mapped_addr = Some(base);
        sh.mapped_size = alloc_size;
    }
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 10);
}
fn NtUnmapViewOfSection(emu: &mut emu::Emu) {
    let base_addr = arg(emu, 1) as u64;
    log_red!(emu, "ntdll!NtUnmapViewOfSection base: 0x{:x}", base_addr);
    if base_addr != 0 {
        emu.maps.dealloc(base_addr);
    }
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 2);
}
fn RtlMoveMemory(emu: &mut emu::Emu) {
    let dst = arg(emu, 0) as u64;
    let src = arg(emu, 1) as u64;
    let sz = arg(emu, 2) as usize;
    log_red!(
        emu,
        "ntdll!RtlMoveMemory dst: 0x{:x} src: 0x{:x} sz: {}",
        dst,
        src,
        sz
    );
    ntdll::rtl_move_memory(emu, dst, src, sz);
    stdcall_ret(emu, 3);
}
fn RtlCopyMemory(emu: &mut emu::Emu) {
    let dst = arg(emu, 0) as u64;
    let src = arg(emu, 1) as u64;
    let sz = arg(emu, 2) as usize;
    log_red!(
        emu,
        "ntdll!RtlCopyMemory dst: 0x{:x} src: 0x{:x} sz: {}",
        dst,
        src,
        sz
    );
    if !ntdll::memcpy(emu, dst, src, sz) {
        log::warn!("RtlCopyMemory failed to copy");
    }
    stdcall_ret(emu, 3);
}
fn NtFlushInstructionCache(emu: &mut emu::Emu) {
    let addr = arg(emu, 1);
    let sz = arg(emu, 2);
    log_red!(emu, "ntdll!NtFlushInstructionCache 0x{:x} sz: {}", addr, sz);
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 3);
}
