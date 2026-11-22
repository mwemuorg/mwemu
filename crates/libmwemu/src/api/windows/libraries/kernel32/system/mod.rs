mod get_calendar_info;
pub use get_calendar_info::{GetCalendarInfoA, GetCalendarInfoW};

pub use crate::winapi::libraries::kernel32::entry_points::kernel32::GetUserDefaultLangID;

mod code_pages;
pub use code_pages::{GetACP, GetOEMCP};

mod processor_groups;
pub use processor_groups::{GetActiveProcessorGroupCount, GetMaximumProcessorGroupCount};

mod thread_ui_language;
pub use thread_ui_language::{GetThreadUILanguage, SetThreadUILanguage};

mod multimedia_timer;
pub(crate) use multimedia_timer::dispatch_multimedia_timer;
pub use multimedia_timer::{timeGetDevCaps, timeGetTime};
