mod lstrcmp;
pub(crate) mod strsafe;
pub use lstrcmp::{lstrcmpA, lstrcmpW};

mod lstrcmpi;
pub use lstrcmpi::{lstrcmpi, lstrcmpiA, lstrcmpiW};

mod lstrcpy;
pub use lstrcpy::{lstrcpy, lstrcpyA, lstrcpyW};

mod lstrcpyn;
pub use lstrcpyn::{lstrcpyn, lstrcpynA, lstrcpynW};
