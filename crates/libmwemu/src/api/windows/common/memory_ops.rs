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
}

pub(crate) fn run(emu: &mut Emu, operation: Operation) {
    let call = WinApi(Stdcall);
    let dest = api_try!(emu, call.arg(emu, 0));
    let (ok, count) = match operation {
        Operation::Copy => {
            let source = api_try!(emu, call.arg(emu, 1));
            let size = api_try!(emu, call.arg(emu, 2));
            (emu.maps.copy_range(dest, source, size as usize), 3)
        }
        Operation::Fill => {
            let size = api_try!(emu, call.arg(emu, 1));
            let value = api_try!(emu, call.arg_u8(emu, 2));
            (emu.maps.fill_range(dest, value, size as usize), 3)
        }
        Operation::Zero => {
            let size = api_try!(emu, call.arg(emu, 1));
            (emu.maps.fill_range(dest, 0, size as usize), 2)
        }
    };
    if ok {
        call.set_return(emu, 0);
        call.cleanup_args(emu, count);
    } else {
        emu.exception(ExceptionType::BadAddressDereferencing);
    }
}

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    let (operation, parameters) = match api {
        "RtlCopyMemory" | "RtlMoveMemory" | "CopyMemory" | "MoveMemory" => {
            (Operation::Copy, "destination source size")
        }
        "RtlFillMemory" | "FillMemory" => (Operation::Fill, "destination size value"),
        "RtlZeroMemory" | "RtlSecureZeroMemory" | "ZeroMemory" => {
            (Operation::Zero, "destination size")
        }
        _ => return false,
    };
    trace::api(emu, api, parameters, false, |emu| run(emu, operation));
    true
}
