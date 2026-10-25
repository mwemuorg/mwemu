use crate::emu;
use crate::kernel::heap::Region;
use crate::maps::mem64::Permission;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi32::kernel32;
//use crate::endpoint;

const LARGE_ALLOC_THRESHOLD: u64 = 0x8000;

/// Allocates from the O1Heap arena for small sizes, maps a dedicated
/// region otherwise (same threshold as kernel32!HeapAlloc).
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

/// Classification of a pointer allocated by `allocate_memory`.
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

/// Releases a pointer allocated by `allocate_memory` (or any alloc_ map).
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

// msvcrt is an exception and these functions dont have to compensate the stack.

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "_initterm_e" => _initterm_e(emu),
        "_initterm" => _initterm(emu),
        "StrCmpCA" => StrCmpCA(emu),
        "fopen" => fopen(emu),
        "fwrite" => fwrite(emu),
        "fflush" => fflush(emu),
        "fclose" => fclose(emu),
        "__p___argv" => __p___argv(emu),
        "__p___argc" => __p___argc(emu),
        "malloc" => malloc(emu),
        "_onexit" => _onexit(emu),
        "_lock" => _lock(emu),
        "free" => free(emu),
        "realloc" => realloc(emu),
        "strtok" => strtok(emu),
        "exit" => exit(emu),
        "__set_app_type" => __set_app_type(emu),
        "_errno" => _errno(emu),
        "__p__fmode" => __p__global_int(emu, "__p__fmode"),
        "__p__commode" => __p__global_int(emu, "__p__commode"),
        "_adjust_fdiv" => __p__global_int(emu, "_adjust_fdiv"),
        "__setusermatherr" => {
            log_red!(emu, "msvcrt!__setusermatherr");
        }
        "_controlfp" => {
            let new_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let mask = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "msvcrt!_controlfp 0x{:x} 0x{:x}", new_val, mask);
            emu.regs_mut().rax = 0x0008_001f;
        }
        "__getmainargs" => {
            let p_argc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p_argv = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p_env = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "msvcrt!__getmainargs");
            if p_argc != 0 {
                emu.maps.write_dword(p_argc, 1);
            }
            if p_argv != 0 {
                emu.maps.write_dword(p_argv, 0);
            }
            if p_env != 0 {
                emu.maps.write_dword(p_env, 0);
            }
            emu.regs_mut().rax = 0;
        }
        "_acmdln" | "__p__acmdln" => __p__acmdln(emu),
        "_XcptFilter" => {
            log_red!(emu, "msvcrt!_XcptFilter");
            emu.regs_mut().rax = 0;
        }
        "_exit" => exit(emu),
        "__dllonexit" => {
            let func = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "msvcrt!__dllonexit 0x{:x}", func);
            emu.regs_mut().rax = func as u64;
        }
        "_except_handler3" | "__CxxFrameHandler" => {
            log_red!(emu, "msvcrt!{}", api);
            emu.regs_mut().rax = 0;
        }
        "_EH_prolog" => {
            log_red!(emu, "msvcrt!_EH_prolog");
        }
        "_CxxThrowException" => {
            log_red!(emu, "msvcrt!_CxxThrowException");
        }
        "_ftol" => {
            let val = emu.fpu_mut().get_st(0);
            log_red!(emu, "msvcrt!_ftol {} => {}", val, val as i64);
            emu.regs_mut().rax = val as i64 as u64;
        }
        "time" => {
            let t_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let now = 0x6718_0000u64;
            if t_ptr != 0 {
                emu.maps.write_dword(t_ptr, now as u32);
            }
            log_red!(emu, "msvcrt!time => 0x{:x}", now);
            emu.regs_mut().rax = now;
        }
        "localtime" => {
            let tm_addr = allocate_memory(emu, 64).unwrap_or(0);
            log_red!(emu, "msvcrt!localtime => 0x{:x}", tm_addr);
            emu.regs_mut().rax = tm_addr;
        }
        "strftime" => {
            let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let max_size = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let s = "01/01/2024 00:00:00";
            if dst != 0 && max_size > 0 {
                emu.maps.write_string(dst, s);
            }
            log_red!(emu, "msvcrt!strftime => '{}'", s);
            emu.regs_mut().rax = s.len() as u64;
        }
        "srand" => {
            log_red!(emu, "msvcrt!srand");
        }
        "rand" => {
            log_red!(emu, "msvcrt!rand");
            emu.regs_mut().rax = 42;
        }
        "atoi" => {
            let ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = emu.maps.read_string(ptr);
            let val = s.parse::<i32>().unwrap_or(0);
            log_red!(emu, "msvcrt!atoi '{}' => {}", s, val);
            emu.regs_mut().rax = val as u64;
        }
        "_itoa" | "_itow" => {
            let val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let buf = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let radix = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(10);
            let s = match radix {
                16 => format!("{:x}", val),
                8 => format!("{:o}", val),
                _ => format!("{}", val),
            };
            if buf != 0 {
                emu.maps.write_string(buf, &s);
            }
            log_red!(emu, "msvcrt!{} {} => '{}'", api, val, s);
            emu.regs_mut().rax = buf;
        }
        "getenv" | "_wgetenv" => {
            let ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name = emu.maps.read_string(ptr);
            log_red!(emu, "msvcrt!{} '{}' => NULL", api, name);
            emu.regs_mut().rax = 0;
        }
        "printf" | "sprintf" | "swprintf" => {
            log_red!(emu, "msvcrt!{}", api);
            emu.regs_mut().rax = 0;
        }
        "strncmp" => {
            let s1_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s2_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let n = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
            let s1 = emu.maps.read_string(s1_ptr);
            let s2 = emu.maps.read_string(s2_ptr);
            let result = s1.get(..n).unwrap_or(&s1).cmp(s2.get(..n).unwrap_or(&s2)) as i32;
            log_red!(emu, "msvcrt!strncmp '{}' '{}' {} => {}", s1, s2, n, result);
            emu.regs_mut().rax = result as u64;
        }
        "toupper" => {
            let c = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let upper = (c as u8 as char).to_ascii_uppercase() as u64;
            emu.regs_mut().rax = upper;
        }
        "tolower" => {
            let c = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let lower = (c as u8 as char).to_ascii_lowercase() as u64;
            emu.regs_mut().rax = lower;
        }
        "wcslen" => {
            let ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let ws = emu.maps.read_wide_string(ptr);
            log_red!(emu, "msvcrt!wcslen '{}' => {}", ws, ws.len());
            emu.regs_mut().rax = ws.len() as u64;
        }
        "wcscpy" | "wcscat" | "wcscmp" => {
            log_red!(emu, "msvcrt!{}", api);
            emu.regs_mut().rax = 0;
        }
        "freopen" | "_wrename" | "_wsystem" => {
            log_red!(emu, "msvcrt!{}", api);
            emu.regs_mut().rax = 0;
        }
        "_iob" => {
            let iob_addr = allocate_memory(emu, 96).unwrap_or(0);
            log_red!(emu, "msvcrt!_iob => 0x{:x}", iob_addr);
            emu.regs_mut().rax = iob_addr;
        }
        "??1type_info@@UAE@XZ" | "??0exception@@QAE@ABV0@@Z" | "??2@YAPAXI@Z" | "??3@YAXPAX@Z" => {
            log_red!(emu, "msvcrt!{}", api);
            emu.regs_mut().rax = 0;
        }
        _ => {
            if !emu.cfg.skip_unimplemented {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump(
                        emu,
                        emu.cfg.dump_filename.as_deref().unwrap_or("mwemu_dump.bin"),
                    );
                }

                log::error!("attempt to call unimplemented API 0x{:x} {}", addr, api);
            }
            log::warn!(
                "calling unimplemented API 0x{:x} {} at 0x{:x}",
                addr,
                api,
                emu.regs().rip
            );
            return api.to_ascii_lowercase();
        }
    }

    String::new()
}

fn _initterm_e(emu: &mut emu::Emu) {
    let start_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let end_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    log_red!(
        emu,
        "msvcrt!_initterm_e 0x{:x} - 0x{:x}",
        start_ptr,
        end_ptr
    );

    emu.regs_mut().rax = 0;
}

fn _initterm(emu: &mut emu::Emu) {
    let start_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let end_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    log_red!(emu, "msvcrt!_initterm 0x{:x} - 0x{:x}", start_ptr, end_ptr);

    emu.regs_mut().rax = 0;
}

fn StrCmpCA(emu: &mut emu::Emu) {
    let str1_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let str2_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    if str1_ptr == 0 || str2_ptr == 0 {
        emu.regs_mut().rax = 0xffffffff;
        log_red!(emu, "msvcrt!StrCmpA null ptrs");
        return;
    }

    let str1 = emu.maps.read_string(str1_ptr);
    let str2 = emu.maps.read_string(str2_ptr);

    log_red!(emu, "msvcrt!StrCmpA {} == {}", str1, str2);

    if str1 == str2 {
        emu.regs_mut().rax = 0;
    } else {
        emu.regs_mut().rax = 0xffffffff;
    }
}

fn fopen(emu: &mut emu::Emu) {
    let filepath_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let mode_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    if filepath_ptr == 0 || mode_ptr == 0 {
        emu.regs_mut().rax = 0xffffffff;
        log_red!(emu, "msvcrt!fopen null ptrs");
        return;
    }

    let filepath = emu.maps.read_string(filepath_ptr);
    let mode = emu.maps.read_string(mode_ptr);

    log_red!(emu, "msvcrt!fopen `{}` fmt:`{}`", filepath, mode);

    emu.regs_mut().rax = helper::handler_create(&filepath);
}

fn fwrite(emu: &mut emu::Emu) {
    let buff_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let size = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let nemb = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let file = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    let filename = helper::handler_get_uri(file as u64);

    log_red!(
        emu,
        "msvcrt!fwrite `{}` 0x{:x} {}",
        filename,
        buff_ptr,
        size
    );

    emu.regs_mut().rax = size as u64;
}

fn fflush(emu: &mut emu::Emu) {
    let file = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    let filename = helper::handler_get_uri(file as u64);

    log_red!(emu, "msvcrt!fflush `{}`", filename);

    //emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}

fn fclose(emu: &mut emu::Emu) {
    let file = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    let filename = helper::handler_get_uri(file as u64);

    log_red!(emu, "msvcrt!fclose `{}`", filename);

    emu.regs_mut().rax = 1;
}

fn __p___argv(emu: &mut emu::Emu) {
    log_red!(emu, "msvcrt!__p___argc");
    emu.regs_mut().rax = 0;
}

fn __p___argc(emu: &mut emu::Emu) {
    let args_base = match emu.maps.get_map_by_name("args") {
        Some(a) => a.get_base(),
        None => {
            let addr = match emu.maps.alloc(1024) {
                Some(a) => a,
                None => {
                    log::error!("msvcrt!__p___argc: out of memory");
                    emu.regs_mut().rax = 0;
                    return;
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
                    return;
                }
            }
        }
    };

    log_red!(emu, "msvcrt!__p___argc {}", args_base);

    emu.regs_mut().rax = args_base;
}

fn malloc(emu: &mut emu::Emu) {
    let size = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    if size > 0 {
        let base = if emu.cfg.memory_guard {
            emu.kernel_alloc(Region::Slab, size, "malloc", "malloc", false)
        } else {
            allocate_memory(emu, size).unwrap_or(0)
        };

        log_red!(emu, "msvcrt!malloc sz: {} addr: 0x{:x}", size, base);

        emu.regs_mut().rax = base;
    } else {
        emu.regs_mut().rax = 0x1337;
    }
}

fn __p__acmdln(emu: &mut emu::Emu) {
    log_red!(emu, "msvcrt!__p___argc");
    emu.regs_mut().rax = 0;
}

fn _onexit(emu: &mut emu::Emu) {
    let fptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    log_red!(emu, "msvcrt!_onexit 0x{:x}", fptr);

    emu.regs_mut().rax = fptr;
}

fn _lock(emu: &mut emu::Emu) {
    let lock_num = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "msvcrt!_lock {}", lock_num);

    emu.regs_mut().rax = 1;
}

fn free(emu: &mut emu::Emu) {
    let addr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "msvcrt!free 0x{:x}", addr);

    if emu.cfg.memory_guard {
        emu.kernel_free(addr as u64, "free");
    } else {
        release(emu, addr as u64);
    }
}

fn realloc(emu: &mut emu::Emu) {
    let addr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let size = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    if addr == 0 {
        if size == 0 {
            emu.regs_mut().rax = 0;
            return;
        } else {
            let base = allocate_memory(emu, size).unwrap_or(0);

            log_red!(emu, "msvcrt!realloc 0x{:x} {} =0x{:x}", addr, size, base);

            emu.regs_mut().rax = base;
            return;
        }
    }

    if size == 0 {
        log_red!(emu, "msvcrt!realloc 0x{:x} {} =0x1337", addr, size);

        emu.regs_mut().rax = 0x1337; // weird msvcrt has to return a random unallocated pointer, and the program has to do free() on it
        return;
    }

    let prev_size = match classify(emu, addr) {
        AllocKind::Arena { size } | AllocKind::Map { size, .. } => size,
        AllocKind::Invalid => {
            emu.regs_mut().rax = 0;
            return;
        }
    };

    let new_addr = if emu.cfg.memory_guard {
        emu.kernel_alloc(Region::Slab, size, "malloc", "realloc", false)
    } else {
        allocate_memory(emu, size).unwrap_or(0)
    };

    emu.maps
        .memcpy(new_addr, addr, std::cmp::min(prev_size, size as usize));
    if emu.cfg.memory_guard {
        emu.kernel_free(addr, "realloc");
    } else {
        release(emu, addr);
    }

    log_red!(
        emu,
        "msvcrt!realloc 0x{:x} {} =0x{:x}",
        addr,
        size,
        new_addr
    );

    emu.regs_mut().rax = new_addr;
}

fn strtok(emu: &mut emu::Emu) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    let delim_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    let str = emu.maps.read_string(str_ptr as u64);
    let delim = emu.maps.read_string(delim_ptr as u64);

    log_red!(emu, "msvcrt!strtok `{}` `{}`", str, delim);

    emu.regs_mut().rax = 0;
}

fn __set_app_type(emu: &mut emu::Emu) {
    let app_type = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "msvcrt!__set_app_type  app_type: 0x{:x}", app_type);
}

fn exit(emu: &mut emu::Emu) {
    let status = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "msvcrt!exit {}", status);
    emu.stop();
}

fn _errno(emu: &mut emu::Emu) {
    log_red!(emu, "msvcrt!_errno =0");
    emu.regs_mut().rax = 0;
}

fn __p__global_int(emu: &mut emu::Emu, name: &str) {
    let addr = allocate_memory(emu, 4).unwrap_or(0);
    if addr != 0 {
        emu.maps.write_dword(addr, 0);
    }
    log_red!(emu, "msvcrt!{} => 0x{:x}", name, addr);
    emu.regs_mut().rax = addr;
}
