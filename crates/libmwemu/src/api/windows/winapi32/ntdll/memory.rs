use crate::api::windows::common::ntdll;
use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::helper;
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
        panic!("NtAllocateVirtualMemory mapping zero bytes.")
    }

    let alloc_addr: u64 = if do_alloc {
        match emu.maps.alloc(size) {
            Some(a) => a,
            None => {
                panic!("/!\\ out of memory   cannot allocate forntdll!NtAllocateVirtualMemory ")
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
        panic!("NtAllocateVirtualMemory: cannot write on address pointer");
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
