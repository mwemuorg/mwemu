use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::maps::buffers::BufferKind;
use crate::winapi::common::buffers;
use crate::winapi::common::trace::input_string;
use crate::winapi::environment_store::{self, Encoding, Environment, MAX_ENVIRONMENT_STRING};
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

fn set_result(emu: &mut Emu, result: Result<u64, u64>) {
    let call = WinApi(Stdcall);
    let value = match result {
        Ok(value) => value,
        Err(error) => {
            emu.set_last_error(error);
            0
        }
    };
    call.set_return(emu, value);
}

fn get_variable(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let name = api_try!(emu, call.arg(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
    let capacity = api_try!(emu, call.arg_u32(emu, 2)) as usize;
    log_red!(
        emu,
        "kernel32!{api} name={} output={output:#x} capacity={capacity}",
        input_string(emu, name, Encoding::from_wide(wide))
    );
    let result = get_value(emu, name, output, capacity, Encoding::from_wide(wide));
    call.cleanup_args(emu, 3);
    set_result(emu, result);
}

fn get_value(
    emu: &mut Emu,
    name: u64,
    output: u64,
    capacity: usize,
    encoding: Encoding,
) -> Result<u64, u64> {
    if name == 0 {
        return Err(ERROR_ENVVAR_NOT_FOUND);
    }
    let name = encoding.read(emu, name)?;
    let value = environment_store::process(emu)
        .get(&name)
        .ok_or(ERROR_ENVVAR_NOT_FOUND)?;
    let bytes = encoding.terminated(value);
    let required = bytes.len() / encoding.width();
    if capacity < required {
        return Ok(required as u64);
    }
    if output == 0
        || !emu.maps.validate_write_range(output, bytes.len() as u64)
        || !emu.maps.write_bytes(output, &bytes)
    {
        return Err(ERROR_NOACCESS);
    }
    Ok((required - 1) as u64)
}

fn set_variable(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let name = api_try!(emu, call.arg(emu, 0));
    let value = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "kernel32!{api} name={} value={}",
        input_string(emu, name, Encoding::from_wide(wide)),
        input_string(emu, value, Encoding::from_wide(wide))
    );
    let result = set_value(emu, name, value, Encoding::from_wide(wide));
    call.cleanup_args(emu, 2);
    set_result(emu, result);
}

fn set_value(emu: &mut Emu, name: u64, value: u64, encoding: Encoding) -> Result<u64, u64> {
    if name == 0 {
        return Err(ERROR_INVALID_PARAMETER);
    }
    let name = encoding.read(emu, name)?;
    if !environment_store::valid_name(&name) {
        return Err(ERROR_INVALID_PARAMETER);
    }
    let value = if value == 0 {
        None
    } else {
        Some(encoding.read(emu, value)?)
    };
    environment_store::process(emu).set(name, value);
    Ok(1)
}

fn expand(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let source = api_try!(emu, call.arg(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
    let capacity = api_try!(emu, call.arg_u32(emu, 2)) as usize;
    log_red!(
        emu,
        "kernel32!{api} source={} output={output:#x} capacity={capacity}",
        input_string(emu, source, Encoding::from_wide(wide))
    );
    let result = expand_value(emu, source, output, capacity, Encoding::from_wide(wide));
    call.cleanup_args(emu, 3);
    set_result(emu, result);
}

fn expand_value(
    emu: &mut Emu,
    source: u64,
    output: u64,
    capacity: usize,
    encoding: Encoding,
) -> Result<u64, u64> {
    let source = encoding.read(emu, source)?;
    let limit = emu.maps.max_alloc_size as usize / size_of::<u16>();
    let value = environment_store::process(emu)
        .expand(&source, limit)
        .ok_or(ERROR_NOT_ENOUGH_MEMORY)?;
    let bytes = encoding.terminated(&value);
    let required = bytes.len() / encoding.width();
    if required <= capacity
        && (output == 0
            || !emu.maps.validate_write_range(output, bytes.len() as u64)
            || !emu.maps.write_bytes(output, &bytes))
    {
        return Err(ERROR_NOACCESS);
    }
    Ok(required as u64)
}

fn get_strings(emu: &mut Emu, api: &str, encoding: Encoding) {
    log_red!(emu, "kernel32!{api}");
    let mut text = Vec::new();
    for (name, value) in &environment_store::process(emu).variables {
        text.extend(name);
        text.push(b'=' as u16);
        text.extend(value);
        text.push(0);
    }
    if text.is_empty() {
        text.push(0);
    }
    text.push(0);
    let bytes = encoding.encode(&text);
    let result = allocate_block(emu, &bytes);
    set_result(emu, result);
}

fn allocate_block(emu: &mut Emu, bytes: &[u8]) -> Result<u64, u64> {
    let block = buffers::allocate(
        emu,
        bytes.len() as u64,
        size_of::<u16>() as u64,
        0,
        BufferKind::Environment,
    )
    .ok_or(ERROR_NOT_ENOUGH_MEMORY)?;
    if !emu.maps.write_bytes(block, bytes) {
        buffers::free(emu, block, BufferKind::Environment);
        return Err(ERROR_NOACCESS);
    }
    Ok(block)
}

fn free_strings(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let block = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernel32!{api} block={block:#x}");
    let freed = block != 0 && buffers::free(emu, block, BufferKind::Environment);
    call.cleanup_args(emu, 1);
    set_result(
        emu,
        if freed {
            Ok(1)
        } else {
            Err(ERROR_INVALID_PARAMETER)
        },
    );
}

fn set_strings(emu: &mut Emu, api: &str, encoding: Encoding) {
    let call = WinApi(Stdcall);
    let block = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernel32!{api} block={block:#x}");
    let result = read_block(emu, block, encoding).map(|environment| {
        emu.environment = Some(environment);
        1
    });
    call.cleanup_args(emu, 1);
    set_result(emu, result);
}

fn read_block(emu: &Emu, mut address: u64, encoding: Encoding) -> Result<Environment, u64> {
    let start = address;
    let mut environment = Environment::default();
    loop {
        let line = encoding.read(emu, address)?;
        if line.is_empty() {
            return Ok(environment);
        }
        // Drive-current-directory entries may begin with '='.
        let separator = line
            .iter()
            .enumerate()
            .skip(1)
            .find(|(_, unit)| **unit == b'=' as u16)
            .map(|(index, _)| index)
            .ok_or(ERROR_INVALID_PARAMETER)?;
        let (name, value) = (&line[..separator], &line[separator + 1..]);
        environment.set(name.to_vec(), Some(value.to_vec()));
        address = address
            .checked_add(((line.len() + 1) * encoding.width()) as u64)
            .ok_or(ERROR_NOACCESS)?;
        if address - start > emu.maps.max_alloc_size || line.len() > MAX_ENVIRONMENT_STRING {
            return Err(ERROR_NOT_ENOUGH_MEMORY);
        }
    }
}

pub fn GetEnvironmentVariableA(emu: &mut Emu) {
    get_variable(emu, "GetEnvironmentVariableA", false);
}

pub fn GetEnvironmentVariableW(emu: &mut Emu) {
    get_variable(emu, "GetEnvironmentVariableW", true);
}

pub fn SetEnvironmentVariableA(emu: &mut Emu) {
    set_variable(emu, "SetEnvironmentVariableA", false);
}

pub fn SetEnvironmentVariableW(emu: &mut Emu) {
    set_variable(emu, "SetEnvironmentVariableW", true);
}

pub fn ExpandEnvironmentStringsA(emu: &mut Emu) {
    expand(emu, "ExpandEnvironmentStringsA", false);
}

pub fn ExpandEnvironmentStringsW(emu: &mut Emu) {
    expand(emu, "ExpandEnvironmentStringsW", true);
}

pub fn GetEnvironmentStrings(emu: &mut Emu) {
    get_strings(emu, "GetEnvironmentStrings", Encoding::Ansi);
}

pub fn GetEnvironmentStringsA(emu: &mut Emu) {
    get_strings(emu, "GetEnvironmentStringsA", Encoding::Ansi);
}

pub fn GetEnvironmentStringsW(emu: &mut Emu) {
    get_strings(emu, "GetEnvironmentStringsW", Encoding::Wide);
}

pub fn FreeEnvironmentStringsA(emu: &mut Emu) {
    free_strings(emu, "FreeEnvironmentStringsA");
}

pub fn FreeEnvironmentStringsW(emu: &mut Emu) {
    free_strings(emu, "FreeEnvironmentStringsW");
}

pub fn SetEnvironmentStringsW(emu: &mut Emu) {
    set_strings(emu, "SetEnvironmentStringsW", Encoding::Wide);
}

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "GetEnvironmentVariableA" => GetEnvironmentVariableA(emu),
        "GetEnvironmentVariableW" => GetEnvironmentVariableW(emu),
        "SetEnvironmentVariableA" => SetEnvironmentVariableA(emu),
        "SetEnvironmentVariableW" => SetEnvironmentVariableW(emu),
        "ExpandEnvironmentStringsA" => ExpandEnvironmentStringsA(emu),
        "ExpandEnvironmentStringsW" => ExpandEnvironmentStringsW(emu),
        "GetEnvironmentStrings" => GetEnvironmentStrings(emu),
        "GetEnvironmentStringsA" => GetEnvironmentStringsA(emu),
        "GetEnvironmentStringsW" => GetEnvironmentStringsW(emu),
        "FreeEnvironmentStringsA" => FreeEnvironmentStringsA(emu),
        "FreeEnvironmentStringsW" => FreeEnvironmentStringsW(emu),
        "SetEnvironmentStringsW" => SetEnvironmentStringsW(emu),
    }, handled(()) => true, fallback => false)
}
