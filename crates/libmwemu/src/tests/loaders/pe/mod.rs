#[cfg(feature = "asset-tests")]
mod exe32win_minecraft;
#[cfg(feature = "asset-tests")]
mod exe64win_msgbox;
mod hello_world;
#[cfg(feature = "asset-tests")]
mod mingw_tests;
mod module_loading_tests;
#[cfg(feature = "asset-tests")]
mod packed;
#[cfg(feature = "asset-tests")]
mod pe64_loader_tests;
mod peb_teb_ldr_structures_test;
#[cfg(feature = "asset-tests")]
mod ssdt;
#[cfg(feature = "asset-tests")]
mod tls_callbacks_tests;
mod tls_fls_tests;
