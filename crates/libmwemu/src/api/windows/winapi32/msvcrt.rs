use crate::emu;
use crate::maps::mem64::Permission;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi32::kernel32;
//use crate::endpoint;

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

        _ => {
            if !emu.cfg.skip_unimplemented {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump(
                        emu,
                        emu.cfg.dump_filename.as_ref().unwrap(),
                    );
                }

                unimplemented!("atemmpt to call unimplemented API 0x{:x} {}", addr, api);
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
    let start_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("msvcrt!_initterm_e: error reading start pointer") as u64;
    let end_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("msvcrt!_initterm_e: error reading en pointer") as u64;

    log_red!(
        emu,
        "msvcrt!_initterm_e 0x{:x} - 0x{:x}",
        start_ptr,
        end_ptr
    );

    emu.regs_mut().rax = 0;
}

fn _initterm(emu: &mut emu::Emu) {
    let start_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("msvcrt!_initterm_e: error reading start pointer") as u64;
    let end_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("msvcrt!_initterm_e: error reading end pointer") as u64;

    log_red!(emu, "msvcrt!_initterm 0x{:x} - 0x{:x}", start_ptr, end_ptr);

    emu.regs_mut().rax = 0;
}

fn StrCmpCA(emu: &mut emu::Emu) {
    let str1_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("msvcrt!StrCmpA: error reading str1 pointer") as u64;
    let str2_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("msvcrt!StrCmpA: error reading str2 pointer") as u64;

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
    let filepath_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("msvcrt!fopen error reading filepath pointer") as u64;
    let mode_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("msvcrt!fopen error reading mode pointer") as u64;

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
    let buff_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("msvcrt!fwrite error reading buff_ptr") as u64;
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("msvcrt!fwrite error reading size");
    let nemb = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("msvcrt!fwrite error reading nemb");
    let file = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("msvcrt!fwrite error reading FILE *");

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
    let file = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("msvcrt!fflush error getting FILE *");

    let filename = helper::handler_get_uri(file as u64);

    log_red!(emu, "msvcrt!fflush `{}`", filename);

    //emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}

fn fclose(emu: &mut emu::Emu) {
    let file = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("msvcrt!fclose error getting FILE *");

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
            let addr = emu.maps.alloc(1024).expect("out of memory");
            emu.maps
                .create_map("args", addr, 1024, Permission::READ_WRITE)
                .expect("cannot create args map")
                .get_base()
        }
    };

    log_red!(emu, "msvcrt!__p___argc {}", args_base);

    emu.regs_mut().rax = args_base;
}

fn malloc(emu: &mut emu::Emu) {
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("msvcrt!malloc error reading size") as u64;

    if size > 0 {
        let base = emu.maps.alloc(size).expect("msvcrt!malloc out of memory");

        emu.maps
            .create_map(
                &format!("alloc_{:x}", base),
                base,
                size,
                Permission::READ_WRITE,
            )
            .expect("msvcrt!malloc cannot create map");

        log_red!(emu, "msvcrt!malloc sz: {} addr: 0x{:x}", size, base);

        emu.regs_mut().rax = base;
    } else {
        emu.regs_mut().rax = 0x1337; // weird msvcrt has to return a random unallocated pointer, and the program has to do free() on it
    }
}

fn __p__acmdln(emu: &mut emu::Emu) {
    log_red!(emu, "msvcrt!__p___argc");
    emu.regs_mut().rax = 0;
}

fn _onexit(emu: &mut emu::Emu) {
    let fptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("msvcrt!_onexit") as u64;

    log_red!(emu, "msvcrt!_onexit 0x{:x}", fptr);

    emu.regs_mut().rax = fptr;
}

fn _lock(emu: &mut emu::Emu) {
    let lock_num = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("msvcrt!_lock");

    log_red!(emu, "msvcrt!_lock {}", lock_num);

    emu.regs_mut().rax = 1;
}

fn free(emu: &mut emu::Emu) {
    let addr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("msvcrt!free error reading addr");

    log_red!(emu, "msvcrt!free 0x{:x}", addr);
}

fn realloc(emu: &mut emu::Emu) {
    let addr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("msvcrt!realloc error reading addr") as u64;
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("msvcrt!realloc error reading size") as u64;

    if addr == 0 {
        if size == 0 {
            emu.maps.dealloc(addr);
            emu.regs_mut().rax = 0;
            return;
        } else {
            let base = emu.maps.alloc(size).expect("msvcrt!malloc out of memory");

            emu.maps
                .create_map(
                    &format!("alloc_{:x}", base),
                    base,
                    size,
                    Permission::READ_WRITE,
                )
                .expect("msvcrt!malloc cannot create map");

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

    let mem = emu
        .maps
        .get_mem_by_addr_mut(addr)
        .expect("msvcrt!realloc error getting mem");
    let prev_size = mem.size();

    let new_addr = emu.maps.alloc(size).expect("msvcrt!realloc out of memory");

    emu.maps
        .create_map(
            &format!("alloc_{:x}", new_addr),
            new_addr,
            size,
            Permission::READ_WRITE,
        )
        .expect("msvcrt!realloc cannot create map");

    emu.maps.memcpy(new_addr, addr, prev_size);
    emu.maps.dealloc(addr);

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
    let str_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("msvcrt!strtok error reading str_ptr");

    let delim_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("msvcrt!strtok error reading delim");

    let str = emu.maps.read_string(str_ptr as u64);
    let delim = emu.maps.read_string(delim_ptr as u64);

    log_red!(emu, "msvcrt!strtok `{}` `{}`", str, delim);

    emu.regs_mut().rax = 0;
}

fn __set_app_type(emu: &mut emu::Emu) {
    let app_type = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("msvcrt!__set_app_type error reading app_type");

    log_red!(emu, "msvcrt!__set_app_type  app_type: 0x{:x}", app_type);
}

fn exit(emu: &mut emu::Emu) {
    let status = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("msvcrt!exit error reading param status.");
    log_red!(emu, "msvcrt!exit {}", status);
    emu.stop();
}

fn _errno(emu: &mut emu::Emu) {
    log_red!(emu, "msvcrt!_errno =0");
    emu.regs_mut().rax = 0;
}
