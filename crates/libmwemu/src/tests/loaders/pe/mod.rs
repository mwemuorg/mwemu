mod exe32win_minecraft;
// Release-only: 5M instructions in a debug build take ~25s and are the single
// biggest serial chunk of the suite, which tips slow CI runners (macOS/Windows)
// over the job timeout. Emulation is ~15-30x faster in release, so we only build
// and run it there; the debug pass skips it.
#[cfg(not(debug_assertions))]
mod exe64win_enigma;
mod exe64win_msgbox;
mod mingw_tests;
mod pe64_loader_tests;
mod peb_teb_ldr_structures_test;
mod tls_fls_tests;
