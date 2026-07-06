use crate::emu;
use crate::winapi::helper;

pub fn ResumeThread(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;

    log_red!(emu, "kernel32!ResumeThread hndl: 0x{:x}", hndl);

    // Get the URI from the handle to extract thread ID
    let uri = helper::handler_get_uri(hndl);
    if !uri.starts_with("tid://") {
        log::error!("ResumeThread: Invalid thread handle 0x{:x}", hndl);
        emu.regs_mut().rax = 0xFFFFFFFF; // -1 for error
        return;
    }

    // Extract thread ID from URI (format: "tid://0x1000")
    let tid_str = &uri[6..]; // Skip "tid://"
    let thread_id = if tid_str.starts_with("0x") {
        u64::from_str_radix(&tid_str[2..], 16).unwrap_or(0)
    } else {
        tid_str.parse::<u64>().unwrap_or(0)
    };

    if thread_id == 0 {
        log::error!("ResumeThread: Failed to parse thread ID from URI: {}", uri);
        emu.regs_mut().rax = 0xFFFFFFFF; // -1 for error
        return;
    }

    // Find the thread in the threads vector
    let mut previous_suspend_count = 0;
    let mut thread_found = false;

    for thread in &mut emu.threads {
        if thread.id == thread_id {
            thread_found = true;
            // Track previous suspend count (Windows tracks multiple suspends)
            // For now we use a simple boolean, so count is either 0 or 1
            previous_suspend_count = if thread.suspended { 1 } else { 0 };

            // Resume the thread
            if thread.suspended {
                thread.suspended = false;
                log::trace!("Thread 0x{:x} resumed (was suspended)", thread_id);
            } else {
                log::trace!("Thread 0x{:x} was already running", thread_id);
            }
            break;
        }
    }

    if !thread_found {
        log::error!("ResumeThread: Thread 0x{:x} not found", thread_id);
        emu.regs_mut().rax = 0xFFFFFFFF; // -1 for error
        return;
    }

    // Return the previous suspend count
    emu.regs_mut().rax = previous_suspend_count;
    log::trace!(
        "ResumeThread returning previous suspend count: {}",
        previous_suspend_count
    );
}
