use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::exception::types::ExceptionType;
use crate::maps::buffers::BufferKind;
use crate::maps::mem64::Permission;
use crate::winapi::common::{
    buffers,
    crt::Runtime,
    crt_stdio,
    memory::{pointer_size, read_pointer, write_pointer},
    threads,
};
use crate::winapi::policy::api_try;

pub(super) fn dispatch(emu: &mut Emu, runtime: Runtime, api: &str) -> bool {
    match api {
        "_initterm" | "_initterm_e" => initializers(emu, api == "_initterm_e"),
        "_initialize_onexit_table" => initialize_onexit(emu),
        "_register_onexit_function" | "atexit" | "_crt_atexit" | "_onexit" => {
            register(emu, runtime, api)
        }
        "_execute_onexit_table" => execute_table(emu),
        "__dllonexit" => dll_onexit(emu),
        "exit" | "_exit" | "_Exit" | "_cexit" | "_c_exit" => terminate(emu, runtime, api),
        _ => return false,
    }
    true
}

fn invoke(emu: &mut Emu, callback: u64) -> Result<u32, ()> {
    let result = if emu.cfg.arch.is_64bits() {
        emu.call64(callback, &[]).map(|value| value as u32)
    } else {
        emu.call32(callback, &[])
    };
    result.map_err(|error| {
        log::warn!("CRT callback {callback:#x}: {error}");
        emu.stop();
    })
}

fn initializers(emu: &mut Emu, with_result: bool) {
    let call = WinApi(Cdecl);
    let mut slot = api_try!(emu, call.arg(emu, 0));
    let end = api_try!(emu, call.arg(emu, 1));
    let width = pointer_size(emu);
    while slot < end {
        let Some(callback) = read_pointer(emu, slot) else {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return;
        };
        if callback != 0 {
            let result = invoke(emu, callback);
            match result {
                Ok(value) if with_result && value != 0 => {
                    call.set_return(emu, value.into());
                    return;
                }
                Ok(_) => {}
                Err(()) => return,
            }
        }
        let Some(next) = slot.checked_add(width) else {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return;
        };
        slot = next;
    }
    if with_result {
        call.set_return(emu, 0);
    }
}

const ONEXIT_FIELDS: u64 = 3;
const INITIAL_ONEXIT_CAPACITY: u64 = 32;

fn initialize_onexit(emu: &mut Emu) {
    let call = WinApi(Cdecl);
    let table = api_try!(emu, call.arg(emu, 0));
    let width = pointer_size(emu);
    let size = (ONEXIT_FIELDS * width) as usize;
    if table == 0 {
        call.set_return(emu, u32::MAX as u64);
        return;
    }
    let (Some(first), Some(end)) = (
        read_pointer(emu, table),
        table
            .checked_add(2 * width)
            .and_then(|p| read_pointer(emu, p)),
    ) else {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    };
    if first != end {
        call.set_return(emu, 0);
        return;
    }
    if !emu.maps.write_bytes(
        table,
        &[0; ONEXIT_FIELDS as usize * size_of::<u64>()][..size],
    ) {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    call.set_return(emu, 0);
}

fn append(emu: &mut Emu, table: u64, callback: u64) -> Result<bool, ()> {
    let width = pointer_size(emu);
    if table == 0 {
        return Ok(false);
    }
    if !emu.maps.validate_write_range(table, ONEXIT_FIELDS * width) {
        return Err(());
    }
    let (Some(mut first), Some(mut last), Some(mut end)) = (
        read_pointer(emu, table),
        read_pointer(emu, table + width),
        read_pointer(emu, table + 2 * width),
    ) else {
        return Err(());
    };
    if first > last
        || last > end
        || !(last - first).is_multiple_of(width)
        || !(end - first).is_multiple_of(width)
    {
        return Ok(false);
    }
    if last == end {
        let used = last - first;
        let Some(capacity) = used.max(INITIAL_ONEXIT_CAPACITY * width / 2).checked_mul(2) else {
            return Ok(false);
        };
        let allocation = if first == 0 {
            buffers::allocate(emu, capacity, width, 0, BufferKind::Crt)
        } else {
            buffers::reallocate(emu, first, capacity, BufferKind::Crt)
        };
        let Some(base) = allocation else {
            return Ok(false);
        };
        first = base;
        last = base + used;
        end = base + capacity;
    }
    if !write_pointer(emu, last, callback)
        || !write_pointer(emu, table, first)
        || !write_pointer(emu, table + width, last + width)
        || !write_pointer(emu, table + 2 * width, end)
    {
        return Err(());
    }
    Ok(true)
}

fn global_table(emu: &mut Emu, runtime: Runtime) -> Option<u64> {
    let name = match runtime {
        Runtime::Msvcrt => "msvcrt_onexit_table",
        Runtime::Ucrt => "ucrt_onexit_table",
    };
    if let Some(map) = emu.maps.get_map_by_name(name) {
        return Some(map.get_base());
    }
    let size = ONEXIT_FIELDS * pointer_size(emu);
    let address = emu.maps.alloc(size)?;
    emu.maps
        .create_map(name, address, size, Permission::READ_WRITE)
        .ok()?;
    Some(address)
}

fn register(emu: &mut Emu, runtime: Runtime, api: &str) {
    let call = WinApi(Cdecl);
    let arg = api_try!(emu, call.arg(emu, 0));
    let (table, callback) = if api == "_register_onexit_function" {
        (arg, api_try!(emu, call.arg(emu, 1)))
    } else {
        (global_table(emu, runtime).unwrap_or(0), arg)
    };
    let result = match append(emu, table, callback) {
        Ok(true) if api == "_onexit" => callback,
        Ok(false) if api != "_onexit" => u32::MAX.into(),
        Ok(_) => 0,
        Err(()) => {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return;
        }
    };
    call.set_return(emu, result);
}

fn dll_onexit(emu: &mut Emu) {
    let call = WinApi(Cdecl);
    let callback = api_try!(emu, call.arg(emu, 0));
    let first_ptr = api_try!(emu, call.arg(emu, 1));
    let end_ptr = api_try!(emu, call.arg(emu, 2));
    let width = pointer_size(emu);
    if first_ptr == 0 || end_ptr == 0 {
        call.set_return(emu, 0);
        return;
    }
    if !emu.maps.validate_write_range(first_ptr, width)
        || !emu.maps.validate_write_range(end_ptr, width)
    {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    let (Some(first), Some(end)) = (read_pointer(emu, first_ptr), read_pointer(emu, end_ptr))
    else {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    };
    let Some(used) = end
        .checked_sub(first)
        .filter(|used| used.is_multiple_of(width))
    else {
        call.set_return(emu, 0);
        return;
    };
    let Some(size) = used.checked_add(width) else {
        call.set_return(emu, 0);
        return;
    };
    if first == 0 && end != 0 {
        call.set_return(emu, 0);
        return;
    }
    let base = if first == 0 {
        buffers::allocate(emu, size, width, 0, BufferKind::Crt)
    } else {
        buffers::reallocate(emu, first, size, BufferKind::Crt)
    };
    let Some(base) = base else {
        call.set_return(emu, 0);
        return;
    };
    if !write_pointer(emu, base + used, callback)
        || !write_pointer(emu, first_ptr, base)
        || !write_pointer(emu, end_ptr, base + size)
    {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    call.set_return(emu, callback);
}

fn execute_table(emu: &mut Emu) {
    let call = WinApi(Cdecl);
    let table = api_try!(emu, call.arg(emu, 0));
    if table == 0 {
        call.set_return(emu, u32::MAX.into());
    } else if drain(emu, table) {
        call.set_return(emu, 0);
    }
}

fn drain(emu: &mut Emu, table: u64) -> bool {
    let width = pointer_size(emu);
    if !emu.maps.validate_write_range(table, ONEXIT_FIELDS * width) {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return false;
    }
    // A callback can append entries, reallocate the table, or drain it recursively.
    loop {
        let (Some(first), Some(last), Some(end)) = (
            read_pointer(emu, table),
            read_pointer(emu, table + width),
            read_pointer(emu, table + 2 * width),
        ) else {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return false;
        };
        if first > last || last > end || !(last - first).is_multiple_of(width) {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return false;
        }
        if first == last {
            if first != 0 {
                buffers::free(emu, first, BufferKind::Crt);
            }
            return emu.maps.write_bytes(
                table,
                &[0; 3 * size_of::<u64>()][..(ONEXIT_FIELDS * width) as usize],
            );
        }
        let slot = last - width;
        let Some(callback) = read_pointer(emu, slot) else {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return false;
        };
        if !write_pointer(emu, table + width, slot) {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return false;
        }
        if callback != 0 && invoke(emu, callback).is_err() {
            return false;
        }
    }
}

fn terminate(emu: &mut Emu, runtime: Runtime, api: &str) {
    let call = WinApi(Cdecl);
    let code = if matches!(api, "exit" | "_exit" | "_Exit") {
        Some(api_try!(emu, call.arg_u32(emu, 0)))
    } else {
        None
    };
    if matches!(api, "exit" | "_cexit") {
        if let Some(table) = global_table(emu, runtime)
            && !drain(emu, table)
        {
            return;
        }
        crt_stdio::close_streams(emu, runtime);
    }
    if let Some(code) = code {
        threads::exit_with_code(emu, code);
    }
}
