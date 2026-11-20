use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::{ERROR_MOD_NOT_FOUND, ERROR_PROC_NOT_FOUND};

pub fn GetProcAddress(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let module = api_try!(emu, call.arg(emu, 0));
    let symbol = api_try!(emu, call.arg(emu, 1));
    let ordinal = symbol <= u16::MAX as u64;
    let name = if ordinal {
        format!("#{symbol}")
    } else {
        emu.maps.read_string(symbol)
    };
    let result = if ordinal {
        emu.export_indexes
            .resolve_ordinal_by_base(module, symbol as u32)
    } else {
        emu.export_indexes.resolve_name_by_base(module, &name)
    };
    if result == 0 {
        let error = if emu.export_indexes.get_by_base(module).is_some() {
            ERROR_PROC_NOT_FOUND
        } else {
            ERROR_MOD_NOT_FOUND
        };
        emu.set_last_error(error);
    }
    log_red!(
        emu,
        "kernel32!GetProcAddress module=0x{:x} name={} => 0x{:x}",
        module,
        name,
        result
    );
    call.cleanup_args(emu, 2);
    call.set_return(emu, result);
}
