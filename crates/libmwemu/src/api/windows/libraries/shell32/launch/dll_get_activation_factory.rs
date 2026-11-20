use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::memory::write_pointer;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::{HRESULT_E_INVALID_ARG, REGDB_E_CLASSNOTREG};

pub fn DllGetActivationFactory(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [class_id, output] = api_try!(emu, call.args(emu));
    let result = if output == 0 || !write_pointer(emu, output, 0) || class_id == 0 {
        HRESULT_E_INVALID_ARG
    } else {
        REGDB_E_CLASSNOTREG
    };
    log_red!(
        emu,
        "shell32!DllGetActivationFactory class_id={class_id:#x} factory_output={output:#x} status={result:#x}"
    );
    call.set_return(emu, result);
    call.cleanup_args(emu, 2);
}
