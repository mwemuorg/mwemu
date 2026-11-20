use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::memory::{writable_pointer, write_pointer};
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

const STRSAFE_MAX_CCH: u64 = i32::MAX as u64;
const WIDE_UNIT: u64 = size_of::<u16>() as u64;

fn read_unit(emu: &Emu, text: u64, index: u64) -> Option<u16> {
    text.checked_add(index.checked_mul(WIDE_UNIT)?)
        .and_then(|at| emu.maps.read_word(at))
}

pub(crate) fn strsafe(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let destination = api_try!(emu, call.arg(emu, 0));
    let mut capacity = api_try!(emu, call.arg(emu, 1));
    let source = api_try!(emu, call.arg(emu, 2));
    let bounded = api == "StringCchCopyNW";
    if api == "StringCbCopyW" {
        capacity /= WIDE_UNIT;
    }
    let limit = if bounded {
        api_try!(emu, call.arg(emu, 3))
    } else {
        STRSAFE_MAX_CCH
    };
    let status = if api == "StringCchLengthW" {
        log_red!(
            emu,
            "{library}!{api} source={destination:#x} max_length={capacity} length={source:#x}"
        );
        let count = if destination != 0 && capacity <= STRSAFE_MAX_CCH {
            (0..capacity)
                .map_while(|index| read_unit(emu, destination, index))
                .position(|unit| unit == 0)
        } else {
            None
        };
        if source != 0
            && (!writable_pointer(emu, source)
                || !write_pointer(emu, source, count.unwrap_or_default() as u64))
        {
            HRESULT_E_INVALID_ARG
        } else if count.is_some() {
            S_OK
        } else {
            HRESULT_E_INVALID_ARG
        }
    } else {
        log_red!(
            emu,
            "{library}!{api} destination={destination:#x} capacity={capacity} source={source:#x} max_copy={limit}"
        );
        copy_wide(
            emu,
            destination,
            capacity,
            source,
            limit,
            api == "StringCchCatW",
        )
    };
    call.set_return(emu, status);
    call.cleanup_args(emu, 3 + usize::from(bounded));
}

fn copy_wide(
    emu: &mut Emu,
    dest: u64,
    capacity: u64,
    source: u64,
    limit: u64,
    append: bool,
) -> u64 {
    if dest == 0 || !(1..=STRSAFE_MAX_CCH).contains(&capacity) {
        return HRESULT_E_INVALID_ARG;
    }
    if limit > STRSAFE_MAX_CCH {
        emu.maps.write_word(dest, 0);
        return HRESULT_E_INVALID_ARG;
    }
    let start = if append {
        let Some(count) = (0..capacity)
            .map_while(|index| read_unit(emu, dest, index))
            .position(|unit| unit == 0)
        else {
            return HRESULT_E_INVALID_ARG;
        };
        count as u64
    } else {
        0
    };
    let space = capacity - start - 1;
    let mut content = Vec::new();
    let mut status = S_OK;
    for index in 0..limit {
        let Some(unit) = read_unit(emu, source, index) else {
            return HRESULT_E_INVALID_ARG;
        };
        if unit == 0 {
            break;
        }
        if index == space {
            status = HRESULT_INSUFFICIENT_BUFFER;
            break;
        }
        content.extend(unit.to_le_bytes());
    }
    content.extend(0u16.to_le_bytes());
    let Some(out) = dest.checked_add(start * WIDE_UNIT) else {
        return HRESULT_E_INVALID_ARG;
    };
    if emu.maps.validate_write_range(out, content.len() as u64)
        && emu.maps.write_bytes(out, &content)
    {
        status
    } else {
        HRESULT_E_INVALID_ARG
    }
}
