//! CRT environments are runtime-owned copies of the process environment.
use crate::winapi::common::trace::input_string;
use serde::{Deserialize, Serialize};

use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::exception::types::ExceptionType;
use crate::maps::buffers::BufferKind;
use crate::winapi::common::buffers;
use crate::winapi::common::crt::{self, Runtime};
use crate::winapi::common::memory::{pointer_size, write_pointer};
use crate::winapi::environment_store::{self, Encoding, Environment};
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::*;
use crate::windows::constants::errors::ERROR_NOACCESS;

#[derive(Clone, Default, Serialize, Deserialize)]
struct Array {
    address: u64,
    values: Vec<u64>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
struct EncodedEnvironment {
    initial: Array,
    current: Array,
    cell: u64,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub(crate) struct CrtEnvironment {
    variables: Option<Environment>,
    initial: Environment,
    encodings: [EncodedEnvironment; 2],
}

#[derive(Clone, Copy)]
enum EnvironmentError {
    Invalid,
    NoMemory,
    Access,
}
type EnvironmentResult<T> = Result<T, EnvironmentError>;

fn read(emu: &Emu, pointer: u64, wide: bool) -> EnvironmentResult<Vec<u16>> {
    if pointer == 0 {
        return Err(EnvironmentError::Invalid);
    }
    Encoding::from_wide(wide)
        .read(emu, pointer)
        .map_err(|error| {
            if error == ERROR_NOACCESS {
                EnvironmentError::Access
            } else {
                EnvironmentError::Invalid
            }
        })
}

fn with_environment<T>(
    emu: &mut Emu,
    runtime: Runtime,
    operation: impl FnOnce(&mut Emu, &mut CrtEnvironment) -> EnvironmentResult<T>,
) -> EnvironmentResult<T> {
    let index = match runtime {
        Runtime::Msvcrt => 0,
        Runtime::Ucrt => 1,
    };
    let mut state = std::mem::take(&mut emu.crt_environments[index]);
    if state.variables.is_none() {
        state.initial = environment_store::process(emu).clone();
        state.variables = Some(state.initial.clone());
    }
    let result = operation(emu, &mut state);
    emu.crt_environments[index] = state;
    result
}

fn allocate_array(
    emu: &mut Emu,
    environment: &Environment,
    wide: bool,
) -> EnvironmentResult<Array> {
    let encoding = Encoding::from_wide(wide);
    let width = pointer_size(emu) as usize;
    let table_size = (environment.variables.len() + 1)
        .checked_mul(width)
        .ok_or(EnvironmentError::NoMemory)?;
    let mut bytes = vec![0; table_size];
    let mut starts = Vec::new();
    let mut values = Vec::new();
    for (name, value) in &environment.variables {
        starts.push(bytes.len());
        bytes.extend(encoding.encode(name));
        bytes.extend(encoding.encode(&[b'=' as u16]));
        values.push(bytes.len() as u64);
        bytes.extend(encoding.terminated(value));
        if bytes.len() as u64 > emu.maps.max_alloc_size {
            return Err(EnvironmentError::NoMemory);
        }
    }
    let address = buffers::allocate(emu, bytes.len() as u64, width as u64, 0, BufferKind::Crt)
        .ok_or(EnvironmentError::NoMemory)?;
    for (index, offset) in starts.iter().enumerate() {
        let pointer = (address + *offset as u64).to_le_bytes();
        bytes[index * width..(index + 1) * width].copy_from_slice(&pointer[..width]);
    }
    if !emu.maps.write_bytes(address, &bytes) {
        buffers::free(emu, address, BufferKind::Crt);
        return Err(EnvironmentError::Access);
    }
    for value in &mut values {
        *value += address;
    }
    Ok(Array { address, values })
}

fn ensure_encoding(emu: &mut Emu, state: &mut CrtEnvironment, wide: bool) -> EnvironmentResult<()> {
    let encoded = &mut state.encodings[usize::from(wide)];
    if encoded.cell != 0 {
        return Ok(());
    }
    let variables = state.variables.as_ref().ok_or(EnvironmentError::Invalid)?;
    let initial = allocate_array(emu, &state.initial, wide)?;
    let current = if variables == &state.initial {
        initial.clone()
    } else {
        match allocate_array(emu, variables, wide) {
            Ok(array) => array,
            Err(error) => {
                buffers::free(emu, initial.address, BufferKind::Crt);
                return Err(error);
            }
        }
    };
    let width = pointer_size(emu);
    let cell = buffers::allocate(emu, width, width, 0, BufferKind::Crt);
    if cell.is_none_or(|cell| !write_pointer(emu, cell, current.address)) {
        if let Some(cell) = cell {
            buffers::free(emu, cell, BufferKind::Crt);
        }
        if current.address != initial.address {
            buffers::free(emu, current.address, BufferKind::Crt);
        }
        buffers::free(emu, initial.address, BufferKind::Crt);
        return Err(EnvironmentError::NoMemory);
    }
    *encoded = EncodedEnvironment {
        initial,
        current,
        cell: cell.unwrap_or(0),
    };
    Ok(())
}

fn report_error(emu: &mut Emu, runtime: Runtime, error: EnvironmentError, failure: u64) {
    let call = WinApi(Cdecl);
    if matches!(error, EnvironmentError::Access) {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    let errno = match error {
        EnvironmentError::NoMemory => ENOMEM,
        _ => EINVAL,
    };
    if let Some(address) = crt::errno_address(emu, runtime)
        && !emu.maps.write_bytes(address, &errno.to_le_bytes())
    {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    call.set_return(emu, failure);
}

fn initialize(emu: &mut Emu, runtime: Runtime, api: &str, wide: bool) {
    let call = WinApi(Cdecl);
    log_red!(emu, "{}!{api}", runtime.library());
    match with_environment(emu, runtime, |emu, state| ensure_encoding(emu, state, wide)) {
        Ok(()) => call.set_return(emu, 0),
        Err(error) => report_error(emu, runtime, error, u32::MAX as u64),
    }
}

fn environment_pointer(emu: &mut Emu, runtime: Runtime, api: &str, wide: bool, initial: bool) {
    let call = WinApi(Cdecl);
    log_red!(emu, "{}!{api}", runtime.library());
    match with_environment(emu, runtime, |emu, state| {
        ensure_encoding(emu, state, wide)?;
        let encoded = &state.encodings[usize::from(wide)];
        Ok(if initial {
            encoded.initial.address
        } else {
            encoded.cell
        })
    }) {
        Ok(pointer) => call.set_return(emu, pointer),
        Err(error) => report_error(emu, runtime, error, 0),
    }
}

pub(crate) fn initial_array(emu: &mut Emu, runtime: Runtime, wide: bool) -> Option<u64> {
    match with_environment(emu, runtime, |emu, state| {
        ensure_encoding(emu, state, wide)?;
        Ok(state.encodings[usize::from(wide)].initial.address)
    }) {
        Ok(address) => Some(address),
        Err(error) => {
            report_error(emu, runtime, error, u32::MAX as u64);
            None
        }
    }
}

fn getenv(emu: &mut Emu, runtime: Runtime, api: &str, wide: bool) {
    let call = WinApi(Cdecl);
    let name = api_try!(emu, call.arg(emu, 0));
    let name = read(emu, name, wide);
    log_red!(
        emu,
        "{}!{api} name={:?}",
        runtime.library(),
        name.as_ref()
            .ok()
            .map(|units| String::from_utf16_lossy(units))
    );
    let result = name.and_then(|name| {
        with_environment(emu, runtime, |emu, state| {
            ensure_encoding(emu, state, wide)?;
            let variables = state.variables.as_ref().ok_or(EnvironmentError::Invalid)?;
            Ok(variables
                .position(&name)
                .and_then(|i| state.encodings[usize::from(wide)].current.values.get(i))
                .copied()
                .unwrap_or(0))
        })
    });
    match result {
        Ok(pointer) => call.set_return(emu, pointer),
        Err(error) => report_error(emu, runtime, error, 0),
    }
}

fn replace_variables(
    emu: &mut Emu,
    state: &mut CrtEnvironment,
    variables: Environment,
    process_variables: Environment,
) -> EnvironmentResult<()> {
    let width = pointer_size(emu);
    if state
        .encodings
        .iter()
        .any(|encoding| encoding.cell != 0 && !emu.maps.validate_write_range(encoding.cell, width))
    {
        return Err(EnvironmentError::Access);
    }
    let mut replacements = Vec::new();
    for (index, encoded) in state.encodings.iter().enumerate() {
        if encoded.cell == 0 {
            continue;
        }
        match allocate_array(emu, &variables, index == 1) {
            Ok(array) => replacements.push((index, array)),
            Err(error) => {
                for (_, array) in replacements {
                    buffers::free(emu, array.address, BufferKind::Crt);
                }
                return Err(error);
            }
        }
    }
    if let Err(error) = environment_store::replace(emu, process_variables) {
        for (_, array) in replacements {
            buffers::free(emu, array.address, BufferKind::Crt);
        }
        return Err(if error == ERROR_NOACCESS {
            EnvironmentError::Access
        } else {
            EnvironmentError::NoMemory
        });
    }
    for (index, array) in replacements {
        let encoded = &mut state.encodings[index];
        if !write_pointer(emu, encoded.cell, array.address) {
            return Err(EnvironmentError::Access);
        }
        if encoded.current.address != encoded.initial.address {
            buffers::free(emu, encoded.current.address, BufferKind::Crt);
        }
        encoded.current = array;
    }
    state.variables = Some(variables);
    Ok(())
}

fn putenv(emu: &mut Emu, runtime: Runtime, api: &str, wide: bool, secure: bool) {
    let call = WinApi(Cdecl);
    let name = api_try!(emu, call.arg(emu, 0));
    let value = if secure {
        api_try!(emu, call.arg(emu, 1))
    } else {
        0
    };
    log_red!(
        emu,
        "{}!{api} input={}",
        runtime.library(),
        input_string(emu, name, Encoding::from_wide(wide))
    );
    let result = put_value(emu, runtime, name, value, wide, secure);
    match result {
        Ok(()) => call.set_return(emu, 0),
        Err(error) => {
            let failure = if secure {
                (match error {
                    EnvironmentError::NoMemory => ENOMEM,
                    _ => EINVAL,
                }) as u64
            } else {
                u32::MAX as u64
            };
            report_error(emu, runtime, error, failure);
        }
    }
}

fn put_value(
    emu: &mut Emu,
    runtime: Runtime,
    name: u64,
    value: u64,
    wide: bool,
    secure: bool,
) -> EnvironmentResult<()> {
    let input = read(emu, name, wide)?;
    let (name, value) = if secure {
        (input, read(emu, value, wide)?)
    } else {
        let separator = input
            .iter()
            .position(|&unit| unit == b'=' as u16)
            .ok_or(EnvironmentError::Invalid)?;
        (input[..separator].to_vec(), input[separator + 1..].to_vec())
    };
    if !environment_store::valid_name(&name) {
        return Err(EnvironmentError::Invalid);
    }
    let value = (!value.is_empty()).then_some(value);
    with_environment(emu, runtime, |emu, state| {
        let mut variables = state.variables.clone().ok_or(EnvironmentError::Invalid)?;
        variables.set(name.clone(), value.clone());
        let mut process_variables = environment_store::process(emu).clone();
        process_variables.set(name, value);
        replace_variables(emu, state, variables, process_variables)
    })
}

pub(crate) fn dispatch(api: &str, emu: &mut Emu, runtime: Runtime) -> bool {
    match api {
        "_initialize_narrow_environment" => initialize(emu, runtime, api, false),
        "_initialize_wide_environment" => initialize(emu, runtime, api, true),
        "_get_initial_narrow_environment" => environment_pointer(emu, runtime, api, false, true),
        "_get_initial_wide_environment" => environment_pointer(emu, runtime, api, true, true),
        "__p__environ" => environment_pointer(emu, runtime, api, false, false),
        "__p__wenviron" => environment_pointer(emu, runtime, api, true, false),
        "getenv" => getenv(emu, runtime, api, false),
        "_wgetenv" => getenv(emu, runtime, api, true),
        "_putenv" => putenv(emu, runtime, api, false, false),
        "_wputenv" => putenv(emu, runtime, api, true, false),
        "_putenv_s" => putenv(emu, runtime, api, false, true),
        "_wputenv_s" => putenv(emu, runtime, api, true, true),
        _ => return false,
    }
    true
}
