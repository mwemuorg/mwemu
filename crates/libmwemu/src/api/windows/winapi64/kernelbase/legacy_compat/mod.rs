use crate::emu::Emu;
use crate::winapi::winapi64::kernelbase;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
        "_initterm_e" => kernelbase::_initterm_e(emu),
        "_initterm" => kernelbase::_initterm(emu),
        "exit" => kernelbase::exit(emu),
        "_exit" => kernelbase::_exit(emu),
        "atexit" => kernelbase::atexit(emu),
        "__FC_Query_Config" => kernelbase::__FC_Query_Config(emu),
        "__FC_Query_System" => kernelbase::__FC_Query_System(emu),
        "__FC_Report" => kernelbase::__FC_Report(emu),
        "__FC_Subscribe" => kernelbase::__FC_Subscribe(emu),
        "__FC_Unsubscribe" => kernelbase::__FC_Unsubscribe(emu),
        "__argc" => kernelbase::__argc(emu),
        "__dllonexit3" => kernelbase::__dllonexit3(emu),
        "__wargv" => kernelbase::__wargv(emu),
        "__wgetmainargs" => kernelbase::__wgetmainargs(emu),
        "_amsg_exit" => kernelbase::_amsg_exit(emu),
        "_c_exit" => kernelbase::_c_exit(emu),
        "_cexit" => kernelbase::_cexit(emu),
        "_invalid_parameter" => kernelbase::_invalid_parameter(emu),
        "_onexit" => kernelbase::_onexit(emu),
        "_purecall" => kernelbase::_purecall(emu),
        "_time64" => kernelbase::_time64(emu),
        "hgets" => kernelbase::hgets(emu),
        "hwprintf" => kernelbase::hwprintf(emu),
        "wprintf" => kernelbase::wprintf(emu),
        "__C_specific_handler" => kernelbase::__C_specific_handler(emu),
        "__chkstk" => kernelbase::__chkstk(emu),
        "__misaligned_access" => kernelbase::__misaligned_access(emu),
        "_local_unwind" => kernelbase::_local_unwind(emu),
        _ => return false,
    }
    true
}
