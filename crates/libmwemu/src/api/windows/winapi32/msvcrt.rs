use crate::api::windows::common::fallback;
use crate::emu;
use crate::kernel::heap::Region;
use crate::maps::mem64::Permission;
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
        "exit" | "_exit" => exit(emu),
        "__set_app_type" => __set_app_type(emu),
        "_errno" => _errno(emu),
        "__p__fmode" => __p__global_int(emu, "__p__fmode"),
        "__p__commode" => __p__global_int(emu, "__p__commode"),
        "_adjust_fdiv" => __p__global_int(emu, "_adjust_fdiv"),
        "__setusermatherr" => {
            let handler = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "msvcrt!__setusermatherr handler:0x{:x}", handler);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "_controlfp" => {
            let new_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let mask = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "msvcrt!_controlfp new:0x{:x} mask:0x{:x}",
                new_val,
                mask
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0x0009_001f; // default x87 control word
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "_acmdln" | "__p__acmdln" => __p__acmdln(emu),
        "_XcptFilter" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "msvcrt!_XcptFilter arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "__dllonexit" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "msvcrt!__dllonexit arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_except_handler3" | "__CxxFrameHandler" => {
            let exception_record = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "msvcrt!{} exc_record:0x{:x}", api, exception_record);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1; // ExceptionContinueSearch
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "_EH_prolog" => {
            let handler = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "msvcrt!_EH_prolog handler:0x{:x}", handler);
            emu.stack_push32(emu.regs().get_ebp() as u32);
        }
        "_CxxThrowException" => {
            let exc_object = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let throw_info = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "msvcrt!_CxxThrowException obj:0x{:x} info:0x{:x}",
                exc_object,
                throw_info
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "_ftol" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "msvcrt!_ftol arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "time" => {
            let time_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let tick = emu.tick as u32;
            if time_ptr != 0 {
                emu.maps.write_dword(time_ptr, tick);
            }
            log_red!(emu, "msvcrt!time => {}", tick);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = tick as u64;
        }
        "localtime" => {
            let time_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            emu.stack_pop32(false);
            let timestamp = if time_ptr != 0 {
                emu.maps.read_dword(time_ptr).unwrap_or(0) as i64
            } else {
                0
            };
            let tm_addr = allocate_memory(emu, 36).unwrap_or(0);
            if tm_addr != 0 {
                // struct tm: sec, min, hour, mday, mon, year, wday, yday, isdst
                let secs = timestamp % 60;
                let mins = (timestamp / 60) % 60;
                let hours = (timestamp / 3600) % 24;
                let days = timestamp / 86400;
                let wday = ((days + 4) % 7) as u32; // 1970-01-01 was Thursday
                let mut y = 1970i64;
                let mut rem = days;
                loop {
                    let ylen: i64 = if y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) {
                        366
                    } else {
                        365
                    };
                    if rem < ylen {
                        break;
                    }
                    rem -= ylen;
                    y += 1;
                }
                let leap = y % 4 == 0 && (y % 100 != 0 || y % 400 == 0);
                let mdays: [i64; 12] = [
                    31,
                    if leap { 29 } else { 28 },
                    31,
                    30,
                    31,
                    30,
                    31,
                    31,
                    30,
                    31,
                    30,
                    31,
                ];
                let mut mon = 0u32;
                for md in &mdays {
                    if rem < *md {
                        break;
                    }
                    rem -= *md;
                    mon += 1;
                }
                emu.maps.write_dword(tm_addr, secs as u32);
                emu.maps.write_dword(tm_addr + 4, mins as u32);
                emu.maps.write_dword(tm_addr + 8, hours as u32);
                emu.maps.write_dword(tm_addr + 12, (rem + 1) as u32);
                emu.maps.write_dword(tm_addr + 16, mon);
                emu.maps.write_dword(tm_addr + 20, (y - 1900) as u32);
                emu.maps.write_dword(tm_addr + 24, wday);
                emu.maps.write_dword(tm_addr + 28, (days % 365) as u32);
                emu.maps.write_dword(tm_addr + 32, 0); // isdst
            }
            log_red!(emu, "msvcrt!localtime ts:{} => 0x{:x}", timestamp, tm_addr);
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.regs_mut().rax = s.len() as u64;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "srand" => {
            let seed = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "msvcrt!srand seed:{}", seed);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "rand" => {
            let val = ((emu.tick as u64)
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407)
                >> 33) as u32
                & 0x7fff;
            log_red!(emu, "msvcrt!rand => {}", val);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = val as u64;
        }
        "atoi" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            let val = s.trim().parse::<i32>().unwrap_or(0);
            log_red!(emu, "msvcrt!atoi '{}' => {}", s, val);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = val as u32 as u64;
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.regs_mut().rax = buf;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "getenv" | "_wgetenv" => {
            let ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name = emu.maps.read_string(ptr);
            log_red!(emu, "msvcrt!{} '{}' => NULL", api, name);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "printf" => {
            let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "msvcrt!printf fmt:'{}'", fmt);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "sprintf" => {
            let buf = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let fmt = emu.maps.read_string(fmt_ptr);
            if buf != 0 {
                emu.maps.write_string(buf, &fmt);
            }
            log_red!(emu, "msvcrt!sprintf fmt:'{}'", fmt);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "swprintf" => {
            let buf = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let fmt = emu.maps.read_wide_string(fmt_ptr);
            if buf != 0 {
                emu.maps.write_wide_string(buf, &fmt);
            }
            log_red!(emu, "msvcrt!swprintf fmt:'{}'", fmt);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "strncmp" => {
            let s1_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s2_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let n = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
            let s1 = emu.maps.read_string(s1_ptr);
            let s2 = emu.maps.read_string(s2_ptr);
            let cmp1: String = s1.chars().take(n).collect();
            let cmp2: String = s2.chars().take(n).collect();
            let result = cmp1.cmp(&cmp2) as i32;
            log_red!(emu, "msvcrt!strncmp '{}' '{}' {} => {}", s1, s2, n, result);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = result as u32 as u64;
        }
        "toupper" => {
            let c = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let result = if (0x61..=0x7a).contains(&c) {
                c - 0x20
            } else {
                c
            };
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = result as u64;
        }
        "tolower" => {
            let c = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let result = if (0x41..=0x5a).contains(&c) {
                c + 0x20
            } else {
                c
            };
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = result as u64;
        }
        "wcslen" => {
            let s_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(s_ptr);
            let len = s.len();
            log_red!(emu, "msvcrt!wcslen '{}' => {}", s, len);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = len as u64;
        }
        "wcscpy" => {
            let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(src);
            if dst != 0 {
                emu.maps.write_wide_string(dst, &s);
            }
            log_red!(emu, "msvcrt!wcscpy dst:0x{:x} '{}'", dst, s);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = dst;
        }
        "wcscat" => {
            let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let existing = emu.maps.read_wide_string(dst);
            let append = emu.maps.read_wide_string(src);
            let result = format!("{}{}", existing, append);
            if dst != 0 {
                emu.maps.write_wide_string(dst, &result);
            }
            log_red!(emu, "msvcrt!wcscat => '{}'", result);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = dst;
        }
        "wcscmp" => {
            let s1_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s2_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s1 = emu.maps.read_wide_string(s1_ptr);
            let s2 = emu.maps.read_wide_string(s2_ptr);
            let result = s1.cmp(&s2) as i32;
            log_red!(emu, "msvcrt!wcscmp '{}' '{}' => {}", s1, s2, result);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = result as u32 as u64;
        }
        "freopen" => {
            let path_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let path = emu.maps.read_string(path_ptr);
            let hndl = helper::handler_create(&path);
            log_red!(emu, "msvcrt!freopen '{}' => 0x{:x}", path, hndl);
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "_wrename" => {
            let old_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let new_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let old_name = emu.maps.read_wide_string(old_ptr);
            let new_name = emu.maps.read_wide_string(new_ptr);
            log_red!(emu, "msvcrt!_wrename '{}' => '{}'", old_name, new_name);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "_wsystem" => {
            let cmd_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let cmd = emu.maps.read_wide_string(cmd_ptr);
            log_red!(emu, "msvcrt!_wsystem cmd:'{}'", cmd);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "_iob" => {
            let iob_addr = allocate_memory(emu, 96).unwrap_or(0);
            log_red!(emu, "msvcrt!_iob => 0x{:x}", iob_addr);
            emu.set_last_error(0);
            emu.regs_mut().rax = iob_addr;
        }
        "??2@YAPAXI@Z" => {
            // operator new(unsigned int)
            let size = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            emu.stack_pop32(false);
            let addr = if size > 0 {
                allocate_memory(emu, size).unwrap_or(0)
            } else {
                0
            };
            log_red!(emu, "msvcrt!operator_new sz:{} => 0x{:x}", size, addr);
            emu.set_last_error(0);
            emu.regs_mut().rax = addr;
        }
        "??3@YAXPAX@Z" => {
            // operator delete(void*)
            let ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            emu.stack_pop32(false);
            log_red!(emu, "msvcrt!operator_delete ptr:0x{:x}", ptr);
            if ptr != 0 {
                if emu.cfg.memory_guard {
                    emu.kernel_free(ptr, "operator_delete");
                } else {
                    release(emu, ptr);
                }
            }
        }
        "??1type_info@@UAE@XZ" => {
            let this = emu.regs().get_ecx();
            log_red!(emu, "msvcrt!type_info::~type_info this:0x{:x}", this);
            if this != 0 && emu.maps.is_mapped(this) {
                let _vt = emu.maps.read_dword(this).unwrap_or(0);
            }
            emu.set_last_error(0);
        }
        "??0exception@@QAE@ABV0@@Z" => {
            // exception::exception(const exception&) - thiscall copy ctor
            let this = emu.regs().get_ecx();
            let other = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            emu.stack_pop32(false);
            if other != 0 && this != 0 {
                let vtable = emu.maps.read_dword(other).unwrap_or(0);
                let msg_ptr = emu.maps.read_dword(other + 4).unwrap_or(0);
                emu.maps.write_dword(this, vtable);
                emu.maps.write_dword(this + 4, msg_ptr);
            }
            log_red!(
                emu,
                "msvcrt!exception::exception(copy) this:0x{:x} other:0x{:x}",
                this,
                other
            );
            emu.regs_mut().rax = this;
        }

        // --- String functions ---
        "strcmp" => {
            let s1_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s2_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s1 = emu.maps.read_string(s1_ptr);
            let s2 = emu.maps.read_string(s2_ptr);
            let result = s1.cmp(&s2) as i32;
            log_red!(emu, "msvcrt!strcmp '{}' '{}' => {}", s1, s2, result);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = result as u32 as u64;
        }
        "strcpy" => {
            let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(src);
            if dst != 0 {
                emu.maps.write_string(dst, &s);
            }
            log_red!(emu, "msvcrt!strcpy dst:0x{:x} '{}'", dst, s);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = dst;
        }
        "strncpy" => {
            let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let n = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
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
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = dst;
        }
        "strcat" => {
            let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let existing = emu.maps.read_string(dst);
            let append = emu.maps.read_string(src);
            let result = format!("{}{}", existing, append);
            if dst != 0 {
                emu.maps.write_string(dst, &result);
            }
            log_red!(emu, "msvcrt!strcat => '{}'", result);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = dst;
        }
        "strncat" => {
            let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let n = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
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
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "strstr" => {
            let haystack_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let needle_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = result;
        }
        "strchr" => {
            let s_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let c = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u8 as char;
            let s = emu.maps.read_string(s_ptr);
            let found = s.find(c);
            let result = match found {
                Some(pos) => s_ptr + pos as u64,
                None => 0,
            };
            log_red!(emu, "msvcrt!strchr '{}' '{}' => 0x{:x}", s, c, result);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = result;
        }
        "strrchr" => {
            let s_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let c = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u8 as char;
            let s = emu.maps.read_string(s_ptr);
            let found = s.rfind(c);
            let result = match found {
                Some(pos) => s_ptr + pos as u64,
                None => 0,
            };
            log_red!(emu, "msvcrt!strrchr '{}' '{}' => 0x{:x}", s, c, result);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = result;
        }
        "_stricmp" => {
            let s1_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s2_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s1 = emu.maps.read_string(s1_ptr).to_lowercase();
            let s2 = emu.maps.read_string(s2_ptr).to_lowercase();
            let result = s1.cmp(&s2) as i32;
            log_red!(emu, "msvcrt!_stricmp '{}' '{}' => {}", s1, s2, result);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = result as u32 as u64;
        }
        "_strnicmp" => {
            let s1_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s2_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let n = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
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
            emu.regs_mut().rax = result as u64;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "strlen" => {
            let s_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = emu.maps.read_string(s_ptr);
            let len = s.len();
            log_red!(emu, "msvcrt!strlen '{}' => {}", s, len);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = len as u64;
        }
        "strtol" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _endptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let base = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(10);
            let s = emu.maps.read_string(str_ptr);
            let val = i64::from_str_radix(s.trim(), base).unwrap_or(0);
            log_red!(emu, "msvcrt!strtol '{}' base:{} => {}", s, base, val);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = val as u64;
        }
        "strtoul" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _endptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let base = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(10);
            let s = emu.maps.read_string(str_ptr);
            let val = u64::from_str_radix(s.trim(), base).unwrap_or(0);
            log_red!(emu, "msvcrt!strtoul '{}' base:{} => {}", s, base, val);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = val;
        }
        "atol" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            let val = s.trim().parse::<i32>().unwrap_or(0);
            log_red!(emu, "msvcrt!atol '{}' => {}", s, val);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = val as u32 as u64;
        }
        "_snprintf" | "snprintf" | "sprintf_s" | "_snprintf_s" | "_sprintf_p" => {
            let buf = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _size = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "msvcrt!{} buf:0x{:x} fmt:'{}'", api, buf, fmt);
            if buf != 0 && !fmt.is_empty() {
                emu.maps.write_string(buf, &fmt);
            }
            emu.regs_mut().rax = fmt.len() as u64;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "sscanf" | "_sscanf_s_l" | "sscanf_s" => {
            let buf = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(buf);
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "msvcrt!{} buf:'{}' fmt:'{}'", api, s, fmt);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "wcsncpy" => {
            let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let n = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
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
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = dst;
        }
        "wcsncat" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "msvcrt!wcsncat arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_wcsicmp" => {
            let s1_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s2_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s1 = emu.maps.read_wide_string(s1_ptr).to_lowercase();
            let s2 = emu.maps.read_wide_string(s2_ptr).to_lowercase();
            let result = s1.cmp(&s2) as i32;
            log_red!(emu, "msvcrt!_wcsicmp '{}' '{}' => {}", s1, s2, result);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = result as u32 as u64;
        }
        "_wcsnicmp" => {
            let s1_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s2_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let n = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
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
            emu.regs_mut().rax = result as u64;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "_wtoi" => {
            let ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let ws = emu.maps.read_wide_string(ptr);
            let val = ws.parse::<i32>().unwrap_or(0);
            log_red!(emu, "msvcrt!_wtoi '{}' => {}", ws, val);
            emu.regs_mut().rax = val as u64;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "_wtol" => {
            let ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let ws = emu.maps.read_wide_string(ptr);
            let val = ws.parse::<i32>().unwrap_or(0);
            log_red!(emu, "msvcrt!_wtol '{}' => {}", ws, val);
            emu.regs_mut().rax = val as u64;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "towupper" => {
            let c = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let ch = char::from_u32(c).unwrap_or('\0');
            let upper = ch.to_uppercase().next().unwrap_or(ch);
            log_red!(emu, "msvcrt!towupper '{}' => '{}'", ch, upper);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = upper as u32 as u64;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "towlower" => {
            let c = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let ch = char::from_u32(c).unwrap_or('\0');
            let lower = ch.to_lowercase().next().unwrap_or(ch);
            log_red!(emu, "msvcrt!towlower '{}' => '{}'", ch, lower);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = lower as u32 as u64;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "mbstowcs" | "_mbstowcs_l" => {
            let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let n = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
            let s = emu.maps.read_string(src);
            let copy_len = s.len().min(n);
            for (i, b) in s.as_bytes()[..copy_len].iter().enumerate() {
                emu.maps.write_word(dst + (i * 2) as u64, *b as u16);
            }
            if copy_len < n {
                emu.maps.write_word(dst + (copy_len * 2) as u64, 0);
            }
            log_red!(emu, "msvcrt!{} '{}' => {}", api, s, copy_len);
            emu.regs_mut().rax = copy_len as u64;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "wcstombs" | "_wcstombs_l" => {
            let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let n = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
            let ws = emu.maps.read_wide_string(src);
            let copy_len = ws.len().min(n);
            for (i, b) in ws.as_bytes()[..copy_len].iter().enumerate() {
                emu.maps.write_byte(dst + i as u64, *b);
            }
            if copy_len < n {
                emu.maps.write_byte(dst + copy_len as u64, 0);
            }
            log_red!(emu, "msvcrt!{} '{}' => {}", api, ws, copy_len);
            emu.regs_mut().rax = copy_len as u64;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "isalpha" | "isdigit" | "isspace" | "isalnum" | "ispunct" | "isupper" | "islower"
        | "isxdigit" => {
            let c = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
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
            let name = format!("'{}'", ch);
            log_red!(emu, "msvcrt!{} {} => {}", api, name, r);
            emu.set_last_error(0);
            emu.regs_mut().rax = r;
        }

        // --- Memory functions ---
        "calloc" => {
            let nmemb = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let size = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let total = nmemb.saturating_mul(size);
            if total == 0 {
                emu.regs_mut().rax = 0;
            } else {
                let base = allocate_memory(emu, total).unwrap_or(0);
                if base != 0 {
                    emu.maps.memset(base, 0, total as usize);
                }
                log_red!(emu, "msvcrt!calloc {}*{} =0x{:x}", nmemb, size, base);
                let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
                if first_param != 0 && emu.maps.is_mapped(first_param) {
                    let _ = emu.maps.read_dword(first_param).unwrap_or(0);
                }
                emu.regs_mut().rax = base;
            }
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "memset" => {
            let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let val = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u8;
            let n = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
            if dst != 0 && n > 0 {
                for i in 0..n {
                    emu.maps.write_byte(dst + i as u64, val);
                }
            }
            log_red!(emu, "msvcrt!memset dst:0x{:x} val:0x{:x} n:{}", dst, val, n);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = dst;
        }
        "memcpy" => {
            let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let n = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = dst;
        }
        "memmove" => {
            let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let n = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = dst;
        }
        "memcmp" => {
            let s1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let n = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = result as u32 as u64;
        }
        "memchr" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "msvcrt!memchr arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_msize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "msvcrt!_msize arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_aligned_malloc" => {
            let size = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _alignment = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(16);
            let base = allocate_memory(emu, size).unwrap_or(0);
            log_red!(emu, "msvcrt!_aligned_malloc {} =0x{:x}", size, base);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = base;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "_aligned_free" => {
            let addr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "msvcrt!_aligned_free 0x{:x}", addr);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            release(emu, addr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }

        // --- I/O functions ---
        "fread" => {
            let buf = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let size = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let count = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let fp = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            helper::handler_set_eof(fp);
            emu.regs_mut().rax = 0;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "fseek" => {
            let fp = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let offset = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as i32 as i64;
            let origin = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let new_pos = helper::handler_seek(fp, offset, origin);
            log_red!(
                emu,
                "msvcrt!fseek fp:0x{:x} offset:{} origin:{} => pos:{}",
                fp,
                offset,
                origin,
                new_pos
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.regs_mut().rax = if new_pos >= 0 { 0 } else { u64::MAX };
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "ftell" => {
            let fp = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pos = helper::handler_tell(fp);
            log_red!(emu, "msvcrt!ftell fp:0x{:x} => {}", fp, pos);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.regs_mut().rax = pos as u64;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "feof" => {
            let fp = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let is_eof = helper::handler_eof(fp);
            log_red!(emu, "msvcrt!feof fp:0x{:x} => {}", fp, is_eof as u32);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = is_eof as u64;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "fgets" => {
            let buf = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let n = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let stream = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let name = helper::handler_get_uri(stream);
            log_red!(
                emu,
                "msvcrt!fgets buf:0x{:x} n:{} stream:0x{:x} '{}'",
                buf,
                n,
                stream,
                name
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
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
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "fprintf" => {
            let _stream = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "msvcrt!fprintf fmt:'{}'", fmt);
            emu.regs_mut().rax = fmt.len() as u64;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "vprintf" => {
            let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "msvcrt!vprintf fmt:'{}'", fmt);
            emu.regs_mut().rax = fmt.len() as u64;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "vsprintf" | "_vsnprintf" | "vsnprintf" => {
            let buf = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "msvcrt!{} buf:0x{:x} fmt:'{}'", api, buf, fmt);
            if buf != 0 && !fmt.is_empty() {
                emu.maps.write_string(buf, &fmt);
            }
            emu.regs_mut().rax = fmt.len() as u64;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "vfprintf" => {
            let _stream = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "msvcrt!vfprintf fmt:'{}'", fmt);
            emu.regs_mut().rax = fmt.len() as u64;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "fputs" => {
            let s_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = emu.maps.read_string(s_ptr);
            log_red!(emu, "msvcrt!fputs '{}'", s);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "puts" => {
            let s_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = emu.maps.read_string(s_ptr);
            log_red!(emu, "msvcrt!puts '{}'", s);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "putchar" | "putc" | "_putch" => {
            let c = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let ch_str = format!("{}", c as u8 as char);
            log_red!(emu, "msvcrt!{} '{}'", api, ch_str);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = c as u64;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "getc" | "fgetc" | "_fgetchar" | "getchar" => {
            let stream = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name = helper::handler_get_uri(stream);
            log_red!(
                emu,
                "msvcrt!{} stream:0x{:x} '{}' => EOF",
                api,
                stream,
                name
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0xffff_ffff; // EOF
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "_fileno" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "msvcrt!_fileno arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_setmode" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "msvcrt!_setmode arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_open" | "_wopen" => {
            let path_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let path = emu.maps.read_string(path_ptr);
            log_red!(emu, "msvcrt!{} '{}'", api, path);
            emu.regs_mut().rax = helper::handler_create(&path);
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "_close" => {
            let fd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name = helper::handler_get_uri(fd);
            let ok = helper::handler_close(fd);
            log_red!(emu, "msvcrt!_close fd:{} '{}' ok:{}", fd, name, ok);
            emu.regs_mut().rax = if ok { 0 } else { u64::MAX };
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "_read" => {
            let fd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let buf = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let count = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let name = helper::handler_get_uri(fd);
            log_red!(
                emu,
                "msvcrt!_read fd:{} '{}' buf:0x{:x} count:{}",
                fd,
                name,
                buf,
                count
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "_write" => {
            let fd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let buf = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let count = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let name = helper::handler_get_uri(fd);
            log_red!(
                emu,
                "msvcrt!_write fd:{} '{}' buf:0x{:x} count:{}",
                fd,
                name,
                buf,
                count
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = count as u64;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }

        // --- Process/Environment ---
        "_cexit" | "_c_exit" => {
            log_red!(emu, "msvcrt!{} cleaning up", api);
            helper::advance_tick(emu, 1);
        }
        "_beginthread" => {
            let start = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "msvcrt!_beginthread start:0x{:x}", start);
            let hndl = helper::handler_create("msvcrt://_beginthread");
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "_beginthreadex" => {
            let _security = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let _stack_size = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let start = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            log_red!(emu, "msvcrt!_beginthreadex start:0x{:x}", start);
            let hndl = helper::handler_create("msvcrt://_beginthreadex");
            emu.regs_mut().rax = hndl;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "_endthread" | "_endthreadex" => {
            log_red!(emu, "msvcrt!{}", api);
            helper::advance_tick(emu, 1);
            emu.stop();
        }
        "system" => {
            let cmd_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let cmd = emu.maps.read_string(cmd_ptr);
            log_red!(emu, "msvcrt!system '{}'", cmd);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "_putenv" | "_wputenv" | "_putenv_s" => {
            let ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = emu.maps.read_string(ptr);
            log_red!(emu, "msvcrt!{} '{}'", api, s);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "__wgetmainargs" => {
            let p_argc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p_argv = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p_env = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "msvcrt!__wgetmainargs");
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
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "_wcmdln" | "__p__wcmdln" => {
            let addr = allocate_memory(emu, 64).unwrap_or(0);
            if addr != 0 {
                emu.maps.write_wide_string(addr, "mwemu.exe");
            }
            log_red!(emu, "msvcrt!{} => 0x{:x}", api, addr);
            emu.regs_mut().rax = addr;
        }

        // --- Exception handling ---
        "_except_handler4"
        | "_except_handler4_common"
        | "__CxxFrameHandler3"
        | "__CxxFrameHandler4" => {
            let exc_record = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let frame_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "msvcrt!{} exc:0x{:x} frame:0x{:x}",
                api,
                exc_record,
                frame_ptr
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1; // ExceptionContinueSearch
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "_set_se_translator" | "_set_new_handler" | "_set_new_mode" | "set_terminate"
        | "set_unexpected" => {
            let handler = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "msvcrt!{} handler:0x{:x}", api, handler);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "_EH_prolog3" | "_EH_prolog3_catch" | "_EH_prolog3_GS" => {
            let cookie = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "msvcrt!{} cookie:0x{:x}", api, cookie);
            emu.stack_push32(emu.regs().get_ebp() as u32);
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
            let val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as i32;
            let result = val.unsigned_abs();
            log_red!(emu, "msvcrt!{} {} => {}", api, val, result);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = result as u64;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "ceil" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val.ceil();
            emu.fpu_mut().set_st(0, result);
            log_red!(emu, "msvcrt!ceil {} => {}", val, result);
            emu.set_last_error(0);
        }
        "floor" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val.floor();
            emu.fpu_mut().set_st(0, result);
            log_red!(emu, "msvcrt!floor {} => {}", val, result);
            emu.set_last_error(0);
        }
        "sqrt" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val.sqrt();
            emu.fpu_mut().set_st(0, result);
            log_red!(emu, "msvcrt!sqrt {} => {}", val, result);
            emu.set_last_error(0);
        }
        "pow" => {
            let base_val = f64::from_bits(
                (emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64) << 32
                    | emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64,
            );
            let exp_val = f64::from_bits(
                (emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64) << 32
                    | emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64,
            );
            let result = base_val.powf(exp_val);
            log_red!(emu, "msvcrt!pow {} {} => {}", base_val, exp_val, result);
            emu.fpu_mut().set_st(0, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "log" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val.ln();
            emu.fpu_mut().set_st(0, result);
            log_red!(emu, "msvcrt!log {} => {}", val, result);
            emu.set_last_error(0);
        }
        "log10" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val.log10();
            emu.fpu_mut().set_st(0, result);
            log_red!(emu, "msvcrt!log10 {} => {}", val, result);
            emu.set_last_error(0);
        }
        "exp" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val.exp();
            emu.fpu_mut().set_st(0, result);
            log_red!(emu, "msvcrt!exp {} => {}", val, result);
            emu.set_last_error(0);
        }
        "sin" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val.sin();
            emu.fpu_mut().set_st(0, result);
            log_red!(emu, "msvcrt!sin {} => {}", val, result);
            emu.set_last_error(0);
        }
        "cos" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val.cos();
            emu.fpu_mut().set_st(0, result);
            log_red!(emu, "msvcrt!cos {} => {}", val, result);
            emu.set_last_error(0);
        }
        "tan" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val.tan();
            emu.fpu_mut().set_st(0, result);
            log_red!(emu, "msvcrt!tan {} => {}", val, result);
            emu.set_last_error(0);
        }
        "atan" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val.atan();
            emu.fpu_mut().set_st(0, result);
            log_red!(emu, "msvcrt!atan {} => {}", val, result);
            emu.set_last_error(0);
        }
        "atan2" => {
            let y = f64::from_bits(
                (emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64) << 32
                    | emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64,
            );
            let x = f64::from_bits(
                (emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64) << 32
                    | emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64,
            );
            let result = y.atan2(x);
            log_red!(emu, "msvcrt!atan2 {} {} => {}", y, x, result);
            emu.fpu_mut().set_st(0, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "fabs" => {
            let val = emu.fpu_mut().get_st(0);
            let result = val.abs();
            emu.fpu_mut().set_st(0, result);
            log_red!(emu, "msvcrt!fabs {} => {}", val, result);
            emu.set_last_error(0);
        }
        "fmod" => {
            let x = f64::from_bits(
                (emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64) << 32
                    | emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64,
            );
            let y = f64::from_bits(
                (emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64) << 32
                    | emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64,
            );
            let result = x % y;
            log_red!(emu, "msvcrt!fmod {} {} => {}", x, y, result);
            emu.fpu_mut().set_st(0, result);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }

        // --- Misc ---
        "clock" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "msvcrt!clock 0x{:x}", arg1);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            helper::advance_tick(emu, 1);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "_time64" => {
            let t_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let now = 0x6718_0000u64;
            if t_ptr != 0 {
                emu.maps.write_qword(t_ptr, now);
            }
            log_red!(emu, "msvcrt!_time64 => 0x{:x}", now);
            emu.regs_mut().rax = now;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "_controlfp_s" => {
            let p_current = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if p_current != 0 {
                emu.maps.write_dword(p_current, 0x0008_001f);
            }
            log_red!(emu, "msvcrt!_controlfp_s");
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "_unlock" => {
            let lock_num = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "msvcrt!_unlock {}", lock_num);
            helper::advance_tick(emu, 1);
        }
        "_configthreadlocale" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "msvcrt!_configthreadlocale arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_amsg_exit" => {
            let code = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "msvcrt!_amsg_exit code:{}", code);
            emu.stop();
        }
        "_crt_atexit" | "_register_onexit_function" => {
            let func_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "msvcrt!{} func:0x{:x}", api, func_ptr);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "_initialize_onexit_table" => {
            let table_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if table_ptr != 0 {
                emu.maps.write_dword(table_ptr, 0); // _first
                emu.maps.write_dword(table_ptr + 4, 0); // _last
                emu.maps.write_dword(table_ptr + 8, 0); // _end
            }
            log_red!(emu, "msvcrt!_initialize_onexit_table tbl:0x{:x}", table_ptr);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "__CppXcptFilter" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "msvcrt!__CppXcptFilter arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "__acrt_iob_func" => {
            let index = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
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
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = hndl;
        }
        "__iob_func" => {
            let iob_addr = allocate_memory(emu, 96).unwrap_or(0);
            log_red!(emu, "msvcrt!__iob_func => 0x{:x}", iob_addr);
            emu.set_last_error(0);
            emu.regs_mut().rax = iob_addr;
        }
        "__p__environ" => {
            let env_addr = allocate_memory(emu, 8).unwrap_or(0);
            if env_addr != 0 {
                emu.maps.write_dword(env_addr, 0); // NULL-terminated environ
            }
            log_red!(emu, "msvcrt!__p__environ => 0x{:x}", env_addr);
            emu.regs_mut().rax = env_addr;
        }
        "__stdio_common_vfprintf" => {
            let _options = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let _options_hi = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _stream = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "msvcrt!__stdio_common_vfprintf fmt:'{}'", fmt);
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "__stdio_common_vsprintf_s" => {
            let _options = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let _options_hi = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let buf = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let _buf_size = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
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
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "_configure_narrow_argv" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "msvcrt!_configure_narrow_argv arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_set_errno" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "msvcrt!_set_errno arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_get_errno" => {
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            log_red!(emu, "msvcrt!_get_errno => 0");
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "_get_initial_narrow_environment" => {
            let env_addr = allocate_memory(emu, 8).unwrap_or(0);
            if env_addr != 0 {
                emu.maps.write_dword(env_addr, 0); // NULL-terminated empty environment
            }
            log_red!(
                emu,
                "msvcrt!_get_initial_narrow_environment => 0x{:x}",
                env_addr
            );
            emu.regs_mut().rax = env_addr;
        }
        "_initialize_narrow_environment" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "msvcrt!_initialize_narrow_environment arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_localtime64" => {
            let time_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
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
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = tm_addr;
        }
        "_set_app_type" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "msvcrt!_set_app_type arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_set_invalid_parameter_handler" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "msvcrt!_set_invalid_parameter_handler arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_set_thread_local_invalid_parameter_handler" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "msvcrt!_set_thread_local_invalid_parameter_handler arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_strcmpi" => {
            let s1_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s2_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s1 = emu.maps.read_string(s1_ptr).to_lowercase();
            let s2 = emu.maps.read_string(s2_ptr).to_lowercase();
            let result = s1.cmp(&s2) as i32;
            log_red!(emu, "msvcrt!_strcmpi '{}' '{}' => {}", s1, s2, result);
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = result as u64;
        }
        "_time32" => {
            let timer_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let time_val = emu.tick as u32;
            if timer_ptr != 0 {
                emu.maps.write_dword(timer_ptr, time_val);
            }
            log_red!(emu, "msvcrt!_time32 => {}", time_val);
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = time_val as u64;
        }
        "_wfopen" => {
            let fname_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let mode_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = hndl;
        }
        "abort" => {
            log_red!(emu, "msvcrt!abort");
            helper::advance_tick(emu, 1);
            emu.stop();
        }
        "ptr" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "msvcrt!ptr arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "setvbuf" => {
            let stream = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _buffer = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let mode = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let size = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let uri = helper::handler_get_uri(stream);
            log_red!(
                emu,
                "msvcrt!setvbuf stream:0x{:x}('{}') mode:{} size:{}",
                stream,
                uri,
                mode,
                size
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // success
        }
        "signal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "msvcrt!signal 0x{:x}", arg1);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            helper::advance_tick(emu, 1);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "str" => {
            let s_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = emu.maps.read_string(s_ptr);
            log_red!(emu, "msvcrt!str 0x{:x} '{}'", s_ptr, s);
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = s_ptr;
        }
        "strtok_s" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let delim_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let context_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let delim = emu.maps.read_string(delim_ptr);
            let s = emu.maps.read_string(str_ptr);
            if let Some(pos) = s.find(|c: char| delim.contains(c)) {
                emu.maps.write_byte(str_ptr + pos as u64, 0);
                if context_ptr != 0 {
                    emu.maps
                        .write_dword(context_ptr, (str_ptr + pos as u64 + 1) as u32);
                }
                log_red!(emu, "msvcrt!strtok_s '{}' delim:'{}' => token", s, delim);
                emu.regs_mut().rax = str_ptr;
            } else {
                log_red!(emu, "msvcrt!strtok_s '{}' delim:'{}' => null", s, delim);
                emu.regs_mut().rax = 0;
            }
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "vwprintf" => {
            let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _va_list = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let fmt = emu.maps.read_wide_string(fmt_ptr);
            log_red!(emu, "msvcrt!vwprintf fmt:'{}'", fmt);
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "wcsftime" => {
            let wcs = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _maxsize = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let _timeptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let fmt = emu.maps.read_wide_string(fmt_ptr);
            if wcs != 0 && !fmt.is_empty() {
                emu.maps.write_wide_string(wcs, &fmt);
            }
            log_red!(emu, "msvcrt!wcsftime fmt:'{}' => {}", fmt, fmt.len());
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
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
            emu.regs_mut().rax = addr;
        }
        "_set_error_mode" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "msvcrt!_set_error_mode arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_set_fmode" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "msvcrt!_set_fmode arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "__stdio_common_vsprintf" => {
            let _options = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let _options_hi = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let buf = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let _buf_size = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
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
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "__stdio_common_vsscanf" => {
            let _options = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let _options_hi = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let buf = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let _buf_size = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = emu.maps.read_string(buf);
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(
                emu,
                "msvcrt!__stdio_common_vsscanf buf:'{}' fmt:'{}'",
                s,
                fmt
            );
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1; // number of fields read
        }
        "_sprintf_l" => {
            let buf = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let _locale = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "msvcrt!_sprintf_l buf:0x{:x} fmt:'{}'", buf, fmt);
            if buf != 0 && !fmt.is_empty() {
                emu.maps.write_string(buf, &fmt);
            }
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "wprintf" => {
            let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let fmt = emu.maps.read_wide_string(fmt_ptr);
            log_red!(emu, "msvcrt!wprintf fmt:'{}'", fmt);
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = fmt.len() as u64;
        }

        _ => return fallback::unimplemented(addr, api, emu),
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

    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

fn _initterm(emu: &mut emu::Emu) {
    let start_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let end_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    log_red!(emu, "msvcrt!_initterm 0x{:x} - 0x{:x}", start_ptr, end_ptr);

    emu.set_last_error(0);
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
    let total = (size as u64) * (nemb as u64);

    log_red!(
        emu,
        "msvcrt!fwrite `{}` buf:0x{:x} size:{} count:{}",
        filename,
        buff_ptr,
        size,
        nemb
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = nemb as u64;
}

fn fflush(emu: &mut emu::Emu) {
    let file = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    let filename = helper::handler_get_uri(file as u64);

    log_red!(emu, "msvcrt!fflush `{}`", filename);

    //emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}

fn fclose(emu: &mut emu::Emu) {
    let file = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let filename = helper::handler_get_uri(file);
    helper::handler_close(file);
    log_red!(emu, "msvcrt!fclose '{}' handle:0x{:x}", filename, file);
    emu.regs_mut().rax = 0;
}

fn __p___argv(emu: &mut emu::Emu) {
    let args_base = match emu.maps.get_map_by_name("args") {
        Some(a) => a.get_base(),
        None => {
            let addr = match emu.maps.alloc(1024) {
                Some(a) => a,
                None => {
                    log::error!("msvcrt!__p___argv: out of memory");
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
                    log::error!("msvcrt!__p___argv: cannot create args map: {}", e);
                    emu.regs_mut().rax = 0;
                    return;
                }
            }
        }
    };
    log_red!(emu, "msvcrt!__p___argv => 0x{:x}", args_base + 4);
    emu.regs_mut().rax = args_base + 4;
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
    let args_base = match emu.maps.get_map_by_name("args") {
        Some(a) => a.get_base(),
        None => {
            let addr = match emu.maps.alloc(1024) {
                Some(a) => a,
                None => {
                    log::error!("msvcrt!__p__acmdln: out of memory");
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
                    log::error!("msvcrt!__p__acmdln: cannot create args map: {}", e);
                    emu.regs_mut().rax = 0;
                    return;
                }
            }
        }
    };
    log_red!(emu, "msvcrt!__p__acmdln => 0x{:x}", args_base + 8);
    emu.regs_mut().rax = args_base + 8;
}

fn _onexit(emu: &mut emu::Emu) {
    let fptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    log_red!(emu, "msvcrt!_onexit 0x{:x}", fptr);

    emu.set_last_error(0);
    emu.regs_mut().rax = fptr;
}

fn _lock(emu: &mut emu::Emu) {
    let lock_num = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "msvcrt!_lock {}", lock_num);

    emu.set_last_error(0);
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
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let delim_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            emu.regs_mut().rax = 0;
        }
    } else {
        log_red!(emu, "msvcrt!strtok(NULL) => 0");
        emu.regs_mut().rax = 0;
    }
}

fn __set_app_type(emu: &mut emu::Emu) {
    let app_type = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "msvcrt!__set_app_type  app_type: 0x{:x}", app_type);
    emu.set_last_error(0);
}

fn exit(emu: &mut emu::Emu) {
    let status = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "msvcrt!exit {}", status);
    emu.stop();
}

fn _errno(emu: &mut emu::Emu) {
    let addr = allocate_memory(emu, 4).unwrap_or(0);
    if addr != 0 {
        emu.maps.write_dword(addr, 0);
    }
    log_red!(emu, "msvcrt!_errno => 0x{:x}", addr);
    emu.regs_mut().rax = addr;
}

fn __p__global_int(emu: &mut emu::Emu, name: &str) {
    let addr = allocate_memory(emu, 4).unwrap_or(0);
    if addr != 0 {
        emu.maps.write_dword(addr, 0);
    }
    log_red!(emu, "msvcrt!{} => 0x{:x}", name, addr);
    emu.regs_mut().rax = addr;
}
