//! Bounded activation with no registered COM classes or object factories.
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::winapi::common::memory::{pointer_size, read_pointer, write_pointer};
use crate::winapi::common::trace::input_string;
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

const GUID_BYTES: usize = 16;
const MULTI_QI_POINTER_SLOTS: u64 = 3;

pub fn CoCreateInstance(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let class = api_try!(emu, call.arg(emu, 0));
    let iid = api_try!(emu, call.arg(emu, 3));
    let output = api_try!(emu, call.arg(emu, 4));
    let status = if !readable_guid(emu, class)
        || !readable_guid(emu, iid)
        || output == 0
        || !write_pointer(emu, output, 0)
    {
        E_POINTER
    } else {
        REGDB_E_CLASSNOTREG
    };
    log_red!(
        emu,
        "ole32!CoCreateInstance class=0x{:x} iid=0x{:x} output=0x{:x} => 0x{:x}",
        class,
        iid,
        output,
        status
    );
    call.cleanup_args(emu, 5);
    call.set_return(emu, status);
}

pub fn CoCreateInstanceEx(emu: &mut Emu) {
    create_instances(emu, "CoCreateInstanceEx");
}

pub fn CoCreateInstanceFromApp(emu: &mut Emu) {
    create_instances(emu, "CoCreateInstanceFromApp");
}

fn create_instances(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let class = api_try!(emu, call.arg(emu, 0));
    let server = api_try!(emu, call.arg(emu, 3));
    let count = api_try!(emu, call.arg_u32(emu, 4));
    let results = api_try!(emu, call.arg(emu, 5));
    let status = if !readable_guid(emu, class) {
        E_POINTER
    } else {
        fail_interfaces(
            emu,
            count,
            results,
            if server == 0 {
                REGDB_E_CLASSNOTREG
            } else {
                E_NOTIMPL
            },
        )
    };
    log_red!(
        emu,
        "ole32!{} class=0x{:x} count={} results=0x{:x} => 0x{:x}",
        api,
        class,
        count,
        results,
        status
    );
    call.cleanup_args(emu, 6);
    call.set_return(emu, status);
}

pub fn CoGetInstanceFromFile(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let class = api_try!(emu, call.arg(emu, 1));
    let filename = api_try!(emu, call.arg(emu, 5));
    let count = api_try!(emu, call.arg_u32(emu, 6));
    let results = api_try!(emu, call.arg(emu, 7));
    let status = if filename == 0 || (class != 0 && !readable_guid(emu, class)) {
        E_POINTER
    } else {
        fail_interfaces(emu, count, results, E_NOTIMPL)
    };
    log_red!(
        emu,
        "ole32!CoGetInstanceFromFile file={} count={} results=0x{:x} status=0x{:x}",
        input_string(emu, filename, Encoding::Wide),
        count,
        results,
        status
    );
    call.cleanup_args(emu, 8);
    call.set_return(emu, status);
}

pub fn CoGetInstanceFromIStorage(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let class = api_try!(emu, call.arg(emu, 1));
    let storage = api_try!(emu, call.arg(emu, 4));
    let count = api_try!(emu, call.arg_u32(emu, 5));
    let results = api_try!(emu, call.arg(emu, 6));
    let status = if storage == 0 || (class != 0 && !readable_guid(emu, class)) {
        E_POINTER
    } else {
        fail_interfaces(emu, count, results, E_NOTIMPL)
    };
    log_red!(
        emu,
        "ole32!CoGetInstanceFromIStorage storage=0x{:x} count={} results=0x{:x} => 0x{:x}",
        storage,
        count,
        results,
        status
    );
    call.cleanup_args(emu, 7);
    call.set_return(emu, status);
}

fn readable_guid(emu: &Emu, address: u64) -> bool {
    address != 0
        && emu
            .maps
            .can_access_range(address, GUID_BYTES, Permission::READ)
}

fn fail_interfaces(emu: &mut Emu, count: u32, results: u64, status: u64) -> u64 {
    if count == 0 {
        return HRESULT_E_INVALID_ARG;
    }
    let width = pointer_size(emu);
    let stride = MULTI_QI_POINTER_SLOTS * width;
    let Some(size) = (count as u64)
        .checked_mul(stride)
        .and_then(|size| usize::try_from(size).ok())
    else {
        return HRESULT_E_INVALID_ARG;
    };
    if results == 0 || !emu.maps.can_access_range(results, size, Permission::READ) {
        return E_POINTER;
    }
    let mut output = vec![0; width as usize];
    output.extend((status as u32).to_le_bytes());
    // Validate the whole request before publishing any interface result.
    for index in 0..count as u64 {
        let address = results + index * stride;
        if !read_pointer(emu, address).is_some_and(|iid| readable_guid(emu, iid))
            || !emu
                .maps
                .validate_write_range(address + width, output.len() as u64)
        {
            return E_POINTER;
        }
    }
    for index in 0..count as u64 {
        if !emu
            .maps
            .write_bytes(results + index * stride + width, &output)
        {
            return E_POINTER;
        }
    }
    status
}
