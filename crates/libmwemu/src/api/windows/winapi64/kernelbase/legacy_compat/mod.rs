use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::winapi64::kernelbase;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "_initterm_e" => kernelbase::_initterm_e(emu),
        "_initterm" => kernelbase::_initterm(emu),
        "__argc" => kernelbase::__argc(emu),
        "__wargv" => kernelbase::__wargv(emu),
        "__C_specific_handler" => kernelbase::__C_specific_handler(emu),
        "__chkstk" => kernelbase::__chkstk(emu),
        "__misaligned_access" => kernelbase::__misaligned_access(emu),
        "_local_unwind" => kernelbase::_local_unwind(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
