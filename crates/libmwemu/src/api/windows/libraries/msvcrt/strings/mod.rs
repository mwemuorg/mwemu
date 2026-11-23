// Keep the public msvcrt Rust entry points on the same path as guest dispatch.
macro_rules! msvcrt_exports {
    ($($name:ident),+ $(,)?) => {
        $(
            pub fn $name(emu: &mut Emu) {
                dispatch(stringify!($name), emu, Runtime::Msvcrt);
            }
        )+
    };
}

mod bounded;
pub(crate) use bounded::dispatch_bounded;

mod floating;
pub(crate) use floating::dispatch_float;

mod character;
pub(crate) use character::dispatch_character;

mod str;
pub use str::str;

mod wide_search;
pub(crate) use wide_search::dispatch_wide_search;

mod text;
pub(crate) use text::dispatch_text;

pub(super) mod units;
pub(crate) use units::{dispatch as dispatch_units, string_operation};

mod integer;
pub(crate) use integer::dispatch as dispatch_integer;

mod tokens;
pub(crate) use tokens::dispatch as dispatch_tokens;

pub use units::{
    _strcmpi, _stricmp, _strnicmp, _wcsicmp, _wcsnicmp, strcat, strchr, strcmp, strcpy, strlen,
    strncat, strncmp, strncpy, strrchr, strstr, wcscat, wcscmp, wcscpy, wcslen, wcsncat, wcsncpy,
};

pub use integer::{_itoa, _wtoi, _wtol, atoi, atol, strtol, strtoul};

pub use tokens::{strtok, strtok_s};
