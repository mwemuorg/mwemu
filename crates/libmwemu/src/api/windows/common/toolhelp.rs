//! Enumerates the guest process, loaded images, and live emulated threads.
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::emu::object_handle::toolhelp::{SnapshotModule, ToolhelpSnapshot};
use crate::winapi::common::memory::pointer_size;
use crate::winapi::common::{modules, process, strings};
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::constants::files::MAX_PATH;
use crate::windows::constants::process::*;

const MODULE_NAME_CAPACITY: usize = MAX_MODULE_NAME32 + 1;
const MODULE_USAGE_UNKNOWN: u32 = 0xffff;
const BASE_PRIORITY_NORMAL: u32 = 8;

#[derive(Clone, Copy)]
enum List {
    Process,
    Module,
    Thread,
}

pub fn CreateToolhelp32Snapshot(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let flags = api_try!(emu, call.arg_u32(emu, 0));
    let pid = api_try!(emu, call.arg_u32(emu, 1));
    let result = capture(emu, flags, pid);
    let handle = match result {
        Ok(snapshot) => emu.handle_management.insert_toolhelp_snapshot(snapshot) as u64,
        Err(error) => {
            emu.set_last_error(error);
            INVALID_HANDLE_VALUE_64
        }
    };
    log_red!(
        emu,
        "kernel32!CreateToolhelp32Snapshot flags=0x{:x} pid={} => 0x{:x}",
        flags,
        pid,
        handle
    );
    call.cleanup_args(emu, 2);
    call.set_return(emu, handle);
}

fn capture(emu: &Emu, flags: u32, pid: u32) -> Result<ToolhelpSnapshot, u64> {
    let allowed = TH32CS_SNAPALL | TH32CS_SNAPMODULE32 | TH32CS_INHERIT;
    if flags & !allowed != 0 || flags & !TH32CS_INHERIT == 0 {
        return Err(ERROR_INVALID_PARAMETER);
    }
    // Heap walking has no model yet; do not claim a complete SNAPALL snapshot.
    if flags & TH32CS_SNAPHEAPLIST != 0 {
        return Err(ERROR_CALL_NOT_IMPLEMENTED);
    }
    let owner = process::current_process_id(emu) as u32;
    let module_flags = TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32;
    if flags & module_flags != 0 && pid != 0 && pid != owner {
        return Err(ERROR_INVALID_PARAMETER);
    }
    let threads: Vec<_> = emu
        .threads
        .iter()
        .filter(|thread| thread.exit_value.is_none())
        .map(|thread| thread.id as u32)
        .collect();
    let mut snapshot = ToolhelpSnapshot {
        owner,
        process: (flags & TH32CS_SNAPPROCESS != 0).then(|| (owner, emu.cfg.exe_name.clone())),
        process_threads: threads.len() as u32,
        threads: if flags & TH32CS_SNAPTHREAD != 0 {
            threads
        } else {
            Vec::new()
        },
        ..ToolhelpSnapshot::default()
    };
    let native_modules = flags & TH32CS_SNAPMODULE != 0
        || (!emu.cfg.arch.is_64bits() && flags & TH32CS_SNAPMODULE32 != 0);
    if native_modules {
        if emu.base != 0 {
            snapshot.modules.push(module(
                emu,
                emu.base,
                &emu.cfg.exe_name,
                modules::main_module_path(emu),
            ));
        }
        for loaded in emu.export_indexes.iter_ordered() {
            if loaded.base != emu.base {
                let path = modules::module_path(emu, loaded.base).ok_or(ERROR_MOD_NOT_FOUND)?;
                snapshot
                    .modules
                    .push(module(emu, loaded.base, &loaded.module_name, path));
            }
        }
    }
    Ok(snapshot)
}

fn module(emu: &Emu, base: u64, name: &str, path: String) -> SnapshotModule {
    let size = emu
        .maps
        .find_pe_image_info(base)
        .map(|(_, size)| size)
        .or_else(|| emu.maps.get_mem_by_addr(base).map(|map| map.size() as u64))
        .unwrap_or(0);
    SnapshotModule {
        base,
        size: size as u32,
        name: name.to_owned(),
        path,
    }
}

pub fn Process32First(emu: &mut Emu) {
    enumerate(emu, "Process32First", List::Process, true, false);
}
pub fn Process32FirstW(emu: &mut Emu) {
    enumerate(emu, "Process32FirstW", List::Process, true, true);
}
pub fn Process32Next(emu: &mut Emu) {
    enumerate(emu, "Process32Next", List::Process, false, false);
}
pub fn Process32NextW(emu: &mut Emu) {
    enumerate(emu, "Process32NextW", List::Process, false, true);
}
pub fn Module32First(emu: &mut Emu) {
    enumerate(emu, "Module32First", List::Module, true, false);
}
pub fn Module32FirstW(emu: &mut Emu) {
    enumerate(emu, "Module32FirstW", List::Module, true, true);
}
pub fn Module32Next(emu: &mut Emu) {
    enumerate(emu, "Module32Next", List::Module, false, false);
}
pub fn Module32NextW(emu: &mut Emu) {
    enumerate(emu, "Module32NextW", List::Module, false, true);
}
pub fn Thread32First(emu: &mut Emu) {
    enumerate(emu, "Thread32First", List::Thread, true, false);
}
pub fn Thread32Next(emu: &mut Emu) {
    enumerate(emu, "Thread32Next", List::Thread, false, false);
}

fn enumerate(emu: &mut Emu, api: &str, list: List, first: bool, wide: bool) {
    let call = WinApi(Stdcall);
    let snapshot = api_try!(emu, call.arg(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
    let result = write_entry(emu, snapshot, output, list, first, wide);
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    log_red!(
        emu,
        "kernel32!{} snapshot=0x{:x} entry=0x{:x} => {:?}",
        api,
        snapshot,
        output,
        result
    );
    call.cleanup_args(emu, 2);
    call.set_return(emu, u64::from(result.is_ok()));
}

fn write_entry(
    emu: &mut Emu,
    handle: u64,
    output: u64,
    list: List,
    first: bool,
    wide: bool,
) -> Result<(), u64> {
    let pointer = pointer_size(emu) as usize;
    let snapshot = emu
        .handle_management
        .toolhelp_snapshot(handle)
        .ok_or(ERROR_INVALID_HANDLE)?;
    let index = if first {
        0
    } else {
        match list {
            List::Process => snapshot.process_next,
            List::Module => snapshot.module_next,
            List::Thread => snapshot.thread_next,
        }
    };
    let bytes = match list {
        List::Process => snapshot
            .process
            .as_ref()
            .filter(|_| index == 0)
            .map(|(pid, name)| process_entry(*pid, name, snapshot.process_threads, pointer, wide)),
        List::Module => snapshot
            .modules
            .get(index)
            .map(|entry| module_entry(entry, snapshot.owner, pointer, wide)),
        List::Thread => snapshot
            .threads
            .get(index)
            .map(|tid| thread_entry(*tid, snapshot.owner)),
    }
    .ok_or(ERROR_NO_MORE_FILES)?;
    if output == 0 {
        return Err(ERROR_INVALID_PARAMETER);
    }
    if emu.maps.read_dword(output) != Some(bytes.len() as u32) {
        return Err(ERROR_BAD_LENGTH);
    }
    if !emu.maps.write_bytes(output, &bytes) {
        return Err(ERROR_NOACCESS);
    }
    // Advance only after the complete guest output has been written.
    if let Some(snapshot) = emu.handle_management.toolhelp_snapshot_mut(handle) {
        match list {
            List::Process => snapshot.process_next = index + 1,
            List::Module => snapshot.module_next = index + 1,
            List::Thread => snapshot.thread_next = index + 1,
        }
    }
    Ok(())
}

fn process_entry(pid: u32, name: &str, threads: u32, pointer: usize, wide: bool) -> Vec<u8> {
    let mut bytes: Vec<u8> = [0u32, 0, pid]
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect();
    align(&mut bytes, pointer);
    bytes.resize(bytes.len() + pointer, 0); // th32DefaultHeapID
    bytes.extend(
        [0, threads, 0, BASE_PRIORITY_NORMAL, 0]
            .into_iter()
            .flat_map(u32::to_le_bytes),
    );
    fixed_text(&mut bytes, name, MAX_PATH, wide);
    finish_entry(bytes, pointer)
}

fn module_entry(module: &SnapshotModule, pid: u32, pointer: usize, wide: bool) -> Vec<u8> {
    let mut bytes: Vec<u8> = [0u32, 1, pid, MODULE_USAGE_UNKNOWN, MODULE_USAGE_UNKNOWN]
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect();
    align(&mut bytes, pointer);
    bytes.extend_from_slice(&module.base.to_le_bytes()[..pointer]);
    bytes.extend_from_slice(&module.size.to_le_bytes());
    align(&mut bytes, pointer);
    bytes.extend_from_slice(&module.base.to_le_bytes()[..pointer]);
    fixed_text(&mut bytes, &module.name, MODULE_NAME_CAPACITY, wide);
    fixed_text(&mut bytes, &module.path, MAX_PATH, wide);
    finish_entry(bytes, pointer)
}

fn thread_entry(tid: u32, pid: u32) -> Vec<u8> {
    let bytes: Vec<u8> = [0u32, 0, tid, pid, BASE_PRIORITY_NORMAL, 0, 0]
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect();
    finish_entry(bytes, size_of::<u32>())
}

fn fixed_text(bytes: &mut Vec<u8>, value: &str, capacity: usize, wide: bool) {
    let unit = if wide {
        size_of::<u16>()
    } else {
        size_of::<u8>()
    };
    let text = strings::encode_text(value, wide);
    let count = text.len().min((capacity - 1) * unit);
    let end = bytes.len() + capacity * unit;
    bytes.extend_from_slice(&text[..count]);
    bytes.resize(end, 0);
}

fn align(bytes: &mut Vec<u8>, alignment: usize) {
    bytes.resize(bytes.len().next_multiple_of(alignment), 0);
}

fn finish_entry(mut bytes: Vec<u8>, alignment: usize) -> Vec<u8> {
    align(&mut bytes, alignment);
    let size = (bytes.len() as u32).to_le_bytes();
    bytes[..size.len()].copy_from_slice(&size);
    bytes
}
