use crate::api::windows::common::fallback;
use crate::api::windows::common::heap as heap_engine;
use crate::emu;
use crate::winapi::winapi64::kernel32;
use crate::windows::constants;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "OleInitialize" => OleInitialize(emu),
        "CoInitialize" => {
            log_red!(emu, "ole32!CoInitialize");
            emu.regs_mut().rax = constants::S_OK;
        }
        "CoInitializeEx" => CoInitializeEx(emu),
        "CoCreateInstance" => CoCreateInstance(emu),
        "CoTaskMemAlloc" => CoTaskMemAlloc(emu),
        "CoTaskMemFree" => CoTaskMemFree(emu),
        "CoUninitialize" => CoUninitialize(emu),
        "CoResumeClassObjects" => {
            log_red!(emu, "ole32!CoResumeClassObjects (stub)");
            emu.regs_mut().rax = constants::S_OK;
        }
        "CoAddRefServerProcess" => {
            log_red!(emu, "ole32!CoAddRefServerProcess (stub)");
            emu.regs_mut().rax = 1;
        }
        "CoReleaseServerProcess" => {
            log_red!(emu, "ole32!CoReleaseServerProcess (stub)");
            emu.regs_mut().rax = 0;
        }
        "CreateClassMoniker" => {
            log_red!(emu, "ole32!CreateClassMoniker (stub)");
            emu.regs_mut().rax = 0x80004001u64; // E_NOTIMPL
        }
        "GetRunningObjectTable" => {
            log_red!(emu, "ole32!GetRunningObjectTable (stub)");
            emu.regs_mut().rax = 0x80004001u64; // E_NOTIMPL
        }
        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}

fn OleInitialize(emu: &mut emu::Emu) {
    log_red!(emu, "ole32!OleInitialize");
    emu.regs_mut().rax = constants::S_OK;
}

pub fn CoInitializeEx(emu: &mut emu::Emu) {
    let co_init = emu.regs().rdx;
    log_red!(emu, "ole32!CoInitializeEx dwCoInit: 0x{:x}", co_init);
    emu.regs_mut().rax = constants::S_OK;
}

pub fn CoCreateInstance(emu: &mut emu::Emu) {
    let rclsid = emu.regs().rcx;
    let riid = emu.regs().r9;
    let ppv = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    log_red!(
        emu,
        "ole32!CoCreateInstance rclsid: 0x{:x} riid: 0x{:x} ppv: 0x{:x}",
        rclsid,
        riid,
        ppv
    );

    if ppv != 0 {
        emu.maps.write_qword(ppv, 0);
    }
    emu.regs_mut().rax = constants::REGDB_E_CLASSNOTREG;
}

pub fn CoTaskMemAlloc(emu: &mut emu::Emu) {
    let size = emu.regs().rcx;
    log_red!(emu, "ole32!CoTaskMemAlloc size: 0x{:x}", size);

    let addr = heap_engine::heap_allocate(emu, 0, size).unwrap_or(0);
    emu.regs_mut().rax = addr;
}

pub fn CoTaskMemFree(emu: &mut emu::Emu) {
    let pv = emu.regs().rcx;
    log_red!(emu, "ole32!CoTaskMemFree pv: 0x{:x}", pv);

    if pv != 0 {
        heap_engine::heap_free(emu, 0, pv);
    }
    emu.regs_mut().rax = 0;
}

pub fn CoUninitialize(emu: &mut emu::Emu) {
    log_red!(emu, "ole32!CoUninitialize");
    emu.regs_mut().rax = 0;
}
