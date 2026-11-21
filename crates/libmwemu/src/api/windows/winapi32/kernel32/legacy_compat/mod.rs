use crate::emu::Emu;
use crate::winapi::winapi32::kernel32;

mod __tlg_create_wsz;
pub use __tlg_create_wsz::_TlgCreateWsz;

mod _guard_check_icall_nop;

mod _lclose;
pub use _lclose::_lclose;

mod _lcreat;
pub use _lcreat::_lcreat;

mod _llseek;
pub use _llseek::_llseek;

mod _lopen;
pub use _lopen::_lopen;

mod _lread;
pub use _lread::_lread;

mod _lwrite;
pub use _lwrite::_lwrite;

mod _nlg__dispatch2;

mod _nlg__notify;

mod _tlg_enable_callback;

mod _tlg_keyword_on;

mod _tlg_write_transfer__event_write_transfer;

mod base_destroy_vdm_environment;
pub use base_destroy_vdm_environment::BaseDestroyVDMEnvironment;

mod base_update_vdm_entry;
pub use base_update_vdm_entry::BaseUpdateVDMEntry;

mod nt_vdm64_create_process_internal_w;
pub use nt_vdm64_create_process_internal_w::NtVdm64CreateProcessInternalW;

mod nt_wow64_console_launch_server_process;
pub use nt_wow64_console_launch_server_process::NtWow64ConsoleLaunchServerProcess;

mod nt_wow64_csr_base_check_run_app;
pub use nt_wow64_csr_base_check_run_app::NtWow64CsrBaseCheckRunApp;

mod nt_wow64_csr_basep_create_act_ctx;
pub use nt_wow64_csr_basep_create_act_ctx::NtWow64CsrBasepCreateActCtx;

mod nt_wow64_csr_basep_create_process;
pub use nt_wow64_csr_basep_create_process::NtWow64CsrBasepCreateProcess;

mod nt_wow64_csr_basep_define_dos_device;
pub use nt_wow64_csr_basep_define_dos_device::NtWow64CsrBasepDefineDosDevice;

mod nt_wow64_csr_basep_get_process_shutdown_param;
pub use nt_wow64_csr_basep_get_process_shutdown_param::NtWow64CsrBasepGetProcessShutdownParam;

mod nt_wow64_csr_basep_nls_get_user_info;
pub use nt_wow64_csr_basep_nls_get_user_info::NtWow64CsrBasepNlsGetUserInfo;

mod nt_wow64_csr_basep_nls_update_cache_count;
pub use nt_wow64_csr_basep_nls_update_cache_count::NtWow64CsrBasepNlsUpdateCacheCount;

mod nt_wow64_csr_basep_refresh_ini_file_mapping;
pub use nt_wow64_csr_basep_refresh_ini_file_mapping::NtWow64CsrBasepRefreshIniFileMapping;

mod nt_wow64_csr_basep_set_client_time_zone_information;
pub use nt_wow64_csr_basep_set_client_time_zone_information::NtWow64CsrBasepSetClientTimeZoneInformation;

mod nt_wow64_csr_basep_set_process_shutdown_param;
pub use nt_wow64_csr_basep_set_process_shutdown_param::NtWow64CsrBasepSetProcessShutdownParam;

mod nt_wow64_csr_basep_sound_sentry_notification;
pub use nt_wow64_csr_basep_sound_sentry_notification::NtWow64CsrBasepSoundSentryNotification;

mod register_console_vdm;
pub use register_console_vdm::RegisterConsoleVDM;

mod vdm_console_operation;
pub use vdm_console_operation::VDMConsoleOperation;

mod vdm_operation_started;
pub use vdm_operation_started::VDMOperationStarted;

mod wow64_enable_wow64_fs_redirection;
pub use wow64_enable_wow64_fs_redirection::Wow64EnableWow64FsRedirection;

mod wow64_get_thread_selector_entry;
pub use wow64_get_thread_selector_entry::Wow64GetThreadSelectorEntry;

mod wow64_suspend_thread;
pub use wow64_suspend_thread::Wow64SuspendThread;

mod wow64_system_service_call;
pub use wow64_system_service_call::Wow64SystemServiceCall;

mod wow64_transition_resolve;
pub use wow64_transition_resolve::Wow64TransitionResolve;

mod write_console_input_vdm;
pub use write_console_input_vdm::{WriteConsoleInputVDMA, WriteConsoleInputVDMW};

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    match api {
        "_TlgCreateWsz" => kernel32::_TlgCreateWsz(emu),
        "_lclose" => kernel32::_lclose(emu),
        "_lcreat" => kernel32::_lcreat(emu),
        "_llseek" => kernel32::_llseek(emu),
        "_lopen" => kernel32::_lopen(emu),
        "_lread" => kernel32::_lread(emu),
        "_lwrite" => kernel32::_lwrite(emu),
        "_hread" => kernel32::_hread(emu),
        "_hwrite" => kernel32::_hwrite(emu),
        "__C_specific_handler" => kernel32::__C_specific_handler(emu),
        "__chkstk" => kernel32::__chkstk(emu),
        "__misaligned_access" => kernel32::__misaligned_access(emu),
        "_local_unwind" => kernel32::_local_unwind(emu),
        _ => return false,
    }
    true
}
