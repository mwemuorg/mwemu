use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::exception::types::ExceptionType;
use crate::winapi::common::trace;
use crate::winapi::policy::api_try;

#[derive(Clone, Copy)]
pub(crate) enum Operation {
    Copy,
    Fill,
    Zero,
    SecureZero,
    Compare,
}

pub(crate) fn matching_prefix(emu: &Emu, left: u64, right: u64, length: u64) -> Option<u64> {
    for offset in 0..length {
        let a = emu.maps.read_byte(left.checked_add(offset)?)?;
        let b = emu.maps.read_byte(right.checked_add(offset)?)?;
        if a != b {
            return Some(offset);
        }
    }
    Some(length)
}

pub(crate) fn run(emu: &mut Emu, operation: Operation) {
    let call = WinApi(Stdcall);
    let dest = api_try!(emu, call.arg(emu, 0));
    let (result, count) = match operation {
        Operation::Copy => {
            let source = api_try!(emu, call.arg(emu, 1));
            let size = api_try!(emu, call.arg(emu, 2));
            (
                emu.maps
                    .copy_range(dest, source, size as usize)
                    .then_some(0),
                3,
            )
        }
        Operation::Fill => {
            let size = api_try!(emu, call.arg(emu, 1));
            let value = api_try!(emu, call.arg_u8(emu, 2));
            (
                emu.maps.fill_range(dest, value, size as usize).then_some(0),
                3,
            )
        }
        Operation::Zero | Operation::SecureZero => {
            let size = api_try!(emu, call.arg(emu, 1));
            let result = if matches!(operation, Operation::SecureZero) {
                dest
            } else {
                0
            };
            (
                emu.maps
                    .fill_range(dest, 0, size as usize)
                    .then_some(result),
                2,
            )
        }
        Operation::Compare => {
            let other = api_try!(emu, call.arg(emu, 1));
            let size = api_try!(emu, call.arg(emu, 2));
            (matching_prefix(emu, dest, other, size), 3)
        }
    };
    if let Some(result) = result {
        call.set_return(emu, result);
        call.cleanup_args(emu, count);
    } else {
        emu.exception(ExceptionType::BadAddressDereferencing);
    }
}

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    let (operation, parameters) = match api {
        "CopyMemoryNonTemporal"
        | "RtlCopyMemory"
        | "RtlMoveMemory"
        | "CopyMemory"
        | "MoveMemory" => (Operation::Copy, "destination source size"),
        "RtlFillMemory" | "FillMemory" => (Operation::Fill, "destination size value"),
        "RtlZeroMemory" | "ZeroMemory" => (Operation::Zero, "destination size"),
        "RtlSecureZeroMemory" => (Operation::SecureZero, "destination size"),
        "RtlCompareMemory" => (Operation::Compare, "source1 source2 size"),
        _ => return false,
    };
    let returns_value = matches!(operation, Operation::SecureZero | Operation::Compare);
    trace::api(emu, api, parameters, returns_value, |emu| {
        run(emu, operation)
    });
    true
}
