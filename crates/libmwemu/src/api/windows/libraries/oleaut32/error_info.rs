use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::memory::{writable_pointer, write_pointer};
use crate::winapi::policy::{api_try, unsupported};
use crate::windows::constants::errors::{E_POINTER, HRESULT_E_INVALID_ARG, S_FALSE, S_OK};

pub fn GetErrorInfo(emu: &mut Emu) {
    get_error_info(emu, "oleaut32");
}

pub fn SetErrorInfo(emu: &mut Emu) {
    set_error_info(emu, "oleaut32");
}

pub fn get_ole32_error_info(emu: &mut Emu) {
    get_error_info(emu, "ole32");
}

pub fn set_ole32_error_info(emu: &mut Emu) {
    set_error_info(emu, "ole32");
}

fn get_error_info(emu: &mut Emu, dll: &str) {
    let call = WinApi(Stdcall);
    let reserved = api_try!(emu, call.arg_u32(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
    let status = if output == 0 || !writable_pointer(emu, output) || !write_pointer(emu, output, 0)
    {
        E_POINTER
    } else if reserved != 0 {
        HRESULT_E_INVALID_ARG
    } else {
        S_FALSE
    };
    log_red!(
        emu,
        "{dll}!GetErrorInfo reserved={reserved} output={output:#x} => {status:#x}"
    );
    call.cleanup_args(emu, 2);
    call.set_return(emu, status);
}

fn set_error_info(emu: &mut Emu, dll: &str) {
    let call = WinApi(Stdcall);
    let reserved = api_try!(emu, call.arg_u32(emu, 0));
    let error_info = api_try!(emu, call.arg(emu, 1));
    let status = if reserved != 0 {
        HRESULT_E_INVALID_ARG
    } else if error_info == 0 {
        S_OK
    } else {
        // Holding a guest IErrorInfo would require AddRef and Release on its vtable.
        unsupported(emu, dll, "SetErrorInfo", emu.pc());
        return;
    };
    log_red!(
        emu,
        "{dll}!SetErrorInfo reserved={reserved} error_info={error_info:#x} => {status:#x}"
    );
    call.cleanup_args(emu, 2);
    call.set_return(emu, status);
}
