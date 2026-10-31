use crate::api::windows::common::heap as heap_engine;
use crate::emu;
use crate::serialization;
use crate::winapi::winapi32::kernel32;
use crate::winapi::winapi32::{arg, stdcall_ret};
use crate::windows::constants;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "OleInitialize" => OleInitialize(emu),
        "CoInitialize" => CoInitialize(emu),
        "CoInitializeEx" => CoInitializeEx(emu),
        "CoCreateInstance" => CoCreateInstance(emu),
        "CoTaskMemAlloc" => CoTaskMemAlloc(emu),
        "CoTaskMemFree" => CoTaskMemFree(emu),
        "CoUninitialize" => CoUninitialize(emu),
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

fn OleInitialize(emu: &mut emu::Emu) {
    log_red!(emu, "ole32!OleInitialize");
    emu.regs_mut().rax = constants::S_OK;
    stdcall_ret(emu, 1);
}

fn CoInitialize(emu: &mut emu::Emu) {
    log_red!(emu, "ole32!CoInitialize");
    emu.regs_mut().rax = constants::S_OK;
    stdcall_ret(emu, 1);
}

fn CoInitializeEx(emu: &mut emu::Emu) {
    let co_init = arg(emu, 1);
    log_red!(emu, "ole32!CoInitializeEx dwCoInit: 0x{:x}", co_init);
    emu.regs_mut().rax = constants::S_OK;
    stdcall_ret(emu, 2);
}

fn CoCreateInstance(emu: &mut emu::Emu) {
    let ppv = arg(emu, 4) as u64;
    log_red!(emu, "ole32!CoCreateInstance ppv: 0x{:x}", ppv);
    if ppv != 0 {
        emu.maps.write_dword(ppv, 0);
    }
    emu.regs_mut().rax = constants::REGDB_E_CLASSNOTREG;
    stdcall_ret(emu, 5);
}

fn CoTaskMemAlloc(emu: &mut emu::Emu) {
    let size = arg(emu, 0) as u64;
    log_red!(emu, "ole32!CoTaskMemAlloc size: 0x{:x}", size);
    emu.regs_mut().rax = heap_engine::heap_allocate(emu, 0, size).unwrap_or(0);
    stdcall_ret(emu, 1);
}

fn CoTaskMemFree(emu: &mut emu::Emu) {
    let pv = arg(emu, 0) as u64;
    log_red!(emu, "ole32!CoTaskMemFree pv: 0x{:x}", pv);
    if pv != 0 {
        heap_engine::heap_free(emu, 0, pv);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}

fn CoUninitialize(emu: &mut emu::Emu) {
    log_red!(emu, "ole32!CoUninitialize");
    emu.regs_mut().rax = 0;
}
