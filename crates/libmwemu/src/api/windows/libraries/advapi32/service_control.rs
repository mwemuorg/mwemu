use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::emu::object_handle::ServiceObject;
use crate::winapi::common::trace::input_string;
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::{self, api_try};
use crate::winapi::registry_store::{REG_EXPAND_SZ, Value, dword, join, sz};
use crate::windows::constants::errors::*;
use crate::windows::constants::security::{GENERIC_ALL, GENERIC_WRITE, MAXIMUM_ALLOWED};

const SERVICES_KEY: &str = r"HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services";
const ACTIVE_DATABASE: &str = "ServicesActive";
const SC_MANAGER_CREATE_SERVICE: u32 = 2;
const SERVICE_WIN32_OWN_PROCESS: u32 = 0x10;
const SERVICE_WIN32_SHARE_PROCESS: u32 = 0x20;
const SERVICE_AUTO_START: u32 = 2;
const SERVICE_DISABLED: u32 = 4;
const SERVICE_ERROR_CRITICAL: u32 = 3;
const MAX_SERVICE_NAME_UNITS: usize = 256;

fn name(emu: &Emu, address: u64, wide: bool) -> Result<String, u64> {
    let units = Encoding::from_wide(wide).read(emu, address)?;
    String::from_utf16(&units).map_err(|_| ERROR_INVALID_NAME)
}

fn checked_handle(emu: &mut Emu, result: Result<u64, u64>) -> u64 {
    result.unwrap_or_else(|error| {
        emu.set_last_error(error);
        0
    })
}

fn open_manager(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let machine_ptr = api_try!(emu, call.arg(emu, 0));
    let database_ptr = api_try!(emu, call.arg(emu, 1));
    let desired_access = api_try!(emu, call.arg_u32(emu, 2));
    let machine = (machine_ptr != 0)
        .then(|| name(emu, machine_ptr, wide))
        .transpose();
    let database = (database_ptr != 0)
        .then(|| name(emu, database_ptr, wide))
        .transpose();
    log_red!(
        emu,
        "advapi32!{api} machine_name={} database_name={} desired_access={desired_access:#x}",
        input_string(emu, machine_ptr, Encoding::from_wide(wide)),
        input_string(emu, database_ptr, Encoding::from_wide(wide))
    );
    let result = (|| {
        if machine?
            .is_some_and(|s| !s.is_empty() && s != "." && !s.eq_ignore_ascii_case("localhost"))
        {
            return Err(ERROR_BAD_NETPATH);
        }
        if database?.is_some_and(|s| !s.is_empty() && !s.eq_ignore_ascii_case(ACTIVE_DATABASE)) {
            return Err(ERROR_DATABASE_DOES_NOT_EXIST);
        }
        Ok(emu
            .handle_management
            .open_service_object(ServiceObject::Manager {
                access: desired_access,
            }))
    })();
    let handle = checked_handle(emu, result);
    call.set_return(emu, handle);
    call.cleanup_args(emu, 3);
}

fn validate_service_name(service: &str) -> Result<(), u64> {
    if service.is_empty()
        || service.encode_utf16().count() > MAX_SERVICE_NAME_UNITS
        || service.contains(['/', '\\'])
    {
        return Err(ERROR_INVALID_NAME);
    }
    Ok(())
}

fn open_service(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let scm_handle = api_try!(emu, call.arg(emu, 0));
    let service_ptr = api_try!(emu, call.arg(emu, 1));
    let desired_access = api_try!(emu, call.arg_u32(emu, 2));
    let service = name(emu, service_ptr, wide);
    log_red!(
        emu,
        "advapi32!{api} scm_handle={scm_handle:#x} service_name={} desired_access={desired_access:#x}",
        input_string(emu, service_ptr, Encoding::from_wide(wide))
    );
    let result = (|| {
        if emu
            .handle_management
            .service_manager_access(scm_handle)
            .is_none()
        {
            return Err(ERROR_INVALID_HANDLE);
        }
        let service = service?;
        validate_service_name(&service)?;
        if !emu.registry.key_exists(&join(SERVICES_KEY, &service)) {
            return Err(ERROR_SERVICE_DOES_NOT_EXIST);
        }
        Ok(emu
            .handle_management
            .open_service_object(ServiceObject::Service {
                name: service,
                access: desired_access,
            }))
    })();
    let handle = checked_handle(emu, result);
    call.set_return(emu, handle);
    call.cleanup_args(emu, 3);
}

pub fn OpenSCManagerA(emu: &mut Emu) {
    open_manager(emu, "OpenSCManagerA", false);
}

pub fn OpenSCManagerW(emu: &mut Emu) {
    open_manager(emu, "OpenSCManagerW", true);
}

pub fn OpenServiceA(emu: &mut Emu) {
    open_service(emu, "OpenServiceA", false);
}

pub fn OpenServiceW(emu: &mut Emu) {
    open_service(emu, "OpenServiceW", true);
}

pub fn CloseServiceHandle(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let service_handle = api_try!(emu, call.arg(emu, 0));
    log_red!(
        emu,
        "advapi32!CloseServiceHandle service_handle={service_handle:#x}"
    );
    let closed = emu.handle_management.close_service_object(service_handle);
    if !closed {
        emu.set_last_error(ERROR_INVALID_HANDLE);
    }
    call.set_return(emu, closed.into());
    call.cleanup_args(emu, 1);
}

fn create_service(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let [
        scm_handle,
        service_ptr,
        display_ptr,
        desired_access,
        service_type,
        start_type,
        error_control,
        binary_ptr,
        load_order_group,
        tag_id,
        dependencies,
        account,
        password,
    ] = api_try!(emu, call.args(emu));
    let service = name(emu, service_ptr, wide);
    let binary = name(emu, binary_ptr, wide);
    let display = (display_ptr != 0)
        .then(|| name(emu, display_ptr, wide))
        .transpose();
    log_red!(
        emu,
        "advapi32!{api} scm_handle={scm_handle:#x} service_name={} display_name={} desired_access={:#x} service_type={:#x} start_type={:#x} error_control={:#x} binary_path={}",
        input_string(emu, service_ptr, Encoding::from_wide(wide)),
        input_string(emu, display_ptr, Encoding::from_wide(wide)),
        desired_access as u32,
        service_type as u32,
        start_type as u32,
        error_control as u32,
        input_string(emu, binary_ptr, Encoding::from_wide(wide))
    );
    let granted = emu.handle_management.service_manager_access(scm_handle);
    let result = if granted.is_none() {
        Err(ERROR_INVALID_HANDLE)
    } else if !granted.is_some_and(|rights| {
        rights & (SC_MANAGER_CREATE_SERVICE | GENERIC_WRITE | GENERIC_ALL | MAXIMUM_ALLOWED) != 0
    }) {
        Err(ERROR_ACCESS_DENIED)
    } else if !matches!(
        service_type as u32,
        SERVICE_WIN32_OWN_PROCESS | SERVICE_WIN32_SHARE_PROCESS
    ) || [load_order_group, tag_id, dependencies, account, password]
        .into_iter()
        .any(|ptr| ptr != 0)
    {
        policy::unsupported(emu, "advapi32", api, emu.pc());
        return;
    } else {
        (|| {
            let (service, binary, display) = (service?, binary?, display?);
            validate_service_name(&service)?;
            if binary.is_empty()
                || !(SERVICE_AUTO_START..=SERVICE_DISABLED).contains(&(start_type as u32))
                || error_control as u32 > SERVICE_ERROR_CRITICAL
            {
                return Err(ERROR_INVALID_PARAMETER);
            }
            let path = join(SERVICES_KEY, &service);
            if emu.registry.key_exists(&path) {
                return Err(ERROR_SERVICE_EXISTS);
            }
            if emu.registry.create_key(&path) != Some(true) {
                return Err(ERROR_INVALID_PARAMETER);
            }
            for (key, value) in [
                ("Type", dword(service_type as u32)),
                ("Start", dword(start_type as u32)),
                ("ErrorControl", dword(error_control as u32)),
                (
                    "ImagePath",
                    Value {
                        kind: REG_EXPAND_SZ,
                        ..sz(&binary)
                    },
                ),
                ("DisplayName", sz(display.as_deref().unwrap_or(&service))),
            ] {
                emu.registry.set_value(&path, key, value);
            }
            Ok(emu
                .handle_management
                .open_service_object(ServiceObject::Service {
                    name: service,
                    access: desired_access as u32,
                }))
        })()
    };
    let handle = checked_handle(emu, result);
    call.set_return(emu, handle);
    call.cleanup_args(emu, 13);
}

pub fn CreateServiceA(emu: &mut Emu) {
    create_service(emu, "CreateServiceA", false);
}

pub fn CreateServiceW(emu: &mut Emu) {
    create_service(emu, "CreateServiceW", true);
}
