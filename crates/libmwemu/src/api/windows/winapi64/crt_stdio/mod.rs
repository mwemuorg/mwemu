mod clearerr;

use crate::windows::constants::security::*;
use clearerr::clearerr;
mod errno_location;
use errno_location::errno_location;
mod fclose;
use fclose::fclose;
mod fflush;
use fflush::fflush;
mod fgetc;
use fgetc::fgetc;
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
mod get_errno;
use get_errno::_get_errno;
mod get_osfhandle;
use get_osfhandle::get_osfhandle;
mod rewind;
use rewind::rewind;
mod set_errno;
use set_errno::_set_errno;
mod stream_flag;
use stream_flag::stream_flag;

/*
    CRT stdio and low-level I/O over the kernel32 file handles.

    A FILE* is a real ucrt stream struct in guest memory whose _file field
    holds a CRT file descriptor; each descriptor wraps a kernel32 file handle
    (what _get_osfhandle returns), so fopen'd files live in the same jailed
    file system as CreateFile. Text mode is not translated: streams behave as
    if opened with "b".

    errno is a single per-process cell, as for a single-threaded process.
*/

use lazy_static::lazy_static;
use std::io::ErrorKind;
use std::sync::Mutex;

use super::wincrt::allocate_memory;
use crate::emu;
use crate::emu::object_handle::FileHandle;

const EOF: u64 = u32::MAX as u64; // (int)-1
const INVALID_HANDLE_VALUE: u64 = u64::MAX;

const ENOENT: u32 = 2;
const EBADF: u32 = 9;
const EACCES: u32 = 13;
const EEXIST: u32 = 17;
const EINVAL: u32 = 22;

const SEEK_SET: u64 = 0;
const SEEK_CUR: u64 = 1;
const SEEK_END: u64 = 2;

/// sizeof(__crt_stdio_stream_data) and its _file field.
const STREAM_SIZE: u64 = 0x58;
const STREAM_FILE_OFFSET: u64 = 0x18;

/// 0-2 are the standard streams.
const FIRST_FD: i32 = 3;

struct Stream {
    file: u64,
    fd: i32,
    append: bool,
    eof: bool,
    error: bool,
}

struct Descriptor {
    fd: i32,
    handle: u32,
}

#[derive(Default)]
struct State {
    streams: Vec<Stream>,
    descriptors: Vec<Descriptor>,
    errno: u64,
}

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State::default());
}

/// Run `api` if it is a CRT stdio API; false otherwise.
pub fn gateway(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "fopen" | "_fsopen" => fopen(emu, api, false),
        "_wfopen" | "_wfsopen" => fopen(emu, api, true),
        "fopen_s" => fopen_s(emu, api, false),
        "_wfopen_s" => fopen_s(emu, api, true),
        "fclose" => fclose(emu),
        "fread" => fread(emu),
        "fwrite" => fwrite(emu),
        "fgetc" | "getc" => fgetc(emu, api),
        "fgets" => fgets(emu),
        "fseek" => fseek(emu, api, emu.regs().rdx as i32 as i64),
        "_fseeki64" => fseek(emu, api, emu.regs().rdx as i64),
        "ftell" => ftell(emu, api, true),
        "_ftelli64" => ftell(emu, api, false),
        "rewind" => rewind(emu),
        "feof" => stream_flag(emu, api, |s| s.eof),
        "ferror" => stream_flag(emu, api, |s| s.error),
        "clearerr" => clearerr(emu),
        "fflush" => fflush(emu),
        "_fileno" => fileno(emu),
        "_filelength" => filelength(emu, api, true),
        "_filelengthi64" => filelength(emu, api, false),
        "_get_osfhandle" => get_osfhandle(emu),
        "_errno" => errno_location(emu),
        "_set_errno" => _set_errno(emu),
        "_get_errno" => _get_errno(emu),
        _ => return false,
    }
    true
}

// --- errno ---

fn errno_cell(emu: &mut emu::Emu) -> u64 {
    let cell = STATE.lock().unwrap().errno;
    if cell != 0 && emu.maps.is_mapped(cell) {
        return cell;
    }
    let cell = allocate_memory(emu, 4).unwrap_or(0);
    emu.maps.write_dword(cell, 0);
    STATE.lock().unwrap().errno = cell;
    cell
}

fn set_errno(emu: &mut emu::Emu, value: u32) {
    let cell = errno_cell(emu);
    emu.maps.write_dword(cell, value);
}

/* errno_t _set_errno(int value); */
/* errno_t _get_errno(int *pValue); */
// --- streams ---

/// kernel32 handle behind a stream.
fn stream_handle(file: u64) -> Option<u32> {
    let state = STATE.lock().unwrap();
    let fd = state.streams.iter().find(|s| s.file == file)?.fd;
    state
        .descriptors
        .iter()
        .find(|d| d.fd == fd)
        .map(|d| d.handle)
}

fn with_stream<T>(file: u64, f: impl FnOnce(&mut Stream) -> T) -> Option<T> {
    let mut state = STATE.lock().unwrap();
    state.streams.iter_mut().find(|s| s.file == file).map(f)
}

fn read_text(emu: &emu::Emu, ptr: u64, wide: bool) -> String {
    if wide {
        emu.maps.read_wide_string(ptr)
    } else {
        emu.maps.read_string(ptr)
    }
}

/// Open `path` with an fopen mode string; the new FILE* or an errno value.
fn open(emu: &mut emu::Emu, path: &str, mode: &str) -> Result<u64, u32> {
    let update = mode.contains('+');
    let (disposition, mut access, append) = match mode.chars().next() {
        Some('r') => (3, GENERIC_READ, false), // OPEN_EXISTING
        Some('w') if mode.contains('x') => (1, GENERIC_WRITE, false), // CREATE_NEW
        Some('w') => (2, GENERIC_WRITE, false), // CREATE_ALWAYS
        Some('a') => (4, GENERIC_WRITE, true), // OPEN_ALWAYS
        _ => return Err(EINVAL),
    };
    if update {
        access |= GENERIC_READ | GENERIC_WRITE;
    }
    let file = FileHandle::new(path.to_string(), access, disposition, 0, 0).map_err(|e| match e
        .downcast_ref::<std::io::Error>()
        .map(|e| e.kind())
    {
        Some(ErrorKind::NotFound) => ENOENT,
        Some(ErrorKind::AlreadyExists) => EEXIST,
        Some(ErrorKind::PermissionDenied) => EACCES,
        _ => EINVAL,
    })?;
    let Some(stream) = allocate_memory(emu, STREAM_SIZE) else {
        return Err(EINVAL);
    };
    let handle = emu.handle_management.insert_file_handle(file);

    let mut state = STATE.lock().unwrap();
    let fd = (FIRST_FD..)
        .find(|fd| state.descriptors.iter().all(|d| d.fd != *fd))
        .unwrap();
    state.descriptors.push(Descriptor { fd, handle });
    state.streams.push(Stream {
        file: stream,
        fd,
        append,
        eof: false,
        error: false,
    });
    drop(state);

    emu.maps.write_bytes(stream, &[0; STREAM_SIZE as usize]);
    emu.maps.write_dword(stream + STREAM_FILE_OFFSET, fd as u32);
    Ok(stream)
}

/* FILE *fopen(const char *filename, const char *mode); */
/* errno_t fopen_s(FILE **pFile, const char *filename, const char *mode); */
/* int fclose(FILE *stream); */
/// Read up to `len` bytes from a stream into guest memory; bytes read.
fn read_into(emu: &mut emu::Emu, file: u64, dst: u64, len: usize) -> Option<usize> {
    let handle = stream_handle(file)?;
    let mut buf = vec![0u8; len];
    let mut done = 0;
    let mut failed = false;
    if let Some(fh) = emu.handle_management.get_mut_file_handle(handle) {
        while done < len {
            match fh.read(&mut buf[done..]) {
                Ok(0) => break,
                Ok(n) => done += n,
                Err(_) => {
                    failed = true;
                    break;
                }
            }
        }
    }
    emu.maps.write_bytes(dst, &buf[..done]);
    with_stream(file, |s| {
        s.error |= failed;
        s.eof |= !failed && done < len;
    });
    Some(done)
}

/* size_t fread(void *buffer, size_t size, size_t count, FILE *stream); */
/* size_t fwrite(const void *buffer, size_t size, size_t count, FILE *stream); */
/* int fgetc(FILE *stream); */
/* char *fgets(char *str, int numChars, FILE *stream); */
/* int fseek(FILE *stream, long offset, int origin); */
/* long ftell(FILE *stream); __int64 _ftelli64(FILE *stream); */
/* void rewind(FILE *stream); */
/* int feof(FILE *stream); int ferror(FILE *stream); */
/* void clearerr(FILE *stream); */
/* int fflush(FILE *stream); — writes are unbuffered, NULL flushes all */
/* int _fileno(FILE *stream); */
fn fd_handle(fd: i32) -> Option<u32> {
    let state = STATE.lock().unwrap();
    state
        .descriptors
        .iter()
        .find(|d| d.fd == fd)
        .map(|d| d.handle)
}

/* long _filelength(int fd); __int64 _filelengthi64(int fd); */
/* intptr_t _get_osfhandle(int fd); */
