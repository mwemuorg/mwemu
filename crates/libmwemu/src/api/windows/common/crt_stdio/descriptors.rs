use std::io::{ErrorKind, SeekFrom};

use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::exception::types::ExceptionType;
use crate::maps::mem64::Permission;
use crate::winapi::common::crt::Runtime;
use crate::winapi::common::crt_stdio::{
    Descriptor, FIRST_FD, IO_CHUNK_BYTES, close_descriptor, open_descriptor, read_text, set_errno,
};
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::*;
use crate::windows::constants::files::*;
use crate::windows::constants::security::*;

fn descriptor(emu: &mut Emu, runtime: Runtime, fd: i32) -> Option<Descriptor> {
    runtime
        .io(emu)
        .descriptors
        .iter()
        .find(|entry| entry.fd == fd)
        .copied()
}

fn standard_open(emu: &mut Emu, runtime: Runtime, fd: i32) -> bool {
    (0..FIRST_FD).contains(&fd) && !runtime.io(emu).closed_standard[fd as usize]
}

fn io_error(error: std::io::Error) -> i32 {
    match error.kind() {
        ErrorKind::PermissionDenied | ErrorKind::InvalidInput => EBADF,
        _ => EIO,
    }
}

pub(super) fn read_bytes(
    emu: &mut Emu,
    runtime: Runtime,
    fd: i32,
    destination: u64,
    length: usize,
) -> Result<usize, i32> {
    let Some(entry) = descriptor(emu, runtime, fd) else {
        return if fd == 0 && standard_open(emu, runtime, fd) {
            Ok(0)
        } else {
            Err(EBADF)
        };
    };
    if !entry.readable {
        return Err(EBADF);
    }
    let mut buffer = [0; IO_CHUNK_BYTES];
    let mut total = 0;
    while total < length {
        let address = destination + total as u64;
        let map = emu.maps.get_mem_by_addr(address).ok_or(EINVAL)?;
        let capacity = (length - total)
            .min(buffer.len())
            .min((map.get_bottom() - address) as usize);
        let result = emu
            .handle_management
            .get_mut_file_handle(entry.handle)
            .ok_or(EBADF)?
            .read(&mut buffer[..capacity]);
        match result {
            Ok(0) => break,
            Ok(count) => {
                if !emu.maps.write_bytes(address, &buffer[..count]) {
                    return Err(EINVAL);
                }
                total += count;
            }
            Err(error) if total == 0 => return Err(io_error(error)),
            Err(_) => break,
        }
    }
    Ok(total)
}

pub(super) fn write_bytes(
    emu: &mut Emu,
    runtime: Runtime,
    fd: i32,
    source: u64,
    length: usize,
) -> Result<usize, i32> {
    let entry = descriptor(emu, runtime, fd);
    if entry.is_some_and(|entry| !entry.writable)
        || (entry.is_none() && (!(1..FIRST_FD).contains(&fd) || !standard_open(emu, runtime, fd)))
    {
        return Err(EBADF);
    }
    let mut total = 0;
    while total < length {
        let address = source + total as u64;
        let map = emu.maps.get_mem_by_addr(address).ok_or(EINVAL)?;
        let count = (length - total)
            .min(IO_CHUNK_BYTES)
            .min((map.get_bottom() - address) as usize);
        let bytes = emu
            .maps
            .try_read_bytes(address, count)
            .ok_or(EINVAL)?
            .to_vec();
        match write_slice(emu, runtime, fd, &bytes) {
            Ok(0) if total == 0 => return Err(EIO),
            Ok(0) => break,
            Ok(written) => total += written,
            Err(error) if total == 0 => return Err(error),
            Err(_) => break,
        }
    }
    Ok(total)
}

pub(super) fn write_slice(
    emu: &mut Emu,
    runtime: Runtime,
    fd: i32,
    bytes: &[u8],
) -> Result<usize, i32> {
    if let Some(entry) = descriptor(emu, runtime, fd) {
        if !entry.writable {
            return Err(EBADF);
        }
        let file = emu
            .handle_management
            .get_mut_file_handle(entry.handle)
            .ok_or(EBADF)?;
        if entry.append {
            file.seek(SeekFrom::End(0)).map_err(io_error)?;
        }
        file.write(bytes).map_err(io_error)
    } else if (1..FIRST_FD).contains(&fd) && standard_open(emu, runtime, fd) {
        emu.emulated_stdout.extend_from_slice(bytes);
        Ok(bytes.len())
    } else {
        Err(EBADF)
    }
}

pub(super) fn read(emu: &mut Emu, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let fd = api_try!(emu, call.arg_i32(emu, 0));
    let destination = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg_u32(emu, 2));
    log_red!(
        emu,
        "{}!_read fd={fd} buffer={destination:#x} count={count}",
        runtime.library()
    );
    if count > i32::MAX as u32 || (destination == 0 && count != 0) {
        set_errno(emu, runtime, EINVAL);
        call.set_return(emu, EOF);
        return;
    }
    if !emu
        .maps
        .can_access_range(destination, count as usize, Permission::WRITE)
    {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    let result = read_bytes(emu, runtime, fd, destination, count as usize);
    return_io_result(emu, runtime, result);
}

pub(super) fn write(emu: &mut Emu, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let fd = api_try!(emu, call.arg_i32(emu, 0));
    let source = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg_u32(emu, 2));
    log_red!(
        emu,
        "{}!_write fd={fd} buffer={source:#x} count={count}",
        runtime.library()
    );
    if count > i32::MAX as u32 || (source == 0 && count != 0) {
        set_errno(emu, runtime, EINVAL);
        call.set_return(emu, EOF);
        return;
    }
    if !emu
        .maps
        .can_access_range(source, count as usize, Permission::READ)
    {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    let result = write_bytes(emu, runtime, fd, source, count as usize);
    return_io_result(emu, runtime, result);
}

fn return_io_result(emu: &mut Emu, runtime: Runtime, result: Result<usize, i32>) {
    let call = WinApi(Cdecl);
    match result {
        Ok(count) => call.set_return(emu, count as u64),
        Err(error) => {
            set_errno(emu, runtime, error);
            call.set_return(emu, EOF);
        }
    }
}

pub(super) fn close(emu: &mut Emu, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let fd = api_try!(emu, call.arg_i32(emu, 0));
    log_red!(emu, "{}!_close fd={fd}", runtime.library());
    if close_descriptor(emu, runtime, fd) {
        call.set_return(emu, 0);
    } else {
        set_errno(emu, runtime, EBADF);
        call.set_return(emu, EOF);
    }
}

pub(super) fn open(emu: &mut Emu, runtime: Runtime, api: &str, wide: bool) {
    let call = WinApi(Cdecl);
    let path = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg_i32(emu, 1));
    let permissions = if flags & _O_CREAT != 0 {
        api_try!(emu, call.arg_i32(emu, 2))
    } else {
        _S_IREAD | _S_IWRITE
    };
    let access = match flags & (_O_WRONLY | _O_RDWR) {
        _O_RDONLY => GENERIC_READ,
        _O_WRONLY => GENERIC_WRITE,
        _O_RDWR => GENERIC_READ | GENERIC_WRITE,
        _ => {
            set_errno(emu, runtime, EINVAL);
            call.set_return(emu, EOF);
            return;
        }
    };
    let supported = _O_WRONLY
        | _O_RDWR
        | _O_APPEND
        | _O_CREAT
        | _O_TRUNC
        | _O_EXCL
        | _O_TEXT
        | _O_BINARY
        | _O_NOINHERIT
        | _O_SEQUENTIAL
        | _O_RANDOM
        | _O_SHORT_LIVED;
    if flags & !supported != 0 || permissions & !(_S_IREAD | _S_IWRITE) != 0 {
        set_errno(emu, runtime, EINVAL);
        call.set_return(emu, EOF);
        return;
    }
    let path = read_text(emu, path, wide);
    log_red!(
        emu,
        "{}!{api} path={path:?} flags={flags:#x}",
        runtime.library()
    );
    let Some(path) = path else {
        set_errno(emu, runtime, EINVAL);
        call.set_return(emu, EOF);
        return;
    };
    let disposition = if flags & _O_CREAT != 0 {
        if flags & _O_EXCL != 0 {
            CREATE_NEW
        } else if flags & _O_TRUNC != 0 {
            CREATE_ALWAYS
        } else {
            OPEN_ALWAYS
        }
    } else if flags & _O_TRUNC != 0 {
        TRUNCATE_EXISTING
    } else {
        OPEN_EXISTING
    };
    let result = open_descriptor(
        emu,
        runtime,
        &path,
        access,
        disposition as u32,
        flags & _O_APPEND != 0,
    );
    return_io_result(emu, runtime, result.map(|fd| fd as usize));
}
