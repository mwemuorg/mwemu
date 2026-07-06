use crate::api::windows::common::ntdll;
use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::helper;
use crate::windows::{constants, structures};

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "NtAllocateVirtualMemory" => NtAllocateVirtualMemory(emu),
        "NtQueryVirtualMemory" => NtQueryVirtualMemory(emu),
        "RtlZeroMemory" => RtlZeroMemory(emu),
        "RtlMoveMemory" => RtlMoveMemory(emu),
        "NtProtectVirtualMemory" => NtProtectVirtualMemory(emu),
        "memset" => memset(emu),
        "RtlCopyMemory" => RtlCopyMemory(emu),
        "NtFlushInstructionCache" => NtFlushInstructionCache(emu),
        "RtlAddFunctionTable" => RtlAddFunctionTable(emu),
        "RtlCaptureContext" => RtlCaptureContext(emu),
        "RtlLookupFunctionEntry" => RtlLookupFunctionEntry(emu),
        _ => return false,
    }
    true
}

fn RtlZeroMemory(emu: &mut emu::Emu) {
    let dest = emu.regs().rcx;
    let length = emu.regs().rdx;
    ntdll::rtl_zero_memory(emu, dest, length);
}

fn RtlMoveMemory(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let sz = emu.regs().r8 as usize;
    ntdll::rtl_move_memory(emu, dst, src, sz);
}

fn NtAllocateVirtualMemory(emu: &mut emu::Emu) {
    let addr_ptr = emu.regs().rcx;
    let size_ptr = emu.regs().rdx;
    let protection_offset = 0x30;
    let protection_addr = emu.regs().rsp + protection_offset;
    let protect_value = emu
        .maps
        .read_dword(protection_addr)
        .expect("Failed to read Protection argument at NtAllocateVirtualMemory");

    let (can_read, can_write, can_execute) = ntdll::protect_to_rwx(protect_value);

    let addr = emu
        .maps
        .read_qword(addr_ptr)
        .expect("bad NtAllocateVirtualMemory address parameter");
    let size = emu
        .maps
        .read_qword(size_ptr)
        .expect("bad NtAllocateVirtualMemory size parameter");

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
            None => panic!("/!\\ out of memory cannot allocate ntdll!NtAllocateVirtualMemory "),
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

    if !emu.maps.write_qword(addr_ptr, alloc_addr) {
        panic!("NtAllocateVirtualMemory: cannot write on address pointer");
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn NtQueryVirtualMemory(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let addr = emu.regs().rdx;

    log_red!(emu, "ntdll!NtQueryVirtualMemory addr: 0x{:x}", addr);

    if handle != 0xffffffff {
        log::trace!("\tusing handle of remote process {:x}", handle);

        if !helper::handler_exist(handle) {
            log::trace!("\nhandler doesnt exist.");
        }
    }

    let out_meminfo_ptr = emu.regs().r9;

    if !emu.maps.is_mapped(addr) {
        log::trace!(
            "/!\\ ntdll!NtQueryVirtualMemory: querying non maped addr: 0x{:x}",
            addr
        );

        emu.regs_mut().rax = constants::STATUS_INVALID_PARAMETER;
        return;
    }

    let base = emu.maps.get_addr_base(addr).unwrap_or(0);
    let region_size = emu
        .maps
        .get_mem_by_addr(addr)
        .map(|m| m.size() as u64)
        .unwrap_or(0);
    let alloc_protect = constants::PAGE_EXECUTE | constants::PAGE_READWRITE;
    let mem_info = structures::MemoryBasicInformation64 {
        base_address: base,
        allocation_base: base,
        allocation_protect: alloc_protect,
        partition_id: 0,
        reserved: 0,
        region_size,
        state: constants::MEM_COMMIT,
        protect: alloc_protect,
        typ: constants::MEM_PRIVATE,
    };
    mem_info.save(out_meminfo_ptr, &mut emu.maps);

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn NtProtectVirtualMemory(emu: &mut emu::Emu) {
    let sz = emu.regs().rcx;
    let status = emu.regs().rdx;
    let page_number = emu.regs().r8;
    let page = emu.regs().r9;
    let prot = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("ntdll!NtProtectVirtualMemory error reading old prot param");

    log_red!(emu, "ntdll!NtProtectVirtualMemory sz: {} {}", sz, prot);

    emu.regs_mut().rax = constants::STATUS_SUCCESS
}

fn memset(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    let byte = emu.regs().rdx;
    let count = emu.regs().r8;
    ntdll::memset(emu, ptr, byte, count);

    emu.regs_mut().rax = ptr;
}

fn RtlCopyMemory(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let sz = emu.regs().r8 as usize;
    let result = ntdll::memcpy(emu, dst, src, sz);
    if !result {
        panic!("RtlCopyMemory failed to copy");
    }
    log_red!(
        emu,
        "** {} ntdll!RtlCopyMemory dst = {:x} src = {:x} sz = {}",
        emu.pos,
        dst,
        src,
        sz
    );
}

fn NtFlushInstructionCache(emu: &mut emu::Emu) {
    let proc_hndl = emu.regs().rcx;
    let addr = emu.regs().rdx;
    let sz = emu.regs().r8;

    log_red!(
        emu,
        "ntdll!NtFlushInstructionCache hndl: {:x} 0x{:x} sz: {}",
        proc_hndl,
        addr,
        sz
    );

    emu.regs_mut().rax = 0;
}

fn RtlAddFunctionTable(emu: &mut emu::Emu) {
    let function_table = emu.regs().rcx;
    let entry_count = emu.regs().rdx;
    let base_address = emu.regs().r8;

    log_red!(emu, "ntdll!RtlAddFunctionTable");

    emu.regs_mut().rax = 1;
}

fn RtlCaptureContext(emu: &mut emu::Emu) {
    let context_record = emu.regs().rcx as usize;
    log_red!(
        emu,
        "** {} ntdll!RtlCaptureContext {:x}",
        emu.pos,
        context_record
    );
}

fn RtlLookupFunctionEntry(emu: &mut emu::Emu) {
    let control_pc = emu.regs().rcx as usize;
    let image_base = emu.regs().rdx as usize;
    let history_table = emu.regs().r8 as usize;
    log_red!(
        emu,
        "** {} ntdll!RtlLookupFunctionEntry {:x} {:x} {:x}",
        emu.pos,
        control_pc,
        image_base,
        history_table
    );
    emu.regs_mut().rax = 0;
}
