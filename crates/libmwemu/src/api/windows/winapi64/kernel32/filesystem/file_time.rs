use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::constants::system::*;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::emu;

/// 100 ns intervals between 1601-01-01 (FILETIME epoch) and 1970-01-01.
const UNIX_EPOCH_AS_FILETIME: u64 = 116_444_736_000_000_000;

fn to_filetime(time: SystemTime) -> u64 {
    match time.duration_since(UNIX_EPOCH) {
        Ok(since) => UNIX_EPOCH_AS_FILETIME + (since.as_nanos() / 100) as u64,
        Err(before) => {
            UNIX_EPOCH_AS_FILETIME.saturating_sub((before.duration().as_nanos() / 100) as u64)
        }
    }
}

fn from_filetime(filetime: u64) -> SystemTime {
    if filetime >= UNIX_EPOCH_AS_FILETIME {
        UNIX_EPOCH + Duration::from_nanos((filetime - UNIX_EPOCH_AS_FILETIME) * 100)
    } else {
        UNIX_EPOCH - Duration::from_nanos((UNIX_EPOCH_AS_FILETIME - filetime) * 100)
    }
}

/*
BOOL GetFileTime(
  [in]            HANDLE     hFile,
  [out, optional] LPFILETIME lpCreationTime,
  [out, optional] LPFILETIME lpLastAccessTime,
  [out, optional] LPFILETIME lpLastWriteTime
);
*/
pub fn GetFileTime(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hfile = api_try!(emu, call.arg(emu, 0));
    let out = [
        api_try!(emu, call.arg(emu, 1)),
        api_try!(emu, call.arg(emu, 2)),
        api_try!(emu, call.arg(emu, 3)),
    ];

    let times = match emu.handle_management.get_mut_file_handle(hfile as u32) {
        Some(file) => file.times().ok(),
        None => None,
    };
    log_red!(emu, "kernel32!GetFileTime hFile:0x{:x} {:?}", hfile, times);

    let Some(times) = times else {
        emu.set_last_error(ERROR_INVALID_HANDLE);
        call.set_return(emu, FALSE);
        return;
    };
    for (ptr, time) in out.into_iter().zip(times) {
        if ptr != 0 {
            emu.maps.write_qword(ptr, to_filetime(time));
        }
    }
    call.set_return(emu, TRUE);
}

/*
BOOL SetFileTime(
  [in]           HANDLE         hFile,
  [in, optional] const FILETIME *lpCreationTime,
  [in, optional] const FILETIME *lpLastAccessTime,
  [in, optional] const FILETIME *lpLastWriteTime
);
*/
pub fn SetFileTime(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hfile = api_try!(emu, call.arg(emu, 0));
    // NULL, 0 and 0xffffffff_ffffffff (stop updating) all leave a time unchanged
    let read = |emu: &emu::Emu, ptr: u64| {
        if ptr == 0 {
            return None;
        }
        match emu.maps.read_qword(ptr) {
            Some(0) | Some(u64::MAX) | None => None,
            Some(filetime) => Some(from_filetime(filetime)),
        }
    };
    let created = read(emu, api_try!(emu, call.arg(emu, 1)));
    let accessed = read(emu, api_try!(emu, call.arg(emu, 2)));
    let written = read(emu, api_try!(emu, call.arg(emu, 3)));

    log_red!(
        emu,
        "kernel32!SetFileTime hFile:0x{:x} created:{:?} accessed:{:?} written:{:?}",
        hfile,
        created,
        accessed,
        written
    );

    let result = match emu.handle_management.get_mut_file_handle(hfile as u32) {
        Some(file) => file
            .set_times(created, accessed, written)
            .map_err(|_| ERROR_ACCESS_DENIED),
        None => Err(ERROR_INVALID_HANDLE),
    };
    match result {
        Ok(()) => call.set_return(emu, TRUE),
        Err(error) => {
            emu.set_last_error(error);
            call.set_return(emu, FALSE);
        }
    }
}
