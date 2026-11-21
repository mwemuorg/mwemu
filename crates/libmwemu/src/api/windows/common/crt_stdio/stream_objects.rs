use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::api::unix::memory::length;
use crate::emu::Emu;
use crate::exception::types::ExceptionType;
use crate::maps::mem64::Permission;
use crate::winapi::common::crt::Runtime;
use crate::winapi::common::crt_stdio::{
    FIRST_FD, Stream, close_descriptor, descriptors, mode_descriptor, read_text, set_errno,
    with_stream,
};
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::*;

fn standard_base(emu: &mut Emu, runtime: Runtime) -> Result<u64, i32> {
    if let Some(base) = runtime.io(emu).standard_base {
        return Ok(base);
    }
    let name = format!("{}_standard_streams", runtime.library());
    let (stride, offset) = runtime.stream_layout(emu);
    let size = stride * FIRST_FD as u64;
    let base = if let Some(map) = emu.maps.get_map_by_name(&name) {
        map.get_base()
    } else {
        let base = emu.maps.alloc(size).ok_or(ENOMEM)?;
        emu.maps
            .create_map(&name, base, size, Permission::READ_WRITE)
            .map_err(|_| ENOMEM)?;
        base
    };
    let mut streams = Vec::new();
    for index in 0..FIRST_FD {
        let file = base + stride * index as u64;
        let fd = if runtime.io(emu).closed_standard[index as usize] {
            -1
        } else {
            index
        };
        if !emu.maps.write_dword(file + offset, fd as u32) {
            return Err(EINVAL);
        }
        streams.push(Stream {
            file,
            fd,
            eof: false,
            error: false,
        });
    }
    let state = runtime.io(emu);
    state.streams.extend(streams);
    state.standard_base = Some(base);
    Ok(base)
}

pub(super) fn iob(emu: &mut Emu, runtime: Runtime, api: &str) {
    let call = WinApi(Cdecl);
    let index = if api == "__acrt_iob_func" {
        api_try!(emu, call.arg_u32(emu, 0))
    } else {
        0
    };
    log_red!(emu, "{}!{api} index={index}", runtime.library());
    if index >= FIRST_FD as u32 {
        set_errno(emu, runtime, EINVAL);
        call.set_return(emu, 0);
        return;
    }
    match standard_base(emu, runtime) {
        Ok(base) => call.set_return(emu, base + runtime.stream_layout(emu).0 * index as u64),
        Err(error) => {
            set_errno(emu, runtime, error);
            call.set_return(emu, 0);
        }
    }
}

pub(super) fn reopen(emu: &mut Emu, runtime: Runtime, api: &str, wide: bool) {
    let call = WinApi(Cdecl);
    let path = api_try!(emu, call.arg(emu, 0));
    let mode = api_try!(emu, call.arg(emu, 1));
    let file = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "{}!{api} stream={file:#x}", runtime.library());
    let result = (|| {
        let path = read_text(emu, path, wide).ok_or(EINVAL)?;
        let mode = read_text(emu, mode, wide).ok_or(EINVAL)?;
        let offset = runtime.stream_layout(emu).1;
        let fd = with_stream(emu, runtime, file, |s| s.fd)
            .filter(|fd| *fd >= 0)
            .ok_or(EBADF)?;
        if !emu.maps.validate_write_range(file + offset, 4) {
            return Err(EINVAL);
        }
        if !close_descriptor(emu, runtime, fd) {
            return Err(EBADF);
        }
        emu.maps.write_dword(file + offset, u32::MAX);
        let new_fd = mode_descriptor(emu, runtime, &path, &mode)?;
        let entry = runtime
            .io(emu)
            .descriptors
            .iter_mut()
            .find(|entry| entry.fd == new_fd)
            .ok_or(EBADF)?;
        entry.fd = fd;
        with_stream(emu, runtime, file, |s| {
            s.fd = fd;
            s.error = false;
            s.eof = false;
        });
        emu.maps.write_dword(file + offset, fd as u32);
        Ok(file)
    })();
    match result {
        Ok(file) => call.set_return(emu, file),
        Err(error) => {
            set_errno(emu, runtime, error);
            call.set_return(emu, 0);
        }
    }
}

pub(super) fn put(emu: &mut Emu, runtime: Runtime, api: &str) {
    let call = WinApi(Cdecl);
    let input = api_try!(emu, call.arg(emu, 0));
    let file = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "{}!{api} stream={file:#x}", runtime.library());
    let fd = with_stream(emu, runtime, file, |s| s.fd);
    let result = if let Some(fd) = fd {
        if api == "fputs" {
            let mut length = 0usize;
            loop {
                match input
                    .checked_add(length as u64)
                    .and_then(|p| emu.maps.read_byte(p))
                {
                    Some(0) => break,
                    Some(_) => length += 1,
                    None => {
                        emu.exception(ExceptionType::BadAddressDereferencing);
                        return;
                    }
                }
            }
            descriptors::write_bytes(emu, runtime, fd, input, length)
                .and_then(|n| if n == length { Ok(0) } else { Err(EIO) })
        } else {
            let byte = input as u8;
            descriptors::write_slice(emu, runtime, fd, &[byte])
                .and_then(|n| if n == 1 { Ok(byte as u64) } else { Err(EIO) })
        }
    } else {
        Err(EBADF)
    };
    match result {
        Ok(value) => call.set_return(emu, value),
        Err(error) => {
            with_stream(emu, runtime, file, |s| s.error = true);
            set_errno(emu, runtime, error);
            call.set_return(emu, EOF);
        }
    }
}

pub(super) fn standard_output(emu: &mut Emu, runtime: Runtime, api: &str) {
    let call = WinApi(Cdecl);
    let input = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "{}!{api}", runtime.library());
    let result = if api == "puts" {
        let count = match length(emu, input, u64::MAX) {
            Ok(len) => len as usize,
            Err(_) => {
                emu.exception(ExceptionType::BadAddressDereferencing);
                return;
            }
        };
        descriptors::write_bytes(emu, runtime, 1, input, count)
            .and_then(|n| {
                if n == count {
                    descriptors::write_slice(emu, runtime, 1, b"\n")
                } else {
                    Err(EIO)
                }
            })
            .and_then(|n| if n == 1 { Ok(0) } else { Err(EIO) })
    } else {
        let byte = input as u8;
        descriptors::write_slice(emu, runtime, 1, &[byte]).and_then(|n| {
            if n == 1 {
                Ok(u64::from(byte))
            } else {
                Err(EIO)
            }
        })
    };
    match result {
        Ok(value) => call.set_return(emu, value),
        Err(error) => {
            set_errno(emu, runtime, error);
            call.set_return(emu, EOF);
        }
    }
}
