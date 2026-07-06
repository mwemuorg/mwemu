use crate::emu;
//use crate::debug::console;
//use crate::windows::constants;
//use crate::context32;
//use crate::peb32;
//use crate::windows::structures;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi32::kernel32;

use lazy_static::lazy_static;
use std::sync::Mutex;

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

lazy_static! {
    static ref COUNT_READ: Mutex<u32> = Mutex::new(0);
    static ref COUNT_WRITE: Mutex<u32> = Mutex::new(0);
    static ref LAST_ERROR: Mutex<u32> = Mutex::new(0);
}

/// kernelbase API ////

fn GetModuleHandleW(emu: &mut emu::Emu) {
    let mod_name_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetModuleHandleW cannot read mod_name_ptr") as u64;

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
    let hndl = emu
        .maps
        .read_dword(emu.regs().rsp)
        .expect("kernelbase!LoadStringW error reading param");
    let id = emu
        .maps
        .read_dword(emu.regs().rsp + 4)
        .expect("kernelbase!LoadStringW error reading param");
    let buff = emu
        .maps
        .read_dword(emu.regs().rsp + 8)
        .expect("kernelbase!LoadStringW error reading param");
    let len = emu
        .maps
        .read_dword(emu.regs().rsp + 12)
        .expect("kernelbase!LoadStringW error reading param");

    log_red!(emu, "kernelbase!LoadStringW {} 0x{}", id, buff);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}

fn _initterm(emu: &mut emu::Emu) {
    let ptr1 = emu
        .maps
        .read_dword(emu.regs().rsp)
        .expect("kernelbase!_initterm error reading param");
    let ptr2 = emu
        .maps
        .read_dword(emu.regs().rsp + 4)
        .expect("kernelbase!_initterm error reading param");
    log_red!(emu, "kernelbase!_initterm 0x{:x} 0x{:x}", ptr1, ptr2);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}

fn _initterm_e(emu: &mut emu::Emu) {
    let ptr1 = emu
        .maps
        .read_dword(emu.regs().rsp)
        .expect("kernelbase!_initterm_e error reading param");
    let ptr2 = emu
        .maps
        .read_dword(emu.regs().rsp + 4)
        .expect("kernelbase!_initterm_e error reading param");
    log_red!(emu, "kernelbase!_initterm_e 0x{:x} 0x{:x}", ptr1, ptr2);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}

fn SetUnhandledExceptionFilter(emu: &mut emu::Emu) {
    let ptr1 = emu
        .maps
        .read_dword(emu.regs().rsp)
        .expect("kernelbase!SetUnhandledExceptionFilter error reading param");

    log_red!(emu, "kernelbase!SetUnhandledExceptionFilter 0x{:x}", ptr1);

    emu.set_uef(ptr1 as u64);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}

fn LocalAlloc(emu: &mut emu::Emu) {
    let flags = emu
        .maps
        .read_dword(emu.regs().rsp)
        .expect("kernelbase!LocalAlloc error reading param");
    let size = emu
        .maps
        .read_dword(emu.regs().rsp + 4)
        .expect("kernelbase!LocalAlloc error reading param") as u64;

    let addr = emu.maps.alloc(size).unwrap_or_default();

    log_red!(emu, "kernelbase!LocalAlloc {} =0x{:x}", size, addr);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = addr;
}
