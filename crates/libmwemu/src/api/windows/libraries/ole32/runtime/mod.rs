pub(crate) use apartment::Apartment;
mod apartment;
pub use apartment::{
    CoInitialize, CoInitializeEx, CoTestCancel, CoUninitialize, OleBuildVersion, OleInitialize,
    OleUninitialize,
};

pub use crate::winapi::libraries::oleaut32::error_info::{
    get_ole32_error_info as GetErrorInfo, set_ole32_error_info as SetErrorInfo,
};

mod co_task_mem_realloc;
pub use co_task_mem_realloc::CoTaskMemRealloc;

mod activation;
pub use activation::{
    CoCreateInstance, CoCreateInstanceEx, CoCreateInstanceFromApp, CoGetInstanceFromFile,
    CoGetInstanceFromIStorage,
};

mod co_task_mem_alloc;
pub use co_task_mem_alloc::CoTaskMemAlloc;

mod co_task_mem_free;
pub use co_task_mem_free::CoTaskMemFree;

mod identifiers;
pub use identifiers::{
    CLSIDFromString, CoCreateGuid, CoFileTimeNow, CoGetCurrentProcess, IIDFromString, IsEqualGUID,
    ProgIDFromCLSID, SHCLSIDFromString, StringFromCLSID, StringFromGUID2, StringFromIID,
};
