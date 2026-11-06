use crate::emu;
use crate::kernel::heap::Region;
use crate::maps::mem64::Permission;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi64::kernel32;

const LARGE_ALLOC_THRESHOLD: u64 = 0x8000;

fn allocate_memory(emu: &mut emu::Emu, size: u64) -> Option<u64> {
    if size < LARGE_ALLOC_THRESHOLD {
        let heap_manage = emu.heap_mut();
        return heap_manage.allocate(size as usize);
    }

    let allocation = emu.maps.alloc(size)?;
    emu.maps
        .create_map(
            &format!("alloc_{:x}", allocation),
            allocation,
            size,
            Permission::READ_WRITE,
        )
        .ok()?;
    Some(allocation)
}

enum AllocKind {
    Arena { size: usize },
    Map { base: u64, size: usize },
    Invalid,
}

fn classify(emu: &emu::Emu, addr: u64) -> AllocKind {
    if let Some(heap) = emu.heap_arenas.first()
        && let Some(size) = heap.allocation_size(addr)
    {
        return AllocKind::Arena { size };
    }

    match emu.maps.get_mem_by_addr(addr) {
        Some(mem) if mem.get_base() == addr && mem.get_name().starts_with("alloc_") => {
            AllocKind::Map {
                base: addr,
                size: mem.size(),
            }
        }
        _ => AllocKind::Invalid,
    }
}

fn release(emu: &mut emu::Emu, addr: u64) {
    match classify(emu, addr) {
        AllocKind::Arena { .. } => {
            if let Some(heap) = emu.heap_arenas.first_mut() {
                heap.free(addr);
            }
        }
        AllocKind::Map { base, .. } => emu.maps.dealloc(base),
        AllocKind::Invalid => {}
    }
}

// --- Native execution helpers (kept from the original 64-bit msvcrt) ---

const NATIVE_SECTIONS: [&str; 2] = ["msvcrt.text", "msvcrtfothk"];

const LOGGED_FUNCTIONS: [&str; 8] = [
    "__set_app_type",
    "malloc",
    "realloc",
    "_errno",
    "_lock",
    "__dllonexit",
    "_msize",
    "_initterm",
];

pub(crate) fn is_native_section(section_name: &str) -> bool {
    NATIVE_SECTIONS.contains(&section_name)
}

pub(crate) fn log_native_call(emu: &mut emu::Emu, addr: u64) {
    let Some(fn_name) = emu.export_indexes.resolve_address_module("msvcrt", addr) else {
        return;
    };
    if !LOGGED_FUNCTIONS.contains(&fn_name) {
        return;
    }
    log_red!(emu, "executing msvcrt!{}", fn_name);
}

// --- 64-bit CRT gateway ---

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    gateway_by_name(api, emu)
}

pub fn gateway_by_name(api: &str, emu: &mut emu::Emu) -> String {
    match api {
        "_initterm_e" => {
            let start_ptr = emu.regs().rcx;
            let end_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "msvcrt!_initterm_e 0x{:x} - 0x{:x}",
                start_ptr,
                end_ptr
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            let mut ptr = start_ptr;
            while ptr < end_ptr {
                let func = emu.maps.read_qword(ptr).unwrap_or(0);
                if func != 0 {
                    log_red!(emu, "  _initterm_e: init func 0x{:x}", func);
                }
                ptr += 8;
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "_initterm" => {
            let start_ptr = emu.regs().rcx;
            let end_ptr = emu.regs().rdx;
            log_red!(emu, "msvcrt!_initterm 0x{:x} - 0x{:x}", start_ptr, end_ptr);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            let mut ptr = start_ptr;
            while ptr < end_ptr {
                let func = emu.maps.read_qword(ptr).unwrap_or(0);
                if func != 0 {
                    log_red!(emu, "  _initterm: init func 0x{:x}", func);
                }
                ptr += 8;
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "StrCmpCA" => {
            let str1_ptr = emu.regs().rcx;
            let str2_ptr = emu.regs().rdx;
            if str1_ptr == 0 || str2_ptr == 0 {
                emu.regs_mut().rax = 0xffffffff;
                log_red!(emu, "msvcrt!StrCmpA null ptrs");
            } else {
                let str1 = emu.maps.read_string(str1_ptr);
                let str2 = emu.maps.read_string(str2_ptr);
                log_red!(emu, "msvcrt!StrCmpA {} == {}", str1, str2);
                if str1 == str2 {
                    emu.regs_mut().rax = 0;
                } else {
                    emu.regs_mut().rax = 0xffffffff;
                }
            }
        }
        "fopen" => {
            let filepath_ptr = emu.regs().rcx;
            let mode_ptr = emu.regs().rdx;
            if filepath_ptr == 0 || mode_ptr == 0 {
                emu.regs_mut().rax = 0xffffffff;
                log_red!(emu, "msvcrt!fopen null ptrs");
            } else {
                let filepath = emu.maps.read_string(filepath_ptr);
                let mode = emu.maps.read_string(mode_ptr);
                log_red!(emu, "msvcrt!fopen `{}` fmt:`{}`", filepath, mode);
                emu.regs_mut().rax = helper::handler_create(&filepath);
            }
        }
        "fwrite" => {
            let buff_ptr = emu.regs().rcx;
            let size = emu.regs().rdx as u32;
            let nemb = emu.regs().r8 as u32;
            let file = emu.regs().r9;
            let filename = helper::handler_get_uri(file);
            log_red!(
                emu,
                "msvcrt!fwrite `{}` 0x{:x} {}",
                filename,
                buff_ptr,
                size
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = size as u64;
        }
        "fflush" => {
            let file = emu.regs().rcx;
            let filename = helper::handler_get_uri(file);
            log_red!(emu, "msvcrt!fflush `{}`", filename);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "fclose" => {
            let file = emu.regs().rcx;
            let filename = helper::handler_get_uri(file);
            helper::handler_close(file);
            log_red!(emu, "msvcrt!fclose '{}' handle:0x{:x}", filename, file);
            emu.regs_mut().rax = 0;
        }
        "__p___argv" => {
            let argv_base = match emu.maps.get_map_by_name("argv") {
                Some(a) => a.get_base(),
                None => {
                    let addr = match emu.maps.alloc(1024) {
                        Some(a) => a,
                        None => {
                            log::error!("msvcrt!__p___argv: out of memory");
                            emu.regs_mut().rax = 0;
                            return String::new();
                        }
                    };
                    match emu
                        .maps
                        .create_map("argv", addr, 1024, Permission::READ_WRITE)
                    {
                        Ok(m) => m.get_base(),
                        Err(e) => {
                            log::error!("msvcrt!__p___argv: cannot create argv map: {}", e);
                            emu.regs_mut().rax = 0;
                            return String::new();
                        }
                    };
                    let str_area = addr + 64;
                    emu.maps.write_string(str_area, "mwemu.exe");
                    emu.maps.write_qword(addr, str_area);
                    emu.maps.write_qword(addr + 8, 0);
                    addr
                }
            };
            log_red!(emu, "msvcrt!__p___argv => 0x{:x}", argv_base);
            emu.set_last_error(0);
            emu.regs_mut().rax = argv_base;
        }
        "__p___argc" => {
            let args_base = match emu.maps.get_map_by_name("args") {
                Some(a) => a.get_base(),
                None => {
                    let addr = match emu.maps.alloc(1024) {
                        Some(a) => a,
                        None => {
                            log::error!("msvcrt!__p___argc: out of memory");
                            emu.regs_mut().rax = 0;
                            return String::new();
                        }
                    };
                    match emu
                        .maps
                        .create_map("args", addr, 1024, Permission::READ_WRITE)
                    {
                        Ok(m) => m.get_base(),
                        Err(e) => {
                            log::error!("msvcrt!__p___argc: cannot create args map: {}", e);
                            emu.regs_mut().rax = 0;
                            return String::new();
                        }
                    }
                }
            };
            log_red!(emu, "msvcrt!__p___argc {}", args_base);
            emu.regs_mut().rax = args_base;
        }
        "malloc" => {
            let size = emu.regs().rcx;
            if size > 0 {
                let base = if emu.cfg.memory_guard {
                    emu.kernel_alloc(Region::Slab, size, "malloc", "malloc", false)
                } else {
                    allocate_memory(emu, size).unwrap_or(0)
                };
                log_red!(emu, "msvcrt!malloc sz: {} addr: 0x{:x}", size, base);
                emu.set_last_error(0);
                emu.regs_mut().rax = base;
            } else {
                emu.set_last_error(0);
                emu.regs_mut().rax = 0x1337;
            }
        }
        "_onexit" => {
            let fptr = emu.regs().rcx;
            log_red!(emu, "msvcrt!_onexit 0x{:x}", fptr);
            let slot = allocate_memory(emu, 8).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_qword(slot, fptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = fptr;
        }
        "_lock" => {
            let lock_num = emu.regs().rcx as u32;
            log_red!(emu, "msvcrt!_lock {}", lock_num);
            let slot = allocate_memory(emu, 4).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_dword(slot, lock_num);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "free" => {
            let addr = emu.regs().rcx;
            log_red!(emu, "msvcrt!free 0x{:x}", addr);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            if emu.cfg.memory_guard {
                emu.kernel_free(addr, "free");
            } else {
                release(emu, addr);
            }
        }
        "realloc" => {
            let ptr = emu.regs().rcx;
            let size = emu.regs().rdx;
            if ptr == 0 {
                if size == 0 {
                    emu.regs_mut().rax = 0;
                } else {
                    let base = allocate_memory(emu, size).unwrap_or(0);
                    log_red!(emu, "msvcrt!realloc 0x{:x} {} =0x{:x}", ptr, size, base);
                    emu.set_last_error(0);
                    emu.regs_mut().rax = base;
                }
            } else if size == 0 {
                log_red!(emu, "msvcrt!realloc 0x{:x} {} =0x1337", ptr, size);
                emu.regs_mut().rax = 0x1337;
            } else {
                let prev_size = match classify(emu, ptr) {
                    AllocKind::Arena { size } | AllocKind::Map { size, .. } => size,
                    AllocKind::Invalid => {
                        emu.regs_mut().rax = 0;
                        return String::new();
                    }
                };
                let new_addr = if emu.cfg.memory_guard {
                    emu.kernel_alloc(Region::Slab, size, "malloc", "realloc", false)
                } else {
                    allocate_memory(emu, size).unwrap_or(0)
                };
                emu.maps
                    .memcpy(new_addr, ptr, std::cmp::min(prev_size, size as usize));
                if emu.cfg.memory_guard {
                    emu.kernel_free(ptr, "realloc");
                } else {
                    release(emu, ptr);
                }
                log_red!(emu, "msvcrt!realloc 0x{:x} {} =0x{:x}", ptr, size, new_addr);
                emu.regs_mut().rax = new_addr;
            }
        }
        "strtok" => {
            let str_ptr = emu.regs().rcx;
            let delim_ptr = emu.regs().rdx;
            let delims = emu.maps.read_string(delim_ptr);
            if str_ptr != 0 {
                let s = emu.maps.read_string(str_ptr);
                if let Some(pos) = s.find(|c: char| delims.contains(c)) {
                    emu.maps.write_byte(str_ptr + pos as u64, 0);
                    log_red!(
                        emu,
                        "msvcrt!strtok '{}' delim:'{}' => '{}'",
                        s,
                        delims,
                        &s[..pos]
                    );
                    emu.regs_mut().rax = str_ptr;
                } else if !s.is_empty() {
                    log_red!(emu, "msvcrt!strtok '{}' delim:'{}' => '{}'", s, delims, s);
                    emu.regs_mut().rax = str_ptr;
                } else {
                    log_red!(emu, "msvcrt!strtok empty");
                    emu.set_last_error(0);
                    emu.regs_mut().rax = 0;
                }
            } else {
                log_red!(emu, "msvcrt!strtok(NULL) => 0");
                emu.regs_mut().rax = 0;
            }
        }
        "exit" | "_exit" => {
            let status = emu.regs().rcx as u32;
            log_red!(emu, "msvcrt!exit {}", status);
            let slot = allocate_memory(emu, 4).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_dword(slot, status);
            }
            emu.stop();
        }
        "__set_app_type" => {
            let app_type = emu.regs().rcx as u32;
            log_red!(emu, "msvcrt!__set_app_type  app_type: 0x{:x}", app_type);
            let slot = allocate_memory(emu, 4).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_dword(slot, app_type);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_errno" => {
            let errno_addr = match emu.maps.get_map_by_name("msvcrt_errno") {
                Some(m) => m.get_base(),
                None => {
                    let addr = allocate_memory(emu, 8).unwrap_or(0);
                    if addr != 0 {
                        emu.maps.write_dword(addr, 0);
                    }
                    addr
                }
            };
            log_red!(emu, "msvcrt!_errno => 0x{:x}", errno_addr);
            emu.set_last_error(0);
            emu.regs_mut().rax = errno_addr;
        }
        "__p__fmode" | "__p__commode" | "_adjust_fdiv" => {
            let addr = allocate_memory(emu, 4).unwrap_or(0);
            if addr != 0 {
                emu.maps.write_dword(addr, 0);
            }
            log_red!(emu, "msvcrt!{} => 0x{:x}", api, addr);
            emu.set_last_error(0);
            emu.regs_mut().rax = addr;
        }
        "__setusermatherr" => {
            let handler = emu.regs().rcx;
            log_red!(emu, "msvcrt!__setusermatherr handler:0x{:x}", handler);
            let slot = allocate_memory(emu, 8).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_qword(slot, handler);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "_controlfp" => {
            let new_val = emu.regs().rcx as u32;
            let mask = emu.regs().rdx as u32;
            log_red!(
                emu,
                "msvcrt!_controlfp new:0x{:x} mask:0x{:x}",
                new_val,
                mask
            );
            let result = 0x0009_001fu32;
            let slot = allocate_memory(emu, 4).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_dword(slot, result);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = result as u64;
        }
        "__getmainargs" => {
            let p_argc = emu.regs().rcx;
            let p_argv = emu.regs().rdx;
            let p_env = emu.regs().r8;
            log_red!(emu, "msvcrt!__getmainargs");
            if p_argc != 0 {
                emu.maps.write_qword(p_argc, 1);
            }
            if p_argv != 0 {
                emu.maps.write_qword(p_argv, 0);
            }
            if p_env != 0 {
                emu.maps.write_qword(p_env, 0);
            }
            emu.regs_mut().rax = 0;
        }
        "_acmdln" | "__p__acmdln" => {
            let args_base = match emu.maps.get_map_by_name("args") {
                Some(a) => a.get_base(),
                None => {
                    let addr = match emu.maps.alloc(1024) {
                        Some(a) => a,
                        None => {
                            log::error!("msvcrt!__p__acmdln: out of memory");
                            emu.regs_mut().rax = 0;
                            return String::new();
                        }
                    };
                    match emu
                        .maps
                        .create_map("args", addr, 1024, Permission::READ_WRITE)
                    {
                        Ok(m) => m.get_base(),
                        Err(e) => {
                            log::error!("msvcrt!__p__acmdln: cannot create args map: {}", e);
                            emu.regs_mut().rax = 0;
                            return String::new();
                        }
                    };
                    let str_area = addr + 128;
                    emu.maps.write_string(str_area, "mwemu.exe");
                    emu.maps.write_qword(addr + 16, str_area);
                    addr
                }
            };
            log_red!(emu, "msvcrt!__p__acmdln => 0x{:x}", args_base + 16);
            emu.set_last_error(0);
            emu.regs_mut().rax = args_base + 16;
        }
        "_XcptFilter" => {
            let exc_code = emu.regs().rcx;
            let exc_ptrs = emu.regs().rdx;
            if exc_ptrs != 0 {
                let exc_record = emu.maps.read_qword(exc_ptrs).unwrap_or(0);
                let ctx_record = emu.maps.read_qword(exc_ptrs + 8).unwrap_or(0);
                log_red!(
                    emu,
                    "msvcrt!_XcptFilter code:0x{:x} ptrs:0x{:x} rec:0x{:x} ctx:0x{:x}",
                    exc_code,
                    exc_ptrs,
                    exc_record,
                    ctx_record
                );
                let first_param = emu.regs().rcx;
                if first_param != 0 && emu.maps.is_mapped(first_param) {
                    let _ = emu.maps.read_dword(first_param).unwrap_or(0);
                }
            } else {
                log_red!(emu, "msvcrt!_XcptFilter code:0x{:x} ptrs:NULL", exc_code);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1; // EXCEPTION_EXECUTE_HANDLER
        }
        "__dllonexit" => {
            let func = emu.regs().rcx;
            let p_begin = emu.regs().rdx;
            let p_end = emu.regs().r8;
            if p_begin != 0 {
                let begin = emu.maps.read_qword(p_begin).unwrap_or(0);
                let end = if p_end != 0 {
                    emu.maps.read_qword(p_end).unwrap_or(0)
                } else {
                    0
                };
                log_red!(
                    emu,
                    "msvcrt!__dllonexit func:0x{:x} begin:0x{:x}(0x{:x}) end:0x{:x}(0x{:x})",
                    func,
                    p_begin,
                    begin,
                    p_end,
                    end
                );
                let first_param = emu.regs().rcx;
                if first_param != 0 && emu.maps.is_mapped(first_param) {
                    let _ = emu.maps.read_dword(first_param).unwrap_or(0);
                }
            } else {
                log_red!(
                    emu,
                    "msvcrt!__dllonexit func:0x{:x} begin:0x{:x} end:0x{:x}",
                    func,
                    p_begin,
                    p_end
                );
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = func;
        }
        "_except_handler3" | "__CxxFrameHandler" => {
            let exception_record = emu.regs().rcx;
            let exc_code = if exception_record != 0 {
                emu.maps.read_dword(exception_record).unwrap_or(0)
            } else {
                0
            };
            log_red!(
                emu,
                "msvcrt!{} exc_record:0x{:x} code:0x{:x}",
                api,
                exception_record,
                exc_code
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_EH_prolog" => {
            let slot = allocate_memory(emu, 8).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_qword(slot, emu.regs().rbp);
            }
            log_red!(emu, "msvcrt!_EH_prolog (64-bit)");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_CxxThrowException" => {
            let exc_object = emu.regs().rcx;
            let throw_info = emu.regs().rdx;
            let vtable = if exc_object != 0 {
                emu.maps.read_qword(exc_object).unwrap_or(0)
            } else {
                0
            };
            log_red!(
                emu,
                "msvcrt!_CxxThrowException obj:0x{:x} vtable:0x{:x} info:0x{:x}",
                exc_object,
                vtable,
                throw_info
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "_ftol" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val as i64;
            emu.fpu_mut().set_st(0, result as f64);
            log_red!(emu, "msvcrt!_ftol {} => {}", val, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = result as u64;
        }
        "time" => {
            let time_ptr = emu.regs().rcx;
            let tick = emu.tick as u32;
            if time_ptr != 0 {
                emu.maps.write_dword(time_ptr, tick);
            }
            log_red!(emu, "msvcrt!time => {}", tick);
            emu.set_last_error(0);
            emu.regs_mut().rax = tick as u64;
        }
        "localtime" => {
            let time_ptr = emu.regs().rcx;
            let tm_addr = allocate_memory(emu, 64).unwrap_or(0);
            if tm_addr != 0 {
                emu.maps.write_dword(tm_addr, 0); // tm_sec
                emu.maps.write_dword(tm_addr + 4, 30); // tm_min
                emu.maps.write_dword(tm_addr + 8, 12); // tm_hour
                emu.maps.write_dword(tm_addr + 12, 1); // tm_mday
                emu.maps.write_dword(tm_addr + 16, 0); // tm_mon
                emu.maps.write_dword(tm_addr + 20, 126); // tm_year (2026)
                emu.maps.write_dword(tm_addr + 24, 4); // tm_wday
                emu.maps.write_dword(tm_addr + 28, 0); // tm_yday
                emu.maps.write_dword(tm_addr + 32, 0); // tm_isdst
            }
            log_red!(
                emu,
                "msvcrt!localtime time_ptr:0x{:x} => 0x{:x}",
                time_ptr,
                tm_addr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = tm_addr;
        }
        "strftime" => {
            let dst = emu.regs().rcx;
            let max_size = emu.regs().rdx as u32;
            let s = "01/01/2024 00:00:00";
            if dst != 0 && max_size > 0 {
                emu.maps.write_string(dst, s);
            }
            log_red!(emu, "msvcrt!strftime => '{}'", s);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = s.len() as u64;
        }
        "srand" => {
            let seed = emu.regs().rcx as u32;
            log_red!(emu, "msvcrt!srand seed:{}", seed);
            let slot = allocate_memory(emu, 4).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_dword(slot, seed);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "rand" => {
            let seed = allocate_memory(emu, 4).unwrap_or(0);
            let val = if seed != 0 {
                let s = emu.maps.read_dword(seed).unwrap_or(12345);
                let next = s.wrapping_mul(214013).wrapping_add(2531011);
                emu.maps.write_dword(seed, next);
                ((next >> 16) & 0x7fff) as u64
            } else {
                42
            };
            log_red!(emu, "msvcrt!rand => {}", val);
            emu.set_last_error(0);
            emu.regs_mut().rax = val;
        }
        "atoi" => {
            let str_ptr = emu.regs().rcx;
            let s = emu.maps.read_string(str_ptr);
            let val = s.trim().parse::<i32>().unwrap_or(0);
            log_red!(emu, "msvcrt!atoi '{}' => {}", s, val);
            emu.set_last_error(0);
            emu.regs_mut().rax = val as u32 as u64;
        }
        "_itoa" | "_itow" => {
            let val = emu.regs().rcx as u32;
            let buf = emu.regs().rdx;
            let radix = emu.regs().r8 as u32;
            let s = match radix {
                16 => format!("{:x}", val),
                8 => format!("{:o}", val),
                _ => format!("{}", val),
            };
            if buf != 0 {
                emu.maps.write_string(buf, &s);
            }
            log_red!(emu, "msvcrt!{} {} => '{}'", api, val, s);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = buf;
        }
        "getenv" | "_wgetenv" => {
            let ptr = emu.regs().rcx;
            let name = emu.maps.read_string(ptr);
            log_red!(emu, "msvcrt!{} '{}' => NULL", api, name);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "printf" => {
            let fmt_ptr = emu.regs().rcx;
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "msvcrt!printf fmt:'{}'", fmt);
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "sprintf" => {
            let buf = emu.regs().rcx;
            let fmt_ptr = emu.regs().rdx;
            let fmt = emu.maps.read_string(fmt_ptr);
            if buf != 0 {
                emu.maps.write_string(buf, &fmt);
            }
            log_red!(emu, "msvcrt!sprintf fmt:'{}'", fmt);
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "swprintf" => {
            let buf = emu.regs().rcx;
            let fmt_ptr = emu.regs().rdx;
            let fmt = emu.maps.read_wide_string(fmt_ptr);
            if buf != 0 {
                emu.maps.write_wide_string(buf, &fmt);
            }
            log_red!(emu, "msvcrt!swprintf fmt:'{}'", fmt);
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "strncmp" => {
            let s1_ptr = emu.regs().rcx;
            let s2_ptr = emu.regs().rdx;
            let n = emu.regs().r8 as usize;
            let s1 = emu.maps.read_string(s1_ptr);
            let s2 = emu.maps.read_string(s2_ptr);
            let cmp1: String = s1.chars().take(n).collect();
            let cmp2: String = s2.chars().take(n).collect();
            let result = cmp1.cmp(&cmp2) as i32;
            log_red!(emu, "msvcrt!strncmp '{}' '{}' {} => {}", s1, s2, n, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = result as u32 as u64;
        }
        "toupper" => {
            let c = emu.regs().rcx as u32;
            let result = if (0x61..=0x7a).contains(&c) {
                c - 0x20
            } else {
                c
            };
            let slot = allocate_memory(emu, 4).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_dword(slot, result);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = result as u64;
        }
        "tolower" => {
            let c = emu.regs().rcx as u32;
            let result = if (0x41..=0x5a).contains(&c) {
                c + 0x20
            } else {
                c
            };
            let slot = allocate_memory(emu, 4).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_dword(slot, result);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = result as u64;
        }
        "wcslen" => {
            let s_ptr = emu.regs().rcx;
            let s = emu.maps.read_wide_string(s_ptr);
            let len = s.len();
            log_red!(emu, "msvcrt!wcslen '{}' => {}", s, len);
            emu.set_last_error(0);
            emu.regs_mut().rax = len as u64;
        }
        "wcscpy" => {
            let dst = emu.regs().rcx;
            let src = emu.regs().rdx;
            let s = emu.maps.read_wide_string(src);
            if dst != 0 {
                emu.maps.write_wide_string(dst, &s);
            }
            log_red!(emu, "msvcrt!wcscpy dst:0x{:x} '{}'", dst, s);
            emu.regs_mut().rax = dst;
        }
        "wcscat" => {
            let dst = emu.regs().rcx;
            let src = emu.regs().rdx;
            let existing = emu.maps.read_wide_string(dst);
            let append = emu.maps.read_wide_string(src);
            let result = format!("{}{}", existing, append);
            if dst != 0 {
                emu.maps.write_wide_string(dst, &result);
            }
            log_red!(emu, "msvcrt!wcscat => '{}'", result);
            emu.set_last_error(0);
            emu.regs_mut().rax = dst;
        }
        "wcscmp" => {
            let s1_ptr = emu.regs().rcx;
            let s2_ptr = emu.regs().rdx;
            let s1 = emu.maps.read_wide_string(s1_ptr);
            let s2 = emu.maps.read_wide_string(s2_ptr);
            let result = s1.cmp(&s2) as i32;
            log_red!(emu, "msvcrt!wcscmp '{}' '{}' => {}", s1, s2, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = result as u32 as u64;
        }
        "freopen" => {
            let path_ptr = emu.regs().rcx;
            let path = emu.maps.read_string(path_ptr);
            let hndl = helper::handler_create(&path);
            log_red!(emu, "msvcrt!freopen '{}' => 0x{:x}", path, hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "_wrename" => {
            let old_ptr = emu.regs().rcx;
            let new_ptr = emu.regs().rdx;
            let old_name = emu.maps.read_wide_string(old_ptr);
            let new_name = emu.maps.read_wide_string(new_ptr);
            log_red!(emu, "msvcrt!_wrename '{}' => '{}'", old_name, new_name);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "_wsystem" => {
            let cmd_ptr = emu.regs().rcx;
            let cmd = emu.maps.read_wide_string(cmd_ptr);
            log_red!(emu, "msvcrt!_wsystem cmd:'{}'", cmd);
            emu.regs_mut().rax = 0;
        }
        "_iob" => {
            let iob_addr = allocate_memory(emu, 96).unwrap_or(0);
            log_red!(emu, "msvcrt!_iob => 0x{:x}", iob_addr);
            emu.set_last_error(0);
            emu.regs_mut().rax = iob_addr;
        }
        "??1type_info@@UAE@XZ" => {
            let this_ptr = emu.regs().rcx;
            let vtable = if this_ptr != 0 {
                emu.maps.read_qword(this_ptr).unwrap_or(0)
            } else {
                0
            };
            log_red!(
                emu,
                "msvcrt!~type_info this:0x{:x} vtable:0x{:x}",
                this_ptr,
                vtable
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
        }
        "??0exception@@QAE@ABV0@@Z" => {
            let this_ptr = emu.regs().rcx;
            let other = emu.regs().rdx;
            if other != 0 && this_ptr != 0 {
                let vtable = emu.maps.read_qword(other).unwrap_or(0);
                let msg_ptr = emu.maps.read_qword(other + 8).unwrap_or(0);
                emu.maps.write_qword(this_ptr, vtable);
                emu.maps.write_qword(this_ptr + 8, msg_ptr);
            }
            log_red!(
                emu,
                "msvcrt!exception::exception(copy) this:0x{:x} other:0x{:x}",
                this_ptr,
                other
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = this_ptr;
        }
        "??2@YAPAXI@Z" => {
            let size = emu.regs().rcx;
            let base = allocate_memory(emu, size).unwrap_or(0);
            log_red!(emu, "msvcrt!operator_new size:{} => 0x{:x}", size, base);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = base;
        }
        "??3@YAXPAX@Z" => {
            let ptr = emu.regs().rcx;
            log_red!(emu, "msvcrt!operator_delete ptr:0x{:x}", ptr);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            release(emu, ptr);
        }

        // --- String functions ---
        "strcmp" => {
            let s1_ptr = emu.regs().rcx;
            let s2_ptr = emu.regs().rdx;
            let s1 = emu.maps.read_string(s1_ptr);
            let s2 = emu.maps.read_string(s2_ptr);
            let result = s1.cmp(&s2) as i32;
            log_red!(emu, "msvcrt!strcmp '{}' '{}' => {}", s1, s2, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = result as u32 as u64;
        }
        "strcpy" => {
            let dst = emu.regs().rcx;
            let src = emu.regs().rdx;
            let s = emu.maps.read_string(src);
            if dst != 0 {
                emu.maps.write_string(dst, &s);
            }
            log_red!(emu, "msvcrt!strcpy dst:0x{:x} '{}'", dst, s);
            emu.regs_mut().rax = dst;
        }
        "strncpy" => {
            let dst = emu.regs().rcx;
            let src = emu.regs().rdx;
            let n = emu.regs().r8 as usize;
            let s = emu.maps.read_string(src);
            let truncated: String = s.chars().take(n).collect();
            if dst != 0 {
                emu.maps.write_string(dst, &truncated);
            }
            log_red!(
                emu,
                "msvcrt!strncpy dst:0x{:x} '{}' n:{}",
                dst,
                truncated,
                n
            );
            emu.regs_mut().rax = dst;
        }
        "strcpy_s" => {
            let dst = emu.regs().rcx;
            let _dst_size = emu.regs().rdx;
            let src = emu.regs().r8;
            let s = emu.maps.read_string(src);
            if dst != 0 {
                emu.maps.write_string(dst, &s);
            }
            log_red!(emu, "msvcrt!strcpy_s dst:0x{:x} '{}'", dst, s);
            emu.regs_mut().rax = 0;
        }
        "strncpy_s" => {
            let dst = emu.regs().rcx;
            let _dst_size = emu.regs().rdx;
            let src = emu.regs().r8;
            let n = emu.regs().r9 as usize;
            let s = emu.maps.read_string(src);
            let truncated: String = s.chars().take(n).collect();
            if dst != 0 {
                emu.maps.write_string(dst, &truncated);
            }
            log_red!(
                emu,
                "msvcrt!strncpy_s dst:0x{:x} '{}' n:{}",
                dst,
                truncated,
                n
            );
            emu.regs_mut().rax = 0;
        }
        "wcscpy_s" => {
            let dst = emu.regs().rcx;
            let _dst_size = emu.regs().rdx;
            let src = emu.regs().r8;
            let s = emu.maps.read_wide_string(src);
            if dst != 0 {
                emu.maps.write_wide_string(dst, &s);
            }
            log_red!(emu, "msvcrt!wcscpy_s dst:0x{:x} '{}'", dst, s);
            emu.regs_mut().rax = 0;
        }
        "wcsncpy_s" => {
            let dst = emu.regs().rcx;
            let _dst_size = emu.regs().rdx;
            let src = emu.regs().r8;
            let n = emu.regs().r9 as usize;
            let s = emu.maps.read_wide_string(src);
            let truncated: String = s.chars().take(n).collect();
            if dst != 0 {
                emu.maps.write_wide_string(dst, &truncated);
            }
            log_red!(
                emu,
                "msvcrt!wcsncpy_s dst:0x{:x} '{}' n:{}",
                dst,
                truncated,
                n
            );
            emu.regs_mut().rax = 0;
        }
        "strcat_s" => {
            let dst = emu.regs().rcx;
            let _dst_size = emu.regs().rdx;
            let src = emu.regs().r8;
            let existing = emu.maps.read_string(dst);
            let append = emu.maps.read_string(src);
            let result = format!("{}{}", existing, append);
            if dst != 0 {
                emu.maps.write_string(dst, &result);
            }
            log_red!(emu, "msvcrt!strcat_s => '{}'", result);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "wcscat_s" => {
            let dst = emu.regs().rcx;
            let _dst_size = emu.regs().rdx;
            let src = emu.regs().r8;
            let existing = emu.maps.read_wide_string(dst);
            let append = emu.maps.read_wide_string(src);
            let result = format!("{}{}", existing, append);
            if dst != 0 {
                emu.maps.write_wide_string(dst, &result);
            }
            log_red!(emu, "msvcrt!wcscat_s => '{}'", result);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "strcat" => {
            let dst = emu.regs().rcx;
            let src = emu.regs().rdx;
            let existing = emu.maps.read_string(dst);
            let append = emu.maps.read_string(src);
            let result = format!("{}{}", existing, append);
            if dst != 0 {
                emu.maps.write_string(dst, &result);
            }
            log_red!(emu, "msvcrt!strcat => '{}'", result);
            emu.set_last_error(0);
            emu.regs_mut().rax = dst;
        }
        "strncat" => {
            let dst = emu.regs().rcx;
            let src = emu.regs().rdx;
            let n = emu.regs().r8 as usize;
            let dst_s = emu.maps.read_string(dst);
            let src_s = emu.maps.read_string(src);
            let copy_len = src_s.len().min(n);
            let offset = dst + dst_s.len() as u64;
            for (i, b) in src_s.as_bytes()[..copy_len].iter().enumerate() {
                emu.maps.write_byte(offset + i as u64, *b);
            }
            emu.maps.write_byte(offset + copy_len as u64, 0);
            log_red!(emu, "msvcrt!strncat 0x{:x} '{}' {}", dst, src_s, n);
            emu.regs_mut().rax = dst;
        }
        "strstr" => {
            let haystack_ptr = emu.regs().rcx;
            let needle_ptr = emu.regs().rdx;
            let haystack = emu.maps.read_string(haystack_ptr);
            let needle = emu.maps.read_string(needle_ptr);
            let found = haystack.find(&needle);
            let result = match found {
                Some(pos) => haystack_ptr + pos as u64,
                None => 0,
            };
            log_red!(
                emu,
                "msvcrt!strstr '{}' '{}' => 0x{:x}",
                haystack,
                needle,
                result
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = result;
        }
        "strchr" => {
            let s_ptr = emu.regs().rcx;
            let c = emu.regs().rdx as u8 as char;
            let s = emu.maps.read_string(s_ptr);
            let found = s.find(c);
            let result = match found {
                Some(pos) => s_ptr + pos as u64,
                None => 0,
            };
            log_red!(emu, "msvcrt!strchr '{}' '{}' => 0x{:x}", s, c, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = result;
        }
        "strrchr" => {
            let s_ptr = emu.regs().rcx;
            let c = emu.regs().rdx as u8 as char;
            let s = emu.maps.read_string(s_ptr);
            let found = s.rfind(c);
            let result = match found {
                Some(pos) => s_ptr + pos as u64,
                None => 0,
            };
            log_red!(emu, "msvcrt!strrchr '{}' '{}' => 0x{:x}", s, c, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = result;
        }
        "_stricmp" => {
            let s1_ptr = emu.regs().rcx;
            let s2_ptr = emu.regs().rdx;
            let s1 = emu.maps.read_string(s1_ptr).to_lowercase();
            let s2 = emu.maps.read_string(s2_ptr).to_lowercase();
            let result = s1.cmp(&s2) as i32;
            log_red!(emu, "msvcrt!_stricmp '{}' '{}' => {}", s1, s2, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = result as u32 as u64;
        }
        "_strnicmp" => {
            let s1_ptr = emu.regs().rcx;
            let s2_ptr = emu.regs().rdx;
            let n = emu.regs().r8 as usize;
            let s1 = emu.maps.read_string(s1_ptr).to_ascii_lowercase();
            let s2 = emu.maps.read_string(s2_ptr).to_ascii_lowercase();
            let result = s1.get(..n).unwrap_or(&s1).cmp(s2.get(..n).unwrap_or(&s2)) as i32;
            log_red!(
                emu,
                "msvcrt!_strnicmp '{}' '{}' {} => {}",
                s1,
                s2,
                n,
                result
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = result as u64;
        }
        "strlen" => {
            let s_ptr = emu.regs().rcx;
            let s = emu.maps.read_string(s_ptr);
            let len = s.len();
            log_red!(emu, "msvcrt!strlen '{}' => {}", s, len);
            emu.set_last_error(0);
            emu.regs_mut().rax = len as u64;
        }
        "strdup" | "_strdup" => {
            let src_ptr = emu.regs().rcx;
            let s = emu.maps.read_string(src_ptr);
            let len = s.len() as u64 + 1;
            let dst = allocate_memory(emu, len).unwrap_or(0);
            if dst != 0 {
                emu.maps.write_string(dst, &s);
            }
            log_red!(emu, "msvcrt!{} '{}' => 0x{:x}", api, s, dst);
            emu.set_last_error(0);
            emu.regs_mut().rax = dst;
        }
        "_wcsdup" => {
            let src_ptr = emu.regs().rcx;
            let s = emu.maps.read_wide_string(src_ptr);
            let len = (s.len() as u64 + 1) * 2;
            let dst = allocate_memory(emu, len).unwrap_or(0);
            if dst != 0 {
                emu.maps.write_wide_string(dst, &s);
            }
            log_red!(emu, "msvcrt!_wcsdup '{}' => 0x{:x}", s, dst);
            emu.set_last_error(0);
            emu.regs_mut().rax = dst;
        }
        "_strupr" | "_strupr_s" => {
            let s_ptr = emu.regs().rcx;
            let s = emu.maps.read_string(s_ptr);
            let upper = s.to_ascii_uppercase();
            if s_ptr != 0 {
                emu.maps.write_string(s_ptr, &upper);
            }
            log_red!(emu, "msvcrt!{} '{}' => '{}'", api, s, upper);
            emu.set_last_error(0);
            emu.regs_mut().rax = s_ptr;
        }
        "_strlwr" | "_strlwr_s" => {
            let s_ptr = emu.regs().rcx;
            let s = emu.maps.read_string(s_ptr);
            let lower = s.to_ascii_lowercase();
            if s_ptr != 0 {
                emu.maps.write_string(s_ptr, &lower);
            }
            log_red!(emu, "msvcrt!{} '{}' => '{}'", api, s, lower);
            emu.set_last_error(0);
            emu.regs_mut().rax = s_ptr;
        }
        "_wcsupr" | "_wcsupr_s" => {
            let s_ptr = emu.regs().rcx;
            let s = emu.maps.read_wide_string(s_ptr);
            let upper = s.to_uppercase();
            if s_ptr != 0 {
                emu.maps.write_wide_string(s_ptr, &upper);
            }
            log_red!(emu, "msvcrt!{} '{}' => '{}'", api, s, upper);
            emu.set_last_error(0);
            emu.regs_mut().rax = s_ptr;
        }
        "_wcslwr" | "_wcslwr_s" => {
            let s_ptr = emu.regs().rcx;
            let s = emu.maps.read_wide_string(s_ptr);
            let lower = s.to_lowercase();
            if s_ptr != 0 {
                emu.maps.write_wide_string(s_ptr, &lower);
            }
            log_red!(emu, "msvcrt!{} '{}' => '{}'", api, s, lower);
            emu.set_last_error(0);
            emu.regs_mut().rax = s_ptr;
        }
        "atof" | "_atof_l" => {
            let str_ptr = emu.regs().rcx;
            let s = emu.maps.read_string(str_ptr);
            let val = s.trim().parse::<f64>().unwrap_or(0.0);
            log_red!(emu, "msvcrt!{} '{}' => {}", api, s, val);
            emu.fpu_mut().set_st(0, val);
            emu.set_last_error(0);
            emu.regs_mut().rax = val.to_bits();
        }
        "_atoi64" | "_wtoi64" => {
            let str_ptr = emu.regs().rcx;
            let s = if api == "_wtoi64" {
                emu.maps.read_wide_string(str_ptr)
            } else {
                emu.maps.read_string(str_ptr)
            };
            let val = s.trim().parse::<i64>().unwrap_or(0);
            log_red!(emu, "msvcrt!{} '{}' => {}", api, s, val);
            emu.set_last_error(0);
            emu.regs_mut().rax = val as u64;
        }
        "strerror" => {
            let errnum = emu.regs().rcx as u32;
            let msg = match errnum {
                0 => "No error",
                2 => "No such file or directory",
                13 => "Permission denied",
                22 => "Invalid argument",
                _ => "Unknown error",
            };
            let addr = allocate_memory(emu, msg.len() as u64 + 1).unwrap_or(0);
            if addr != 0 {
                emu.maps.write_string(addr, msg);
            }
            log_red!(emu, "msvcrt!strerror {} => '{}'", errnum, msg);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = addr;
        }
        "strtol" => {
            let str_ptr = emu.regs().rcx;
            let _endptr = emu.regs().rdx;
            let base = emu.regs().r8 as i32;
            let s = emu.maps.read_string(str_ptr);
            let val = i64::from_str_radix(s.trim(), base as u32).unwrap_or(0) as u32;
            log_red!(emu, "msvcrt!strtol '{}' base:{} => {}", s, base, val);
            emu.set_last_error(0);
            emu.regs_mut().rax = val as u64;
        }
        "strtoul" => {
            let str_ptr = emu.regs().rcx;
            let _endptr = emu.regs().rdx;
            let base = emu.regs().r8 as u32;
            let s = emu.maps.read_string(str_ptr);
            let val = u64::from_str_radix(s.trim(), base).unwrap_or(0) as u32;
            log_red!(emu, "msvcrt!strtoul '{}' base:{} => {}", s, base, val);
            emu.set_last_error(0);
            emu.regs_mut().rax = val as u64;
        }
        "atol" => {
            let str_ptr = emu.regs().rcx;
            let s = emu.maps.read_string(str_ptr);
            let val = s.trim().parse::<i32>().unwrap_or(0);
            log_red!(emu, "msvcrt!atol '{}' => {}", s, val);
            emu.set_last_error(0);
            emu.regs_mut().rax = val as u32 as u64;
        }
        "_snprintf" | "snprintf" | "sprintf_s" | "_snprintf_s" | "_sprintf_p" => {
            let buf = emu.regs().rcx;
            let _size = emu.regs().rdx;
            let fmt_ptr = emu.regs().r8;
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "msvcrt!{} buf:0x{:x} fmt:'{}'", api, buf, fmt);
            if buf != 0 && !fmt.is_empty() {
                emu.maps.write_string(buf, &fmt);
            }
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "sscanf" | "_sscanf_s_l" | "sscanf_s" => {
            let buf = emu.regs().rcx;
            let fmt_ptr = emu.regs().rdx;
            let s = emu.maps.read_string(buf);
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "msvcrt!{} buf:'{}' fmt:'{}'", api, s, fmt);
            emu.regs_mut().rax = 1;
        }
        "wcsstr" => {
            let haystack_ptr = emu.regs().rcx;
            let needle_ptr = emu.regs().rdx;
            let haystack = emu.maps.read_wide_string(haystack_ptr);
            let needle = emu.maps.read_wide_string(needle_ptr);
            let result = match haystack.find(&needle) {
                Some(pos) => haystack_ptr + (pos * 2) as u64,
                None => 0,
            };
            log_red!(
                emu,
                "msvcrt!wcsstr '{}' '{}' => 0x{:x}",
                haystack,
                needle,
                result
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = result;
        }
        "wcschr" => {
            let s_ptr = emu.regs().rcx;
            let c = emu.regs().rdx as u16;
            let ch = char::from_u32(c as u32).unwrap_or('\0');
            let s = emu.maps.read_wide_string(s_ptr);
            let result = match s.find(ch) {
                Some(pos) => s_ptr + (pos * 2) as u64,
                None => 0,
            };
            log_red!(emu, "msvcrt!wcschr '{}' '{}' => 0x{:x}", s, ch, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = result;
        }
        "wcsrchr" => {
            let s_ptr = emu.regs().rcx;
            let c = emu.regs().rdx as u16;
            let ch = char::from_u32(c as u32).unwrap_or('\0');
            let s = emu.maps.read_wide_string(s_ptr);
            let result = match s.rfind(ch) {
                Some(pos) => s_ptr + (pos * 2) as u64,
                None => 0,
            };
            log_red!(emu, "msvcrt!wcsrchr '{}' '{}' => 0x{:x}", s, ch, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = result;
        }
        "wcsncmp" => {
            let s1_ptr = emu.regs().rcx;
            let s2_ptr = emu.regs().rdx;
            let n = emu.regs().r8 as usize;
            let s1 = emu.maps.read_wide_string(s1_ptr);
            let s2 = emu.maps.read_wide_string(s2_ptr);
            let cmp1: String = s1.chars().take(n).collect();
            let cmp2: String = s2.chars().take(n).collect();
            let result = cmp1.cmp(&cmp2) as i32;
            log_red!(emu, "msvcrt!wcsncmp '{}' '{}' {} => {}", s1, s2, n, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = result as u32 as u64;
        }
        "wcsncpy" => {
            let dst = emu.regs().rcx;
            let src = emu.regs().rdx;
            let n = emu.regs().r8 as usize;
            let s = emu.maps.read_wide_string(src);
            let truncated: String = s.chars().take(n).collect();
            if dst != 0 {
                emu.maps.write_wide_string(dst, &truncated);
            }
            log_red!(
                emu,
                "msvcrt!wcsncpy dst:0x{:x} '{}' n:{}",
                dst,
                truncated,
                n
            );
            emu.regs_mut().rax = dst;
        }
        "wcsncat" => {
            let dst = emu.regs().rcx;
            let src = emu.regs().rdx;
            let n = emu.regs().r8 as usize;
            let dst_s = emu.maps.read_wide_string(dst);
            let src_s = emu.maps.read_wide_string(src);
            let copy_len = src_s.len().min(n);
            let offset = dst + (dst_s.len() * 2) as u64;
            for (i, ch) in src_s.chars().take(copy_len).enumerate() {
                emu.maps.write_word(offset + (i * 2) as u64, ch as u16);
            }
            emu.maps.write_word(offset + (copy_len * 2) as u64, 0);
            log_red!(emu, "msvcrt!wcsncat 0x{:x} '{}' {}", dst, src_s, n);
            emu.regs_mut().rax = dst;
        }
        "_wcsicmp" => {
            let s1_ptr = emu.regs().rcx;
            let s2_ptr = emu.regs().rdx;
            let s1 = emu.maps.read_wide_string(s1_ptr).to_lowercase();
            let s2 = emu.maps.read_wide_string(s2_ptr).to_lowercase();
            let result = s1.cmp(&s2) as i32;
            log_red!(emu, "msvcrt!_wcsicmp '{}' '{}' => {}", s1, s2, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = result as u32 as u64;
        }
        "_wcsnicmp" => {
            let s1_ptr = emu.regs().rcx;
            let s2_ptr = emu.regs().rdx;
            let n = emu.regs().r8 as usize;
            let s1 = emu.maps.read_wide_string(s1_ptr).to_ascii_lowercase();
            let s2 = emu.maps.read_wide_string(s2_ptr).to_ascii_lowercase();
            let result = s1.get(..n).unwrap_or(&s1).cmp(s2.get(..n).unwrap_or(&s2)) as i32;
            log_red!(
                emu,
                "msvcrt!_wcsnicmp '{}' '{}' {} => {}",
                s1,
                s2,
                n,
                result
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = result as u64;
        }
        "_wtoi" => {
            let ptr = emu.regs().rcx;
            let ws = emu.maps.read_wide_string(ptr);
            let val = ws.parse::<i32>().unwrap_or(0);
            log_red!(emu, "msvcrt!_wtoi '{}' => {}", ws, val);
            emu.set_last_error(0);
            emu.regs_mut().rax = val as u64;
        }
        "_wtol" => {
            let ptr = emu.regs().rcx;
            let ws = emu.maps.read_wide_string(ptr);
            let val = ws.parse::<i32>().unwrap_or(0);
            log_red!(emu, "msvcrt!_wtol '{}' => {}", ws, val);
            emu.set_last_error(0);
            emu.regs_mut().rax = val as u64;
        }
        "towupper" => {
            let c = emu.regs().rcx as u32;
            let ch = char::from_u32(c).unwrap_or('\0');
            let upper = ch.to_uppercase().next().unwrap_or(ch);
            let result = upper as u32;
            let slot = allocate_memory(emu, 4).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_dword(slot, result);
            }
            log_red!(emu, "msvcrt!towupper '{}' => '{}'", ch, upper);
            emu.set_last_error(0);
            emu.regs_mut().rax = result as u64;
        }
        "towlower" => {
            let c = emu.regs().rcx as u32;
            let ch = char::from_u32(c).unwrap_or('\0');
            let lower = ch.to_lowercase().next().unwrap_or(ch);
            let result = lower as u32;
            let slot = allocate_memory(emu, 4).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_dword(slot, result);
            }
            log_red!(emu, "msvcrt!towlower '{}' => '{}'", ch, lower);
            emu.set_last_error(0);
            emu.regs_mut().rax = result as u64;
        }
        "mbstowcs" | "_mbstowcs_l" => {
            let dst = emu.regs().rcx;
            let src = emu.regs().rdx;
            let n = emu.regs().r8 as usize;
            let s = emu.maps.read_string(src);
            let copy_len = s.len().min(n);
            for (i, b) in s.as_bytes()[..copy_len].iter().enumerate() {
                emu.maps.write_word(dst + (i * 2) as u64, *b as u16);
            }
            if copy_len < n {
                emu.maps.write_word(dst + (copy_len * 2) as u64, 0);
            }
            log_red!(emu, "msvcrt!{} '{}' => {}", api, s, copy_len);
            emu.set_last_error(0);
            emu.regs_mut().rax = copy_len as u64;
        }
        "wcstombs" | "_wcstombs_l" => {
            let dst = emu.regs().rcx;
            let src = emu.regs().rdx;
            let n = emu.regs().r8 as usize;
            let ws = emu.maps.read_wide_string(src);
            let copy_len = ws.len().min(n);
            for (i, b) in ws.as_bytes()[..copy_len].iter().enumerate() {
                emu.maps.write_byte(dst + i as u64, *b);
            }
            if copy_len < n {
                emu.maps.write_byte(dst + copy_len as u64, 0);
            }
            log_red!(emu, "msvcrt!{} '{}' => {}", api, ws, copy_len);
            emu.set_last_error(0);
            emu.regs_mut().rax = copy_len as u64;
        }
        "isalpha" | "isdigit" | "isspace" | "isalnum" | "ispunct" | "isupper" | "islower"
        | "isxdigit" => {
            let c = emu.regs().rcx as u32;
            let ch = c as u8 as char;
            let r: u64 = match api {
                "isalpha" => ch.is_ascii_alphabetic() as u64,
                "isdigit" => ch.is_ascii_digit() as u64,
                "isspace" => ch.is_ascii_whitespace() as u64,
                "isalnum" => ch.is_ascii_alphanumeric() as u64,
                "ispunct" => ch.is_ascii_punctuation() as u64,
                "isupper" => ch.is_ascii_uppercase() as u64,
                "islower" => ch.is_ascii_lowercase() as u64,
                "isxdigit" => ch.is_ascii_hexdigit() as u64,
                _ => 0,
            };
            let slot = allocate_memory(emu, 4).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_dword(slot, r as u32);
            }
            let name = format!("'{}'", ch);
            log_red!(emu, "msvcrt!{} {} => {}", api, name, r);
            emu.set_last_error(0);
            emu.regs_mut().rax = r;
        }

        // --- Memory functions ---
        "calloc" => {
            let nmemb = emu.regs().rcx;
            let size = emu.regs().rdx;
            let total = nmemb.saturating_mul(size);
            if total == 0 {
                emu.regs_mut().rax = 0;
            } else {
                let base = allocate_memory(emu, total).unwrap_or(0);
                if base != 0 {
                    emu.maps.memset(base, 0, total as usize);
                }
                log_red!(emu, "msvcrt!calloc {}*{} =0x{:x}", nmemb, size, base);
                let first_param = emu.regs().rcx;
                if first_param != 0 && emu.maps.is_mapped(first_param) {
                    let _ = emu.maps.read_dword(first_param).unwrap_or(0);
                }
                emu.regs_mut().rax = base;
            }
        }
        "memset" => {
            let dst = emu.regs().rcx;
            let val = emu.regs().rdx as u8;
            let n = emu.regs().r8 as usize;
            if dst != 0 && n > 0 {
                for i in 0..n {
                    emu.maps.write_byte(dst + i as u64, val);
                }
            }
            log_red!(emu, "msvcrt!memset dst:0x{:x} val:0x{:x} n:{}", dst, val, n);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.regs_mut().rax = dst;
        }
        "memcpy" => {
            let dst = emu.regs().rcx;
            let src = emu.regs().rdx;
            let n = emu.regs().r8 as usize;
            if dst != 0 && src != 0 && n > 0 {
                let mut buf = vec![0u8; n];
                for (i, b) in buf.iter_mut().enumerate() {
                    *b = emu.maps.read_byte(src + i as u64).unwrap_or(0);
                }
                for (i, b) in buf.iter().enumerate() {
                    emu.maps.write_byte(dst + i as u64, *b);
                }
            }
            log_red!(emu, "msvcrt!memcpy dst:0x{:x} src:0x{:x} n:{}", dst, src, n);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.regs_mut().rax = dst;
        }
        "memmove" => {
            let dst = emu.regs().rcx;
            let src = emu.regs().rdx;
            let n = emu.regs().r8 as usize;
            if dst != 0 && src != 0 && n > 0 {
                let mut buf = vec![0u8; n];
                for (i, b) in buf.iter_mut().enumerate() {
                    *b = emu.maps.read_byte(src + i as u64).unwrap_or(0);
                }
                for (i, b) in buf.iter().enumerate() {
                    emu.maps.write_byte(dst + i as u64, *b);
                }
            }
            log_red!(
                emu,
                "msvcrt!memmove dst:0x{:x} src:0x{:x} n:{}",
                dst,
                src,
                n
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.regs_mut().rax = dst;
        }
        "memcmp" => {
            let s1 = emu.regs().rcx;
            let s2 = emu.regs().rdx;
            let n = emu.regs().r8 as usize;
            let mut result: i32 = 0;
            for i in 0..n {
                let b1 = emu.maps.read_byte(s1 + i as u64).unwrap_or(0);
                let b2 = emu.maps.read_byte(s2 + i as u64).unwrap_or(0);
                if b1 != b2 {
                    result = if b1 < b2 { -1 } else { 1 };
                    break;
                }
            }
            log_red!(
                emu,
                "msvcrt!memcmp 0x{:x} 0x{:x} n:{} => {}",
                s1,
                s2,
                n,
                result
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = result as u32 as u64;
        }
        "memchr" => {
            let ptr = emu.regs().rcx;
            let val = emu.regs().rdx as u8;
            let n = emu.regs().r8 as usize;
            let mut found: u64 = 0;
            for i in 0..n {
                if emu.maps.read_byte(ptr + i as u64).unwrap_or(0) == val {
                    found = ptr + i as u64;
                    break;
                }
            }
            log_red!(
                emu,
                "msvcrt!memchr ptr:0x{:x} val:0x{:x} n:{} => 0x{:x}",
                ptr,
                val,
                n,
                found
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = found;
        }
        "_msize" => {
            let ptr = emu.regs().rcx;
            let size = match classify(emu, ptr) {
                AllocKind::Arena { size } | AllocKind::Map { size, .. } => size as u64,
                AllocKind::Invalid => u64::MAX,
            };
            log_red!(emu, "msvcrt!_msize 0x{:x} => {}", ptr, size);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = size;
        }
        "_aligned_malloc" => {
            let size = emu.regs().rcx;
            let _alignment = emu.regs().rdx;
            let base = allocate_memory(emu, size).unwrap_or(0);
            log_red!(emu, "msvcrt!_aligned_malloc {} =0x{:x}", size, base);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = base;
        }
        "_aligned_free" => {
            let addr = emu.regs().rcx;
            log_red!(emu, "msvcrt!_aligned_free 0x{:x}", addr);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            release(emu, addr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }

        // --- I/O functions ---
        "fread" => {
            let buf = emu.regs().rcx;
            let size = emu.regs().rdx as u32;
            let count = emu.regs().r8 as u32;
            let fp = emu.regs().r9;
            let name = helper::handler_get_uri(fp);
            log_red!(
                emu,
                "msvcrt!fread buf:0x{:x} size:{} count:{} fp:0x{:x} '{}'",
                buf,
                size,
                count,
                fp,
                name
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            helper::handler_set_eof(fp);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "fseek" => {
            let fp = emu.regs().rcx;
            let offset = emu.regs().rdx as i32 as i64;
            let origin = emu.regs().r8 as u32;
            let new_pos = helper::handler_seek(fp, offset, origin);
            log_red!(
                emu,
                "msvcrt!fseek fp:0x{:x} offset:{} origin:{} => pos:{}",
                fp,
                offset,
                origin,
                new_pos
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = if new_pos >= 0 { 0 } else { u64::MAX };
        }
        "ftell" => {
            let fp = emu.regs().rcx;
            let pos = helper::handler_tell(fp);
            log_red!(emu, "msvcrt!ftell fp:0x{:x} => {}", fp, pos);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = pos as u64;
        }
        "feof" => {
            let fp = emu.regs().rcx;
            let is_eof = helper::handler_eof(fp);
            log_red!(emu, "msvcrt!feof fp:0x{:x} => {}", fp, is_eof as u32);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = is_eof as u64;
        }
        "fgets" => {
            let buf = emu.regs().rcx;
            let n = emu.regs().rdx as u32;
            let stream = emu.regs().r8;
            let name = helper::handler_get_uri(stream);
            log_red!(
                emu,
                "msvcrt!fgets buf:0x{:x} n:{} stream:0x{:x} '{}'",
                buf,
                n,
                stream,
                name
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            if buf != 0 && n > 0 {
                emu.maps.write_byte(buf, b'\n');
                if n > 1 {
                    emu.maps.write_byte(buf + 1, 0);
                }
                emu.regs_mut().rax = buf;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "fprintf" => {
            let _stream = emu.regs().rcx;
            let fmt_ptr = emu.regs().rdx;
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "msvcrt!fprintf fmt:'{}'", fmt);
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "vprintf" => {
            let fmt_ptr = emu.regs().rcx;
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "msvcrt!vprintf fmt:'{}'", fmt);
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "vsprintf" | "_vsnprintf" | "vsnprintf" => {
            let buf = emu.regs().rcx;
            let fmt_ptr = emu.regs().rdx;
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "msvcrt!{} buf:0x{:x} fmt:'{}'", api, buf, fmt);
            if buf != 0 && !fmt.is_empty() {
                emu.maps.write_string(buf, &fmt);
            }
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "vfprintf" => {
            let _stream = emu.regs().rcx;
            let fmt_ptr = emu.regs().rdx;
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "msvcrt!vfprintf fmt:'{}'", fmt);
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "fputs" => {
            let s_ptr = emu.regs().rcx;
            let s = emu.maps.read_string(s_ptr);
            log_red!(emu, "msvcrt!fputs '{}'", s);
            emu.regs_mut().rax = 0;
        }
        "puts" => {
            let s_ptr = emu.regs().rcx;
            let s = emu.maps.read_string(s_ptr);
            log_red!(emu, "msvcrt!puts '{}'", s);
            emu.regs_mut().rax = 0;
        }
        "putchar" | "putc" | "_putch" => {
            let c = emu.regs().rcx as u32;
            let ch_str = format!("{}", c as u8 as char);
            let slot = allocate_memory(emu, 1).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_byte(slot, c as u8);
            }
            log_red!(emu, "msvcrt!{} '{}'", api, ch_str);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = c as u64;
        }
        "getc" | "fgetc" | "_fgetchar" | "getchar" => {
            let stream = emu.regs().rcx;
            let name = helper::handler_get_uri(stream);
            log_red!(
                emu,
                "msvcrt!{} stream:0x{:x} '{}' => EOF",
                api,
                stream,
                name
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0xffff_ffff;
        }
        "_fileno" => {
            let stream = emu.regs().rcx;
            let fd = if helper::handler_exist(stream) {
                stream & 0xffff
            } else {
                0
            };
            log_red!(emu, "msvcrt!_fileno stream:0x{:x} => {}", stream, fd);
            emu.set_last_error(0);
            emu.regs_mut().rax = fd;
        }
        "_setmode" => {
            let fd = emu.regs().rcx;
            let mode = emu.regs().rdx;
            let prev_mode = 0x4000u64; // _O_TEXT
            let name = helper::handler_get_uri(fd);
            log_red!(
                emu,
                "msvcrt!_setmode fd:{} '{}' mode:0x{:x} => 0x{:x}",
                fd,
                name,
                mode,
                prev_mode
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = prev_mode;
        }
        "_open" | "_wopen" => {
            let path_ptr = emu.regs().rcx;
            let path = emu.maps.read_string(path_ptr);
            log_red!(emu, "msvcrt!{} '{}'", api, path);
            emu.regs_mut().rax = helper::handler_create(&path);
        }
        "_close" => {
            let fd = emu.regs().rcx;
            let name = helper::handler_get_uri(fd);
            let ok = helper::handler_close(fd);
            log_red!(emu, "msvcrt!_close fd:{} '{}' ok:{}", fd, name, ok);
            emu.regs_mut().rax = if ok { 0 } else { u64::MAX };
        }
        "_read" => {
            let fd = emu.regs().rcx;
            let buf = emu.regs().rdx;
            let count = emu.regs().r8 as u32;
            let name = helper::handler_get_uri(fd);
            log_red!(
                emu,
                "msvcrt!_read fd:{} '{}' buf:0x{:x} count:{}",
                fd,
                name,
                buf,
                count
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "_write" => {
            let fd = emu.regs().rcx;
            let buf = emu.regs().rdx;
            let count = emu.regs().r8 as u32;
            let name = helper::handler_get_uri(fd);
            log_red!(
                emu,
                "msvcrt!_write fd:{} '{}' buf:0x{:x} count:{}",
                fd,
                name,
                buf,
                count
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = count as u64;
        }

        // --- Process/Environment ---
        "_cexit" | "_c_exit" => {
            log_red!(emu, "msvcrt!{} cleaning up", api);
            let slot = allocate_memory(emu, 4).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_dword(slot, 1); // cleanup flag
            }
            helper::advance_tick(emu, 1);
        }
        "_beginthread" => {
            let start = emu.regs().rcx;
            log_red!(emu, "msvcrt!_beginthread start:0x{:x}", start);
            let hndl = helper::handler_create("msvcrt://_beginthread");
            emu.regs_mut().rax = hndl;
        }
        "_beginthreadex" => {
            let _security = emu.regs().rcx;
            let _stack_size = emu.regs().rdx;
            let start = emu.regs().r8;
            log_red!(emu, "msvcrt!_beginthreadex start:0x{:x}", start);
            let hndl = helper::handler_create("msvcrt://_beginthreadex");
            emu.regs_mut().rax = hndl;
        }
        "_endthread" | "_endthreadex" => {
            log_red!(emu, "msvcrt!{}", api);
            let slot = allocate_memory(emu, 4).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_dword(slot, 0); // exit code
            }
            emu.stop();
        }
        "system" => {
            let cmd_ptr = emu.regs().rcx;
            let cmd = emu.maps.read_string(cmd_ptr);
            log_red!(emu, "msvcrt!system '{}'", cmd);
            emu.regs_mut().rax = 0;
        }
        "_putenv" | "_wputenv" | "_putenv_s" => {
            let ptr = emu.regs().rcx;
            let s = emu.maps.read_string(ptr);
            log_red!(emu, "msvcrt!{} '{}'", api, s);
            emu.regs_mut().rax = 0;
        }
        "__wgetmainargs" => {
            let p_argc = emu.regs().rcx;
            let p_argv = emu.regs().rdx;
            let p_env = emu.regs().r8;
            log_red!(emu, "msvcrt!__wgetmainargs");
            if p_argc != 0 {
                emu.maps.write_qword(p_argc, 1);
            }
            if p_argv != 0 {
                emu.maps.write_qword(p_argv, 0);
            }
            if p_env != 0 {
                emu.maps.write_qword(p_env, 0);
            }
            emu.regs_mut().rax = 0;
        }
        "_wcmdln" | "__p__wcmdln" => {
            let addr = allocate_memory(emu, 64).unwrap_or(0);
            if addr != 0 {
                emu.maps.write_wide_string(addr, "mwemu.exe");
            }
            log_red!(emu, "msvcrt!{} => 0x{:x}", api, addr);
            emu.set_last_error(0);
            emu.regs_mut().rax = addr;
        }

        // --- Exception handling ---
        "_except_handler4"
        | "_except_handler4_common"
        | "__CxxFrameHandler3"
        | "__CxxFrameHandler4" => {
            let exc_record = emu.regs().rcx;
            let frame_ptr = emu.regs().rdx;
            let exc_code = if exc_record != 0 {
                emu.maps.read_dword(exc_record).unwrap_or(0)
            } else {
                0
            };
            log_red!(
                emu,
                "msvcrt!{} exc:0x{:x} code:0x{:x} frame:0x{:x}",
                api,
                exc_record,
                exc_code,
                frame_ptr
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_set_se_translator" | "_set_new_handler" | "_set_new_mode" | "set_terminate"
        | "set_unexpected" => {
            let handler = emu.regs().rcx;
            log_red!(emu, "msvcrt!{} handler:0x{:x}", api, handler);
            let slot = allocate_memory(emu, 8).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_qword(slot, handler);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "_EH_prolog3" | "_EH_prolog3_catch" | "_EH_prolog3_GS" => {
            let slot = allocate_memory(emu, 8).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_qword(slot, emu.regs().rbp);
            }
            log_red!(emu, "msvcrt!{} (64-bit)", api);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }

        // --- Math functions ---
        "_ftol2" | "_ftol2_sse" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val as i64;
            emu.fpu_mut().set_st(0, result as f64);
            log_red!(emu, "msvcrt!{} {} => {}", api, val, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = result as u64;
        }
        "abs" | "labs" => {
            let val = emu.regs().rcx as i32;
            let result = val.unsigned_abs();
            let slot = allocate_memory(emu, 4).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_dword(slot, result);
            }
            log_red!(emu, "msvcrt!{} {} => {}", api, val, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = result as u64;
        }
        "ceil" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val.ceil();
            emu.fpu_mut().set_st(0, result);
            log_red!(emu, "msvcrt!ceil {} => {}", val, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "floor" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val.floor();
            emu.fpu_mut().set_st(0, result);
            log_red!(emu, "msvcrt!floor {} => {}", val, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "sqrt" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val.sqrt();
            emu.fpu_mut().set_st(0, result);
            log_red!(emu, "msvcrt!sqrt {} => {}", val, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "pow" => {
            let base_val = f64::from_bits(emu.regs().rcx);
            let exp_val = f64::from_bits(emu.regs().rdx);
            let result = base_val.powf(exp_val);
            log_red!(emu, "msvcrt!pow {} {} => {}", base_val, exp_val, result);
            emu.fpu_mut().set_st(0, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "log" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val.ln();
            emu.fpu_mut().set_st(0, result);
            log_red!(emu, "msvcrt!log {} => {}", val, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "log10" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val.log10();
            emu.fpu_mut().set_st(0, result);
            log_red!(emu, "msvcrt!log10 {} => {}", val, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "exp" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val.exp();
            emu.fpu_mut().set_st(0, result);
            log_red!(emu, "msvcrt!exp {} => {}", val, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "sin" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val.sin();
            emu.fpu_mut().set_st(0, result);
            log_red!(emu, "msvcrt!sin {} => {}", val, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "cos" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val.cos();
            emu.fpu_mut().set_st(0, result);
            log_red!(emu, "msvcrt!cos {} => {}", val, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "tan" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val.tan();
            emu.fpu_mut().set_st(0, result);
            log_red!(emu, "msvcrt!tan {} => {}", val, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "atan" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val.atan();
            emu.fpu_mut().set_st(0, result);
            log_red!(emu, "msvcrt!atan {} => {}", val, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "atan2" => {
            let y = f64::from_bits(emu.regs().rcx);
            let x = f64::from_bits(emu.regs().rdx);
            let result = y.atan2(x);
            log_red!(emu, "msvcrt!atan2 {} {} => {}", y, x, result);
            emu.fpu_mut().set_st(0, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "fabs" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val.abs();
            emu.fpu_mut().set_st(0, result);
            log_red!(emu, "msvcrt!fabs {} => {}", val, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "fmod" => {
            let x = f64::from_bits(emu.regs().rcx);
            let y = f64::from_bits(emu.regs().rdx);
            let result = x % y;
            log_red!(emu, "msvcrt!fmod {} {} => {}", x, y, result);
            emu.fpu_mut().set_st(0, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }

        // --- Misc ---
        "clock" => {
            let tick_val = emu.tick as u64;
            let slot = allocate_memory(emu, 8).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_qword(slot, tick_val);
            }
            log_red!(emu, "msvcrt!clock => {}", tick_val);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = tick_val;
        }
        "_time64" => {
            let t_ptr = emu.regs().rcx;
            let now = 0x6718_0000u64;
            if t_ptr != 0 {
                emu.maps.write_qword(t_ptr, now);
            }
            log_red!(emu, "msvcrt!_time64 => 0x{:x}", now);
            emu.set_last_error(0);
            emu.regs_mut().rax = now;
        }
        "_controlfp_s" => {
            let p_current = emu.regs().rcx;
            if p_current != 0 {
                emu.maps.write_dword(p_current, 0x0008_001f);
            }
            log_red!(emu, "msvcrt!_controlfp_s");
            emu.regs_mut().rax = 0;
        }
        "_unlock" => {
            let lock_num = emu.regs().rcx as u32;
            let slot = allocate_memory(emu, 4).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_dword(slot, lock_num);
            }
            log_red!(emu, "msvcrt!_unlock {}", lock_num);
            helper::advance_tick(emu, 1);
        }
        "_configthreadlocale" => {
            let per_thread = emu.regs().rcx as i32;
            log_red!(emu, "msvcrt!_configthreadlocale type:{}", per_thread);
            let slot = allocate_memory(emu, 4).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_dword(slot, per_thread as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // _ENABLE_PER_THREAD_LOCALE_OLD = 0
        }
        "_amsg_exit" => {
            let code = emu.regs().rcx as u32;
            log_red!(emu, "msvcrt!_amsg_exit code:{}", code);
            let slot = allocate_memory(emu, 4).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_dword(slot, code);
            }
            emu.stop();
        }
        "_crt_atexit" | "_register_onexit_function" => {
            let func_ptr = emu.regs().rcx;
            log_red!(emu, "msvcrt!{} func:0x{:x}", api, func_ptr);
            let slot = allocate_memory(emu, 8).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_qword(slot, func_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "_initialize_onexit_table" => {
            let table_ptr = emu.regs().rcx;
            if table_ptr != 0 {
                emu.maps.write_qword(table_ptr, 0);
                emu.maps.write_qword(table_ptr + 8, 0);
                emu.maps.write_qword(table_ptr + 16, 0);
            }
            log_red!(emu, "msvcrt!_initialize_onexit_table tbl:0x{:x}", table_ptr);
            emu.regs_mut().rax = 0;
        }
        "__CppXcptFilter" => {
            let exc_num = emu.regs().rcx;
            let exc_ptrs = emu.regs().rdx;
            if exc_ptrs != 0 {
                let exc_record = emu.maps.read_qword(exc_ptrs).unwrap_or(0);
                log_red!(
                    emu,
                    "msvcrt!__CppXcptFilter exc:0x{:x} ptrs:0x{:x} rec:0x{:x}",
                    exc_num,
                    exc_ptrs,
                    exc_record
                );
                let first_param = emu.regs().rcx;
                if first_param != 0 && emu.maps.is_mapped(first_param) {
                    let _ = emu.maps.read_dword(first_param).unwrap_or(0);
                }
            } else {
                log_red!(emu, "msvcrt!__CppXcptFilter exc:0x{:x} ptrs:NULL", exc_num);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1; // EXCEPTION_EXECUTE_HANDLER
        }
        "__acrt_iob_func" => {
            let index = emu.regs().rcx as u32;
            let stream_name = match index {
                0 => "stdin",
                1 => "stdout",
                2 => "stderr",
                _ => "unknown_stream",
            };
            let hndl = helper::handler_create(stream_name);
            log_red!(
                emu,
                "msvcrt!__acrt_iob_func index:{} => 0x{:x}",
                index,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "__iob_func" => {
            let iob_addr = allocate_memory(emu, 96).unwrap_or(0);
            log_red!(emu, "msvcrt!__iob_func => 0x{:x}", iob_addr);
            emu.set_last_error(0);
            emu.regs_mut().rax = iob_addr;
        }
        "__p__environ" => {
            let env_addr = allocate_memory(emu, 16).unwrap_or(0);
            if env_addr != 0 {
                emu.maps.write_qword(env_addr, 0);
            }
            log_red!(emu, "msvcrt!__p__environ => 0x{:x}", env_addr);
            emu.set_last_error(0);
            emu.regs_mut().rax = env_addr;
        }
        "__stdio_common_vfprintf" => {
            let _options = emu.regs().rcx;
            let _stream = emu.regs().rdx;
            let fmt_ptr = emu.regs().r8;
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "msvcrt!__stdio_common_vfprintf fmt:'{}'", fmt);
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "__stdio_common_vsprintf_s" => {
            let _options = emu.regs().rcx;
            let buf = emu.regs().rdx;
            let _buf_size = emu.regs().r8;
            let fmt_ptr = emu.regs().r9;
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(
                emu,
                "msvcrt!__stdio_common_vsprintf_s buf:0x{:x} fmt:'{}'",
                buf,
                fmt
            );
            if buf != 0 && !fmt.is_empty() {
                emu.maps.write_string(buf, &fmt);
            }
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "_configure_narrow_argv" => {
            let mode = emu.regs().rcx;
            log_red!(emu, "msvcrt!_configure_narrow_argv mode:{}", mode);
            let slot = allocate_memory(emu, 8).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_qword(slot, mode);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // success
        }
        "_set_errno" => {
            let err_value = emu.regs().rcx as u32;
            log_red!(emu, "msvcrt!_set_errno value:{}", err_value);
            let slot = allocate_memory(emu, 4).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_dword(slot, err_value);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // success
        }
        "_get_errno" => {
            let out_ptr = emu.regs().rcx;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            log_red!(emu, "msvcrt!_get_errno => 0");
            emu.regs_mut().rax = 0;
        }
        "_get_initial_narrow_environment" => {
            let env_addr = allocate_memory(emu, 16).unwrap_or(0);
            if env_addr != 0 {
                emu.maps.write_qword(env_addr, 0);
            }
            log_red!(
                emu,
                "msvcrt!_get_initial_narrow_environment => 0x{:x}",
                env_addr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = env_addr;
        }
        "_initialize_narrow_environment" => {
            log_red!(emu, "msvcrt!_initialize_narrow_environment");
            let env_addr = allocate_memory(emu, 16).unwrap_or(0);
            if env_addr != 0 {
                emu.maps.write_qword(env_addr, 0); // empty environment
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // success
        }
        "_localtime64" => {
            let time_ptr = emu.regs().rcx;
            let _time_val = if time_ptr != 0 {
                emu.maps.read_qword(time_ptr).unwrap_or(0)
            } else {
                0
            };
            let tm_addr = emu.maps.alloc(36).unwrap_or_default();
            if tm_addr != 0
                && emu
                    .maps
                    .create_map(
                        "tm_struct",
                        tm_addr,
                        36,
                        crate::maps::mem64::Permission::from_flags(true, true, false),
                    )
                    .is_ok()
            {
                emu.maps.write_dword(tm_addr, 0); // tm_sec
                emu.maps.write_dword(tm_addr + 4, 30); // tm_min
                emu.maps.write_dword(tm_addr + 8, 12); // tm_hour
                emu.maps.write_dword(tm_addr + 12, 1); // tm_mday
                emu.maps.write_dword(tm_addr + 16, 0); // tm_mon
                emu.maps.write_dword(tm_addr + 20, 126); // tm_year (2026)
                emu.maps.write_dword(tm_addr + 24, 4); // tm_wday
                emu.maps.write_dword(tm_addr + 28, 0); // tm_yday
                emu.maps.write_dword(tm_addr + 32, 0); // tm_isdst
            }
            log_red!(emu, "msvcrt!_localtime64 => 0x{:x}", tm_addr);
            emu.set_last_error(0);
            emu.regs_mut().rax = tm_addr;
        }
        "_set_app_type" => {
            let app_type = emu.regs().rcx;
            log_red!(emu, "msvcrt!_set_app_type type:{}", app_type);
            let slot = allocate_memory(emu, 8).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_qword(slot, app_type);
            }
        }
        "_set_invalid_parameter_handler" => {
            let handler = emu.regs().rcx;
            log_red!(
                emu,
                "msvcrt!_set_invalid_parameter_handler handler:0x{:x}",
                handler
            );
            let slot = allocate_memory(emu, 8).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_qword(slot, handler);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // previous handler (NULL)
        }
        "_set_thread_local_invalid_parameter_handler" => {
            let handler = emu.regs().rcx;
            log_red!(
                emu,
                "msvcrt!_set_thread_local_invalid_parameter_handler handler:0x{:x}",
                handler
            );
            let slot = allocate_memory(emu, 8).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_qword(slot, handler);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // previous handler (NULL)
        }
        "_strcmpi" => {
            let s1_ptr = emu.regs().rcx;
            let s2_ptr = emu.regs().rdx;
            let s1 = emu.maps.read_string(s1_ptr).to_lowercase();
            let s2 = emu.maps.read_string(s2_ptr).to_lowercase();
            let result = s1.cmp(&s2) as i32;
            log_red!(emu, "msvcrt!_strcmpi '{}' '{}' => {}", s1, s2, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = result as u64;
        }
        "_time32" => {
            let timer_ptr = emu.regs().rcx;
            let time_val = emu.tick as u32;
            if timer_ptr != 0 {
                emu.maps.write_dword(timer_ptr, time_val);
            }
            log_red!(emu, "msvcrt!_time32 => {}", time_val);
            emu.set_last_error(0);
            emu.regs_mut().rax = time_val as u64;
        }
        "_wfopen" => {
            let fname_ptr = emu.regs().rcx;
            let mode_ptr = emu.regs().rdx;
            let fname = emu.maps.read_wide_string(fname_ptr);
            let mode = emu.maps.read_wide_string(mode_ptr);
            let hndl = helper::handler_create(&fname);
            log_red!(
                emu,
                "msvcrt!_wfopen '{}' mode:'{}' => 0x{:x}",
                fname,
                mode,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "abort" => {
            log_red!(emu, "msvcrt!abort");
            let slot = allocate_memory(emu, 4).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_dword(slot, 3); // SIGABRT
            }
            emu.stop();
        }
        "ptr" => {
            let p = emu.regs().rcx;
            let val = if p != 0 {
                emu.maps.read_qword(p).unwrap_or(0)
            } else {
                0
            };
            log_red!(emu, "msvcrt!ptr 0x{:x} => [0x{:x}]", p, val);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = p;
        }
        "setvbuf" => {
            let stream = emu.regs().rcx;
            let _buffer = emu.regs().rdx;
            let mode = emu.regs().r8 as u32;
            let size = emu.regs().r9 as u32;
            let uri = helper::handler_get_uri(stream);
            log_red!(
                emu,
                "msvcrt!setvbuf stream:0x{:x}('{}') mode:{} size:{}",
                stream,
                uri,
                mode,
                size
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "signal" => {
            let signum = emu.regs().rcx as u32;
            let handler = emu.regs().rdx;
            log_red!(emu, "msvcrt!signal sig:{} handler:0x{:x}", signum, handler);
            let slot = allocate_memory(emu, 16).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_dword(slot, signum);
                emu.maps.write_qword(slot + 8, handler);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // SIG_DFL (previous handler)
        }
        "str" => {
            let s_ptr = emu.regs().rcx;
            let s = emu.maps.read_string(s_ptr);
            log_red!(emu, "msvcrt!str 0x{:x} '{}'", s_ptr, s);
            emu.regs_mut().rax = s_ptr;
        }
        "strtok_s" => {
            let str_ptr = emu.regs().rcx;
            let delim_ptr = emu.regs().rdx;
            let context_ptr = emu.regs().r8;
            let delim = emu.maps.read_string(delim_ptr);
            let s = emu.maps.read_string(str_ptr);
            if let Some(pos) = s.find(|c: char| delim.contains(c)) {
                emu.maps.write_byte(str_ptr + pos as u64, 0);
                if context_ptr != 0 {
                    emu.maps.write_qword(context_ptr, str_ptr + pos as u64 + 1);
                }
                log_red!(emu, "msvcrt!strtok_s '{}' delim:'{}' => token", s, delim);
                emu.regs_mut().rax = str_ptr;
            } else {
                log_red!(emu, "msvcrt!strtok_s '{}' delim:'{}' => null", s, delim);
                emu.set_last_error(0);
                emu.regs_mut().rax = 0;
            }
        }
        "vwprintf" => {
            let fmt_ptr = emu.regs().rcx;
            let fmt = emu.maps.read_wide_string(fmt_ptr);
            log_red!(emu, "msvcrt!vwprintf fmt:'{}'", fmt);
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "wcsftime" => {
            let wcs = emu.regs().rcx;
            let _maxsize = emu.regs().rdx;
            let fmt_ptr = emu.regs().r8;
            let _timeptr = emu.regs().r9;
            let fmt = emu.maps.read_wide_string(fmt_ptr);
            if wcs != 0 && !fmt.is_empty() {
                emu.maps.write_wide_string(wcs, &fmt);
            }
            log_red!(emu, "msvcrt!wcsftime fmt:'{}' => {}", fmt, fmt.len());
            emu.set_last_error(0);
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "__lconv_init" => {
            let addr = allocate_memory(emu, 64).unwrap_or(0);
            log_red!(emu, "msvcrt!__lconv_init => 0x{:x}", addr);
            emu.set_last_error(0);
            emu.regs_mut().rax = addr;
        }
        "__setlc_active" | "__unguarded_readlc_active" => {
            let addr = allocate_memory(emu, 4).unwrap_or(0);
            if addr != 0 {
                emu.maps.write_dword(addr, 0);
            }
            log_red!(emu, "msvcrt!{} => 0x{:x}", api, addr);
            emu.set_last_error(0);
            emu.regs_mut().rax = addr;
        }
        "_set_error_mode" => {
            let mode = emu.regs().rcx as u32;
            log_red!(emu, "msvcrt!_set_error_mode mode:{}", mode);
            let slot = allocate_memory(emu, 4).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_dword(slot, mode);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // previous mode (_OUT_TO_DEFAULT)
        }
        "_set_fmode" => {
            let mode = emu.regs().rcx as u32;
            log_red!(emu, "msvcrt!_set_fmode mode:0x{:x}", mode);
            let slot = allocate_memory(emu, 4).unwrap_or(0);
            if slot != 0 {
                emu.maps.write_dword(slot, mode);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // success
        }
        "__stdio_common_vsprintf" => {
            let _options = emu.regs().rcx;
            let buf = emu.regs().rdx;
            let _buf_size = emu.regs().r8;
            let fmt_ptr = emu.regs().r9;
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(
                emu,
                "msvcrt!__stdio_common_vsprintf buf:0x{:x} fmt:'{}'",
                buf,
                fmt
            );
            if buf != 0 && !fmt.is_empty() {
                emu.maps.write_string(buf, &fmt);
            }
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "__stdio_common_vsscanf" => {
            let _options = emu.regs().rcx;
            let buf = emu.regs().rdx;
            let _buf_size = emu.regs().r8;
            let fmt_ptr = emu.regs().r9;
            let s = emu.maps.read_string(buf);
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(
                emu,
                "msvcrt!__stdio_common_vsscanf buf:'{}' fmt:'{}'",
                s,
                fmt
            );
            emu.regs_mut().rax = 1;
        }
        "_sprintf_l" => {
            let buf = emu.regs().rcx;
            let fmt_ptr = emu.regs().rdx;
            let _locale = emu.regs().r8;
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "msvcrt!_sprintf_l buf:0x{:x} fmt:'{}'", buf, fmt);
            if buf != 0 && !fmt.is_empty() {
                emu.maps.write_string(buf, &fmt);
            }
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "wprintf" => {
            let fmt_ptr = emu.regs().rcx;
            let fmt = emu.maps.read_wide_string(fmt_ptr);
            log_red!(emu, "msvcrt!wprintf fmt:'{}'", fmt);
            emu.regs_mut().rax = fmt.len() as u64;
        }

        _ => {
            let rip = emu.regs().rip;
            if emu.cfg.skip_unimplemented {
                let nparams = emu.banzai.get_params(api);
                log::warn!(
                    "banzai: skipping unimplemented API {} (nparams={}) at 0x{:x}",
                    api,
                    nparams,
                    rip
                );
                emu.regs_mut().rax = 0;
                return String::new();
            }

            if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                serialization::Serialization::dump(
                    emu,
                    emu.cfg.dump_filename.as_deref().unwrap_or("mwemu_dump.bin"),
                );
            }

            log::error!("attempt to call unimplemented API {}", api);
            log::warn!("calling unimplemented API {} at 0x{:x}", api, rip);
            return api.to_ascii_lowercase();
        }
    }

    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_sections_match_exact_loader_names() {
        assert!(is_native_section("msvcrt.text"));
        assert!(is_native_section("msvcrtfothk"));
        assert!(!is_native_section("msvcrt.rdata"));
        assert!(!is_native_section("msvcrt.data"));
        assert!(!is_native_section(""));
        assert!(!is_native_section("MSVCRT.TEXT"));
        assert!(!is_native_section("msvcrt.text "));
        assert!(!is_native_section("kernel32.text"));
    }
}
