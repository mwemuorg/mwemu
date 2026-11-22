//! Unbuffered binary streams over the existing file-root jail. Text translation and interprocess sharing are not modeled.

use std::io::ErrorKind;

use crate::api::dispatch_api;
use crate::emu;
use crate::emu::object_handle::FileHandle;
use crate::exception::types::ExceptionType;
use crate::maps::buffers::BufferKind;
use crate::winapi::common::buffers;
use crate::winapi::common::crt::{self, Runtime};
use crate::winapi::common::strings;
use crate::windows::constants::crt::*;
use crate::windows::constants::files::*;
use crate::windows::constants::security::*;

mod descriptors;
mod formatted;
pub(crate) use formatted::print as formatted_print;
mod stream_objects;

mod clearerr;
use clearerr::clearerr;

mod fclose;
use fclose::fclose;

mod fflush;
use fflush::fflush;

mod fgetc;
use fgetc::{fgetc, read_stdin};

mod fgets;
use fgets::fgets;

mod filelength;
use filelength::filelength;

mod fileno;
use fileno::fileno;

mod fopen;
use fopen::fopen;

mod fopen_s;
use fopen_s::fopen_s;

mod fread;
use fread::fread;

mod fseek;
use fseek::fseek;

mod ftell;
use ftell::ftell;

mod fwrite;
use fwrite::fwrite;

mod get_osfhandle;
use get_osfhandle::get_osfhandle;

mod rewind;
use rewind::rewind;

mod stream_flag;
use stream_flag::stream_flag;

// Guest stream layouts: MSVCRT FILE and UCRT __crt_stdio_stream_data.
const MSVCRT_STREAM_SIZE_32: u64 = 32;
const MSVCRT_STREAM_SIZE_64: u64 = 48;
const UCRT_STREAM_SIZE_32: u64 = 56;
const UCRT_STREAM_SIZE_64: u64 = 88;
const STREAM_FILE_OFFSET_32: u64 = 16;
const MSVCRT_STREAM_FILE_OFFSET_64: u64 = 28;
const UCRT_STREAM_FILE_OFFSET_64: u64 = 24;
const IO_CHUNK_BYTES: usize = 64 * 1024;

/// 0-2 are the standard streams.
const FIRST_FD: i32 = 3;

struct Stream {
    file: u64,
    fd: i32,
    eof: bool,
    error: bool,
}

#[derive(Clone, Copy)]
struct Descriptor {
    fd: i32,
    handle: u32,
    append: bool,
    readable: bool,
    writable: bool,
}

#[derive(Default)]
struct RuntimeIo {
    standard_base: Option<u64>,
    streams: Vec<Stream>,
    descriptors: Vec<Descriptor>,
    closed_standard: [bool; 3],
}

/// Host-backed files are not restored by native snapshots; their stream registry starts empty.
#[derive(Default)]
pub(crate) struct CrtIoState {
    msvcrt: RuntimeIo,
    ucrt: RuntimeIo,
}

impl Runtime {
    fn io(self, emu: &mut emu::Emu) -> &mut RuntimeIo {
        match self {
            Self::Msvcrt => &mut emu.crt_io.msvcrt,
            Self::Ucrt => &mut emu.crt_io.ucrt,
        }
    }

    fn stream_layout(self, emu: &emu::Emu) -> (u64, u64) {
        match (self, emu.cfg.arch.is_64bits()) {
            (Self::Msvcrt, false) => (MSVCRT_STREAM_SIZE_32, STREAM_FILE_OFFSET_32),
            (Self::Msvcrt, true) => (MSVCRT_STREAM_SIZE_64, MSVCRT_STREAM_FILE_OFFSET_64),
            (Self::Ucrt, false) => (UCRT_STREAM_SIZE_32, STREAM_FILE_OFFSET_32),
            (Self::Ucrt, true) => (UCRT_STREAM_SIZE_64, UCRT_STREAM_FILE_OFFSET_64),
        }
    }
}

/// Run `api` if it is a CRT stdio API; false otherwise.
pub(crate) fn gateway(api: &str, emu: &mut emu::Emu, runtime: Runtime) -> bool {
    dispatch_api!(api, {
        "__acrt_iob_func" | "__iob_func" => stream_objects::iob(emu, runtime, api),
        "freopen" | "_wfreopen" => stream_objects::reopen(emu, runtime, api, api == "_wfreopen"),
        "sprintf" | "vsprintf" | "snprintf" | "_snprintf" | "vsnprintf" | "_vsnprintf" => formatted::buffer(emu,runtime,api),
        "fprintf" | "vfprintf" | "__stdio_common_vfprintf" | "printf" | "vprintf" => formatted::print(emu, runtime, api),
        "puts" | "putchar" => stream_objects::standard_output(emu,runtime,api),
        "putc" | "fputc" | "fputs" => stream_objects::put(emu, runtime, api),
        "_open" => descriptors::open(emu, runtime, api, false),
        "_wopen" => descriptors::open(emu, runtime, api, true),
        "_read" => descriptors::read(emu, runtime),
        "_write" => descriptors::write(emu, runtime),
        "_close" => descriptors::close(emu, runtime),
        "fopen" | "_fsopen" => fopen(emu, runtime, api, false),
        "_wfopen" | "_wfsopen" => fopen(emu, runtime, api, true),
        "fopen_s" => fopen_s(emu, runtime, api, false),
        "_wfopen_s" => fopen_s(emu, runtime, api, true),
        "fclose" => fclose(emu, runtime),
        "fread" => fread(emu, runtime),
        "fwrite" => fwrite(emu, runtime),
        "fgetc" | "getc" => fgetc(emu, runtime, api),
        "getchar" | "_fgetchar" => read_stdin(emu, runtime, api),
        "fgets" => fgets(emu, runtime),
        "fseek" => fseek(emu, runtime, api, false),
        "_fseeki64" => fseek(emu, runtime, api, true),
        "ftell" => ftell(emu, runtime, api, true),
        "_ftelli64" => ftell(emu, runtime, api, false),
        "rewind" => rewind(emu, runtime),
        "feof" => stream_flag(emu, runtime, api, |s| s.eof),
        "ferror" => stream_flag(emu, runtime, api, |s| s.error),
        "clearerr" => clearerr(emu, runtime),
        "fflush" => fflush(emu, runtime),
        "_fileno" => fileno(emu, runtime),
        "_filelength" => filelength(emu, runtime, api, true),
        "_filelengthi64" => filelength(emu, runtime, api, false),
        "_get_osfhandle" => get_osfhandle(emu, runtime),
    },
        handled(()) => true,
        fallback => false,
    )
}

fn set_errno(emu: &mut emu::Emu, runtime: Runtime, value: i32) {
    let Some(cell) = crt::errno_address(emu, runtime) else {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    };
    if !emu.maps.write_bytes(cell, &value.to_le_bytes()) {
        emu.exception(ExceptionType::BadAddressDereferencing);
    }
}

// --- streams ---

/// kernel32 handle behind a stream.
fn stream_handle(emu: &mut emu::Emu, runtime: Runtime, file: u64) -> Option<u32> {
    let state = runtime.io(emu);
    let fd = state.streams.iter().find(|s| s.file == file)?.fd;
    state
        .descriptors
        .iter()
        .find(|d| d.fd == fd)
        .map(|d| d.handle)
}

fn with_stream<T>(
    emu: &mut emu::Emu,
    runtime: Runtime,
    file: u64,
    f: impl FnOnce(&mut Stream) -> T,
) -> Option<T> {
    let state = runtime.io(emu);
    state.streams.iter_mut().find(|s| s.file == file).map(f)
}

fn read_text(emu: &emu::Emu, ptr: u64, wide: bool) -> Option<String> {
    (ptr != 0)
        .then(|| strings::read_text_checked(emu, ptr, wide))
        .flatten()
}

/// Open `path` with an fopen mode string; the new FILE* or an errno value.
fn open(emu: &mut emu::Emu, runtime: Runtime, path: &str, mode: &str) -> Result<u64, i32> {
    let (stream_size, file_offset) = runtime.stream_layout(emu);
    let stream = buffers::allocate(emu, stream_size, 16, 0, BufferKind::Crt).ok_or(ENOMEM)?;
    if !emu.maps.fill_range(stream, 0, stream_size as usize) {
        buffers::free(emu, stream, BufferKind::Crt);
        return Err(ENOMEM);
    }
    let fd = match mode_descriptor(emu, runtime, path, mode) {
        Ok(fd) => fd,
        Err(error) => {
            buffers::free(emu, stream, BufferKind::Crt);
            return Err(error);
        }
    };
    if !emu.maps.write_dword(stream + file_offset, fd as u32) {
        close_descriptor(emu, runtime, fd);
        buffers::free(emu, stream, BufferKind::Crt);
        return Err(ENOMEM);
    }
    runtime.io(emu).streams.push(Stream {
        file: stream,
        fd,
        eof: false,
        error: false,
    });
    Ok(stream)
}

fn mode_descriptor(
    emu: &mut emu::Emu,
    runtime: Runtime,
    path: &str,
    mode: &str,
) -> Result<i32, i32> {
    let update = mode.contains('+');
    let (disposition, mut access, append) = match mode.chars().next() {
        Some('r') => (OPEN_EXISTING as u32, GENERIC_READ, false),
        Some('w') if mode.contains('x') => (CREATE_NEW as u32, GENERIC_WRITE, false),
        Some('w') => (CREATE_ALWAYS as u32, GENERIC_WRITE, false),
        Some('a') => (OPEN_ALWAYS as u32, GENERIC_WRITE, true),
        _ => return Err(EINVAL),
    };
    if update {
        access |= GENERIC_READ | GENERIC_WRITE;
    }
    open_descriptor(emu, runtime, path, access, disposition, append)
}

fn open_descriptor(
    emu: &mut emu::Emu,
    runtime: Runtime,
    path: &str,
    access: u32,
    disposition: u32,
    append: bool,
) -> Result<i32, i32> {
    let state = runtime.io(emu);
    let fd = (0..i32::MAX)
        .find(|fd| {
            (*fd >= FIRST_FD || state.closed_standard[*fd as usize])
                && state.descriptors.iter().all(|entry| entry.fd != *fd)
        })
        .ok_or(EMFILE)?;
    let file =
        FileHandle::new(path.to_string(), access, disposition, 0, 0).map_err(
            |error| match error
                .downcast_ref::<std::io::Error>()
                .map(|error| error.kind())
            {
                Some(ErrorKind::NotFound) => ENOENT,
                Some(ErrorKind::AlreadyExists) => EEXIST,
                Some(ErrorKind::PermissionDenied) => EACCES,
                _ => EINVAL,
            },
        )?;
    let handle = emu.handle_management.insert_file_handle(file);
    runtime.io(emu).descriptors.push(Descriptor {
        fd,
        handle,
        append,
        readable: access & GENERIC_READ != 0,
        writable: access & GENERIC_WRITE != 0,
    });
    Ok(fd)
}

pub(crate) fn close_streams(emu: &mut emu::Emu, runtime: Runtime) {
    while let Some(file) = runtime.io(emu).streams.last().map(|stream| stream.file) {
        fclose::close_stream(emu, runtime, file);
    }
    runtime.io(emu).closed_standard.fill(true);
}

fn close_descriptor(emu: &mut emu::Emu, runtime: Runtime, fd: i32) -> bool {
    let state = runtime.io(emu);
    let removed = state
        .descriptors
        .iter()
        .position(|entry| entry.fd == fd)
        .map(|index| state.descriptors.remove(index));
    if removed.is_some() || ((0..FIRST_FD).contains(&fd) && !state.closed_standard[fd as usize]) {
        for stream in &mut state.streams {
            if stream.fd == fd {
                stream.fd = -1;
            }
        }
    }
    if let Some(entry) = removed {
        emu.handle_management
            .remove_file_handle(entry.handle)
            .is_some()
    } else if (0..FIRST_FD).contains(&fd) && !state.closed_standard[fd as usize] {
        state.closed_standard[fd as usize] = true;
        true
    } else {
        false
    }
}

fn io_length(emu: &emu::Emu, size: u64, count: u64) -> Option<usize> {
    let length = size.checked_mul(count)?;
    if !emu.cfg.arch.is_64bits() && length > u32::MAX as u64 {
        return None;
    }
    usize::try_from(length).ok()
}

fn read_into(
    emu: &mut emu::Emu,
    runtime: Runtime,
    file: u64,
    destination: u64,
    length: usize,
) -> Option<usize> {
    let fd = with_stream(emu, runtime, file, |stream| stream.fd)?;
    match descriptors::read_bytes(emu, runtime, fd, destination, length) {
        Ok(count) => {
            if count < length {
                with_stream(emu, runtime, file, |stream| stream.eof = true);
            }
            Some(count)
        }
        Err(error) => {
            with_stream(emu, runtime, file, |stream| stream.error = true);
            set_errno(emu, runtime, error);
            Some(0)
        }
    }
}

fn fd_handle(emu: &mut emu::Emu, runtime: Runtime, fd: i32) -> Option<u32> {
    let state = runtime.io(emu);
    state
        .descriptors
        .iter()
        .find(|d| d.fd == fd)
        .map(|d| d.handle)
}
