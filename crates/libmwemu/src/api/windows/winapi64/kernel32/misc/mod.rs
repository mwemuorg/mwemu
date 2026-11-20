pub use crate::winapi::libraries::kernel32::entry_points::kernel32::{
    GetLastError, GetStartupInfoA, GetStartupInfoW, GetUserDefaultLCID, GlobalAddAtomA,
    GlobalAddAtomW, OutputDebugStringA, OutputDebugStringW, SetLastError, Sleep,
};

mod query_performance_counter;
pub use query_performance_counter::QueryPerformanceCounter;

mod win_exec;
pub use win_exec::WinExec;
