use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn BackupWrite(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _hFile = api_try!(emu, call.arg_u32(emu, 0));
    let _lpBuffer = api_try!(emu, call.arg_u32(emu, 1));
    let _nNumberOfBytesToWrite = api_try!(emu, call.arg_u32(emu, 2));
    let _lpNumberOfBytesWritten = api_try!(emu, call.arg_u32(emu, 3));
    let _bAbort = api_try!(emu, call.arg_u32(emu, 4));
    let _bProcessSecurity = api_try!(emu, call.arg_u32(emu, 5));
    let _lpContext = api_try!(emu, call.arg_u32(emu, 6));

    log_red!(emu, "kernel32!BackupWrite");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 7);
}
