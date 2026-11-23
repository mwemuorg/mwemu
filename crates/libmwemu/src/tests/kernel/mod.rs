mod api_calls;
mod driver_capture_tests;
#[cfg(feature = "asset-tests")]
mod linux_uaf_tests;
#[cfg(feature = "host-tests")]
mod macos_uaf_tests;
mod register_model_tests;
mod windows_driver_tests;
