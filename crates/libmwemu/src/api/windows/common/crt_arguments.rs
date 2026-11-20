//! Lazy, unexpanded CRT startup arguments. Guest maps retain the pointer cells in snapshots.
use crate::api::abi::windows::{
    CallingConvention::{Cdecl, Stdcall},
    WindowsAbi::WinApi,
};
use crate::emu::Emu;
use crate::exception::types::ExceptionType;
use crate::maps::mem64::Permission;
use crate::winapi::common::crt::{self, Runtime};
use crate::winapi::common::crt_environment;
use crate::winapi::common::memory::{pointer_size, read_array, read_pointer, write_pointer};
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::*;
use crate::windows::peb::command_line;

const TAB: u16 = b'\t' as u16;
const SPACE: u16 = b' ' as u16;
const ARGUMENT_CELL_COUNT: usize = 3;
const MAX_COMMAND_LINE_UNITS: usize = 32767;

// The executable is configured separately; only the remaining arguments use these quote rules.
fn parse_arguments(text: &str) -> Vec<Vec<u16>> {
    let input: Vec<_> = text.encode_utf16().take_while(|&unit| unit != 0).collect();
    let mut result = Vec::new();
    let mut index = 0;
    while index < input.len() {
        while index < input.len() && matches!(input[index], TAB | SPACE) {
            index += 1;
        }
        if index == input.len() {
            break;
        }
        let mut argument = Vec::new();
        let mut quoted = false;
        while index < input.len() && (quoted || !matches!(input[index], TAB | SPACE)) {
            let start = index;
            while index < input.len() && input[index] == b'\\' as u16 {
                index += 1;
            }
            let slashes = index - start;
            if input.get(index) == Some(&(b'"' as u16)) {
                argument.extend(std::iter::repeat_n(b'\\' as u16, slashes / 2));
                if !slashes.is_multiple_of(2) {
                    argument.push(b'"' as u16);
                } else if quoted && input.get(index + 1) == Some(&(b'"' as u16)) {
                    argument.push(b'"' as u16);
                    index += 1;
                } else {
                    quoted = !quoted;
                }
                index += 1;
            } else {
                argument.extend(std::iter::repeat_n(b'\\' as u16, slashes));
                if index < input.len() && (quoted || !matches!(input[index], TAB | SPACE)) {
                    argument.push(input[index]);
                    index += 1;
                }
            }
        }
        result.push(argument);
    }
    result
}

fn startup_arguments(emu: &Emu) -> Result<Vec<Vec<u16>>, i32> {
    let name = if emu.cfg.filename.is_empty() {
        &emu.filename
    } else {
        &emu.cfg.filename
    };
    let name = if name.is_empty() {
        "program.exe"
    } else {
        name.as_str()
    };
    if name.encode_utf16().count() + emu.cfg.arguments.encode_utf16().count()
        >= MAX_COMMAND_LINE_UNITS
    {
        return Err(EINVAL);
    }
    let mut arguments = vec![name.encode_utf16().take_while(|&unit| unit != 0).collect()];
    arguments.extend(parse_arguments(&emu.cfg.arguments));
    Ok(arguments)
}

fn ensure_arguments(emu: &mut Emu, runtime: Runtime) -> Result<u64, i32> {
    let name = format!("crt_arguments_{}", runtime.library());
    if let Some(map) = emu.maps.get_map_by_name(&name) {
        return Ok(map.get_base());
    }
    let arguments = startup_arguments(emu)?;
    let width = pointer_size(emu) as usize;
    let mut bytes = vec![0; ARGUMENT_CELL_COUNT * width];
    bytes[..size_of::<u32>()].copy_from_slice(&(arguments.len() as u32).to_le_bytes());
    let mut pointers = Vec::new();
    for (index, encoding) in [Encoding::Ansi, Encoding::Wide].into_iter().enumerate() {
        bytes.resize(bytes.len().next_multiple_of(width), 0);
        let table = bytes.len();
        pointers.push(((index + 1) * width, table));
        bytes.resize(table + (arguments.len() + 1) * width, 0);
        for (argument, units) in arguments.iter().enumerate() {
            pointers.push((table + argument * width, bytes.len()));
            bytes.extend(encoding.terminated(units));
        }
    }
    let base = emu.maps.alloc(bytes.len() as u64).ok_or(ENOMEM)?;
    for (cell, offset) in pointers {
        bytes[cell..cell + width].copy_from_slice(&(base + offset as u64).to_le_bytes()[..width]);
    }
    let map = emu
        .maps
        .create_map(&name, base, bytes.len() as u64, Permission::READ_WRITE)
        .map_err(|_| ENOMEM)?;
    map.write_bytes(base, &bytes);
    Ok(base)
}

fn report_error(emu: &mut Emu, runtime: Runtime, error: i32, result: u64) {
    let call = WinApi(Cdecl);
    let Some(address) = crt::errno_address(emu, runtime) else {
        call.set_return(emu, result);
        return;
    };
    if !emu.maps.write_bytes(address, &error.to_le_bytes()) {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    call.set_return(emu, result);
}

fn pointer(emu: &mut Emu, runtime: Runtime, api: &str) {
    let call = WinApi(Cdecl);
    let cell = match api {
        "__p___argc" => 0,
        "__p___argv" => 1,
        _ => 2,
    };
    match ensure_arguments(emu, runtime) {
        Ok(base) => call.set_return(emu, base + cell * pointer_size(emu)),
        Err(error) => report_error(emu, runtime, error, 0),
    }
}

fn configure(emu: &mut Emu, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let mode = api_try!(emu, call.arg_i32(emu, 0));
    let error = match mode {
        CRT_ARGV_NO_ARGUMENTS => {
            call.set_return(emu, 0);
            return;
        }
        CRT_ARGV_UNEXPANDED_ARGUMENTS => None,
        CRT_ARGV_EXPANDED_ARGUMENTS if !emu.cfg.arguments.contains(['*', '?']) => None,
        CRT_ARGV_EXPANDED_ARGUMENTS => Some(ENOSYS),
        _ => Some(EINVAL),
    };
    let result = error.map_or_else(|| ensure_arguments(emu, runtime).map(|_| ()), Err);
    match result {
        Ok(()) => call.set_return(emu, 0),
        Err(error) => report_error(emu, runtime, error, error as u64),
    }
}

fn getmainargs(emu: &mut Emu, runtime: Runtime, wide: bool) {
    let call = WinApi(Cdecl);
    let argc = api_try!(emu, call.arg(emu, 0));
    let argv = api_try!(emu, call.arg(emu, 1));
    let environment = api_try!(emu, call.arg(emu, 2));
    let wildcard = api_try!(emu, call.arg_i32(emu, 3));
    let startup = api_try!(emu, call.arg(emu, 4));
    if argc == 0 || argv == 0 || environment == 0 {
        report_error(emu, runtime, EINVAL, u32::MAX as u64);
        return;
    }
    let width = pointer_size(emu);
    if !emu.maps.validate_write_range(argc, size_of::<u32>() as u64)
        || !emu.maps.validate_write_range(argv, width)
        || !emu.maps.validate_write_range(environment, width)
        || (startup != 0 && read_array::<4>(emu, startup).is_none())
    {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    if wildcard != 0 && emu.cfg.arguments.contains(['*', '?']) {
        report_error(emu, runtime, ENOSYS, u32::MAX as u64);
        return;
    }
    let base = match ensure_arguments(emu, runtime) {
        Ok(base) => base,
        Err(error) => {
            report_error(emu, runtime, error, u32::MAX as u64);
            return;
        }
    };
    let Some(count) = read_array::<4>(emu, base) else {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    };
    let Some(arguments) = read_pointer(emu, base + width * (1 + wide as u64)) else {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    };
    let Some(env) = crt_environment::initial_array(emu, runtime, wide) else {
        return;
    };
    if !emu.maps.write_bytes(argc, &count)
        || !write_pointer(emu, argv, arguments)
        || !write_pointer(emu, environment, env)
    {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    call.set_return(emu, 0);
}

pub(crate) fn dispatch(api: &str, emu: &mut Emu, runtime: Runtime) -> bool {
    match api {
        "__p__acmdln" | "__p__wcmdln" => {
            log_red!(emu, "{}!{api}", runtime.library());
            command_cell(emu, runtime, api);
        }
        "__p___argc" | "__p___argv" | "__p___wargv" => {
            log_red!(emu, "{}!{api}", runtime.library());
            pointer(emu, runtime, api);
        }
        "_configure_narrow_argv" | "_configure_wide_argv" => {
            log_red!(emu, "{}!{api}", runtime.library());
            configure(emu, runtime);
        }
        "__getmainargs" | "__wgetmainargs" => {
            log_red!(emu, "{}!{api}", runtime.library());
            getmainargs(emu, runtime, api == "__wgetmainargs");
        }
        _ => return false,
    }
    true
}

pub(crate) fn command_string(emu: &mut Emu, wide: bool) -> Option<u64> {
    let name = if wide {
        "command_line"
    } else {
        "command_line_ansi"
    };
    if let Some(map) = emu.maps.get_map_by_name(name) {
        return Some(map.get_base());
    }
    let mut units = if !wide && let Some(map) = emu.maps.get_map_by_name("command_line") {
        Encoding::Wide.read(emu, map.get_base()).ok()?
    } else {
        let image = if emu.cfg.filename.is_empty() {
            &emu.cfg.exe_name
        } else {
            &emu.cfg.filename
        };
        command_line(image, &emu.cfg.arguments)
            .encode_utf16()
            .collect()
    };
    if units.len() >= MAX_COMMAND_LINE_UNITS {
        return None;
    }
    units.push(0);
    let bytes = if wide { Encoding::Wide } else { Encoding::Ansi }.encode(&units);
    let len = bytes.len() as u64;
    let base = emu.maps.alloc(len)?;
    emu.maps
        .create_map(name, base, len, Permission::READ_WRITE)
        .ok()?;
    emu.maps.write_bytes(base, &bytes).then_some(base)
}

pub fn GetCommandLineA(emu: &mut Emu) {
    get_command_line(emu, false);
}
pub fn GetCommandLineW(emu: &mut Emu) {
    get_command_line(emu, true);
}
fn get_command_line(emu: &mut Emu, wide: bool) {
    let call = WinApi(Stdcall);
    log_red!(emu, "GetCommandLine{}", if wide { "W" } else { "A" });
    let value = command_string(emu, wide).unwrap_or(0);
    call.set_return(emu, value);
}

fn command_cell(emu: &mut Emu, runtime: Runtime, api: &str) {
    let call = WinApi(Cdecl);
    let key = format!("{}_{}", runtime.library(), api);
    if let Some(map) = emu.maps.get_map_by_name(&key) {
        call.set_return(emu, map.get_base());
        return;
    }
    let Some(value) = command_string(emu, api == "__p__wcmdln") else {
        report_error(emu, runtime, ENOMEM, 0);
        return;
    };
    let width = pointer_size(emu);
    let Some(base) = emu.maps.alloc(width) else {
        report_error(emu, runtime, ENOMEM, 0);
        return;
    };
    if emu
        .maps
        .create_map(&key, base, width, Permission::READ_WRITE)
        .is_err()
        || !write_pointer(emu, base, value)
    {
        report_error(emu, runtime, ENOMEM, 0);
        return;
    }
    call.set_return(emu, base);
}
