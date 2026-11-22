use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::{buffers, memory};
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

// Capacity includes the UTF-16 terminator.
const APP_ID_CAPACITY: usize = 128;

pub fn SetCurrentProcessExplicitAppUserModelID(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let source = api_try!(emu, call.arg(emu, 0));
    log_red!(
        emu,
        "shell32!SetCurrentProcessExplicitAppUserModelID source={source:#x}"
    );
    let result = match Encoding::Wide.read(emu, source) {
        Ok(units) if units.len() < APP_ID_CAPACITY => {
            emu.app_user_model_id = units;
            S_OK
        }
        _ => HRESULT_E_INVALID_ARG,
    };
    call.set_return(emu, result);
    call.cleanup_args(emu, 1);
}

pub fn GetCurrentProcessExplicitAppUserModelID(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let output = api_try!(emu, call.arg(emu, 0));
    log_red!(
        emu,
        "shell32!GetCurrentProcessExplicitAppUserModelID output={output:#x}"
    );
    let result = if output == 0 || !memory::write_pointer(emu, output, 0) {
        E_POINTER
    } else if emu.app_user_model_id.is_empty() {
        E_FAIL
    } else {
        let encoded = Encoding::Wide.terminated(&emu.app_user_model_id);
        buffers::return_task_string(emu, output, &encoded)
    };
    call.set_return(emu, result);
    call.cleanup_args(emu, 1);
}
