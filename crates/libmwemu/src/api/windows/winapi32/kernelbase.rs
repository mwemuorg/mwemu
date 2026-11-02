use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};
//use crate::debug::console;
//use crate::windows::constants;
//use crate::context32;
//use crate::peb32;
//use crate::windows::structures;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi32::kernel32;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "GetModuleHandleW" => GetModuleHandleW(emu),
        "LoadStringW" => LoadStringW(emu),
        "SetUnhandledExceptionFilter" => SetUnhandledExceptionFilter(emu),
        "_initterm" => _initterm(emu),
        "_initterm_e" => _initterm_e(emu),
        "LocalAlloc" => LocalAlloc(emu),
        "GetTokenInformation" => GetTokenInformation(emu),
        "PathCombineA" => PathCombineA(emu),
        "PathCombineW" => PathCombineW(emu),
        "IsCharAlphaNumericA" => IsCharAlphaNumericA(emu),

        _ => {
            if emu.cfg.skip_unimplemented {
                let nparams = emu.banzai.get_params(api);
                if nparams >= 0 {
                    for _ in 0..nparams {
                        emu.stack_pop32(false);
                    }
                }
                log::warn!(
                    "banzai: skipping unimplemented API 0x{:x} {} (nparams={}) at 0x{:x}",
                    addr,
                    api,
                    nparams,
                    emu.regs().rip
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

            log::error!("attempt to call unimplemented API 0x{:x} {}", addr, api);
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

/// kernelbase API ////
fn GetModuleHandleW(emu: &mut emu::Emu) {
    let mod_name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    let mod_name: String;

    if mod_name_ptr == 0 {
        mod_name = "self".to_string();
        emu.regs_mut().rax = match emu.maps.get_base() {
            Some(base) => base,
            None => helper::handler_create(&mod_name),
        }
    } else {
        mod_name = emu.maps.read_wide_string(mod_name_ptr).to_lowercase();
        let mod_mem = match emu.maps.get_mem2(&mod_name) {
            Some(m) => m,
            None => {
                emu.regs_mut().rax = 0;
                return;
            }
        };
        emu.regs_mut().rax = mod_mem.get_base();
    }

    log_red!(emu, "kernel32!GetModuleHandleW '{}'", mod_name);

    emu.stack_pop32(false);
}

fn LoadStringW(emu: &mut emu::Emu) {
    let hndl = emu.maps.read_dword(emu.regs().rsp).unwrap_or(0);
    let id = emu.maps.read_dword(emu.regs().rsp + 4).unwrap_or(0);
    let buff = emu.maps.read_dword(emu.regs().rsp + 8).unwrap_or(0);
    let len = emu.maps.read_dword(emu.regs().rsp + 12).unwrap_or(0);

    log_red!(emu, "kernelbase!LoadStringW {} 0x{}", id, buff);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}

fn _initterm(emu: &mut emu::Emu) {
    let ptr1 = emu.maps.read_dword(emu.regs().rsp).unwrap_or(0);
    let ptr2 = emu.maps.read_dword(emu.regs().rsp + 4).unwrap_or(0);
    log_red!(emu, "kernelbase!_initterm 0x{:x} 0x{:x}", ptr1, ptr2);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}

fn _initterm_e(emu: &mut emu::Emu) {
    let ptr1 = emu.maps.read_dword(emu.regs().rsp).unwrap_or(0);
    let ptr2 = emu.maps.read_dword(emu.regs().rsp + 4).unwrap_or(0);
    log_red!(emu, "kernelbase!_initterm_e 0x{:x} 0x{:x}", ptr1, ptr2);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}

fn SetUnhandledExceptionFilter(emu: &mut emu::Emu) {
    let ptr1 = emu.maps.read_dword(emu.regs().rsp).unwrap_or(0);

    log_red!(emu, "kernelbase!SetUnhandledExceptionFilter 0x{:x}", ptr1);

    emu.set_uef(ptr1 as u64);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}

fn LocalAlloc(emu: &mut emu::Emu) {
    // LocalAlloc(uFlags, uBytes) is equivalent to HeapAlloc(GetProcessHeap(), uFlags, uBytes)
    // push a dummy handle for the global heap
    emu.stack_push32(0x0);
    kernel32::HeapAlloc(emu);
}

fn GetTokenInformation(emu: &mut emu::Emu) {
    let token_information_class = arg(emu, 1);
    log_red!(
        emu,
        "kernelbase!GetTokenInformation class: 0x{:x}",
        token_information_class
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}

fn PathCombineA(emu: &mut emu::Emu) {
    let dst = arg(emu, 0) as u64;
    let dir = arg(emu, 1) as u64;
    let file = arg(emu, 2) as u64;
    let path1 = if dir != 0 {
        emu.maps.read_string(dir)
    } else {
        String::new()
    };
    let path2 = if file != 0 {
        emu.maps.read_string(file)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernelbase!PathCombineA path1: {} path2: {}",
        path1,
        path2
    );
    if dst != 0 && !path1.is_empty() && !path2.is_empty() {
        emu.maps.write_string(dst, &format!("{}\\{}", path1, path2));
    }
    emu.regs_mut().rax = dst;
    stdcall_ret(emu, 3);
}

fn PathCombineW(emu: &mut emu::Emu) {
    let dst = arg(emu, 0) as u64;
    let dir = arg(emu, 1) as u64;
    let file = arg(emu, 2) as u64;
    let path1 = if dir != 0 {
        emu.maps.read_wide_string(dir)
    } else {
        String::new()
    };
    let path2 = if file != 0 {
        emu.maps.read_wide_string(file)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernelbase!PathCombineW path1: {} path2: {}",
        path1,
        path2
    );
    if dst != 0 && !path1.is_empty() && !path2.is_empty() {
        emu.maps
            .write_wide_string(dst, &format!("{}\\{}", path1, path2));
    }
    emu.regs_mut().rax = dst;
    stdcall_ret(emu, 3);
}

fn IsCharAlphaNumericA(emu: &mut emu::Emu) {
    let c = arg(emu, 0) as u8 as char;
    log_red!(emu, "kernelbase!IsCharAlphaNumericA char: {}", c);
    emu.regs_mut().rax = c.is_ascii_alphanumeric() as u64;
    stdcall_ret(emu, 1);
}
