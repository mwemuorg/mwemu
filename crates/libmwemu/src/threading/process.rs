//! Unix process model for `fork()`.
//!
//! mwemu emulates a single address space, so a forked child does not run
//! concurrently with its parent. Instead `fork_begin` snapshots the parent and
//! execution continues as the child; when the child exits, `process_exit`
//! records its wait status and restores the parent, which then sees `fork()`
//! return the child's pid. An i386 child in nanosleep yields to its parent;
//! it can resume at a syscall boundary or when its parent waits for it.
//!
//! Limitation: a child that blocks waiting on its parent (e.g. reading a pipe
//! the parent writes after `fork()` returns) cannot make progress.

use std::{mem, sync::Arc, time::Duration};

use crate::arch::Arch;
use crate::emu::Emu;
use crate::kernel::heap::KernelHeap;
use crate::maps::Maps;
use crate::maps::heap_allocation::O1Heap;
use crate::syscall::linux::constants::errors::ENOEXEC;
use crate::syscall::linux::files::LinuxFileContext;
use crate::threading::context::ThreadContext;
use rs_header::elf::{elf32::Elf32, elf64::Elf64};

pub const INITIAL_PID: u64 = 1234;
const INITIAL_PPID: u64 = 1;

/// Parent state that a child may clobber, saved at `fork()` time.
struct ForkFrame {
    arch: Arch,
    ld_bootstrap: bool,
    elf64: Option<Arc<Elf64>>,
    pid: u64,
    ppid: u64,
    maps: Maps,
    linux_files: Option<LinuxFileContext>,
    elf32: Option<Arc<Elf32>>,
    heap_addr: u64,
    threads: Vec<ThreadContext>,
    current_thread_id: usize,
    #[allow(clippy::vec_box)] // same type as Emu::heap_arenas
    heap_arenas: Vec<Box<O1Heap>>,
    guard_heap: Option<KernelHeap>, // --memory-guard allocation ledger
    emulated_stdout: Vec<u8>,
    getopt_char_index: usize,
    atfork: Vec<AtFork>,
}

/// Handlers registered with `pthread_atfork` (0 = none).
#[derive(Clone, Copy)]
pub struct AtFork {
    pub prepare: u64,
    pub parent: u64,
    pub child: u64,
}

/// A child that has exited but has not been reaped by `wait*()` yet.
struct Zombie {
    pid: u64,
    ppid: u64,
    status: u64,
}

struct SleepingProcess {
    deadline: Duration,
    context: ForkFrame,
}

pub struct ProcessTable {
    pub pid: u64,
    pub ppid: u64,
    pub exit_status: Option<u64>, // wait status of the root process once it has ended
    pub atfork: Vec<AtFork>,      // pthread_atfork handlers, in registration order
    next_pid: u64,
    frames: Vec<ForkFrame>,
    zombies: Vec<Zombie>,
    sleeping: Vec<SleepingProcess>,
    pub(crate) clock_offset: Duration,
}

impl Default for ProcessTable {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessTable {
    pub fn new() -> Self {
        ProcessTable {
            pid: INITIAL_PID,
            ppid: INITIAL_PPID,
            exit_status: None,
            atfork: Vec::new(),
            next_pid: INITIAL_PID + 1,
            frames: Vec::new(),
            zombies: Vec::new(),
            sleeping: Vec::new(),
            clock_offset: Duration::ZERO,
        }
    }

    pub(crate) fn next_child_pid(&self) -> u64 {
        self.next_pid
    }

    pub(crate) fn sleeping_child(&self, pid: i32) -> Option<usize> {
        self.sleeping
            .iter()
            .enumerate()
            .filter(|(_, task)| {
                task.context.ppid == self.pid && (pid == -1 || task.context.pid == pid as u64)
            })
            .min_by_key(|(_, task)| task.deadline)
            .map(|(index, _)| index)
    }

    pub(crate) fn can_fork_within(&self, bytes: usize) -> bool {
        const MAX_CHILDREN: usize = 64;
        self.frames.len() + self.sleeping.len() + self.zombies.len() < MAX_CHILDREN
            && self
                .frames
                .iter()
                .chain(self.sleeping.iter().map(|task| &task.context))
                .map(|frame| frame.maps.size())
                .sum::<usize>()
                <= bytes
    }

    /// A vfork child runs on the thread scheduler but is reaped as a process.
    pub(crate) fn record_completed_child(&mut self, pid: u64, status: u64) {
        if !self
            .zombies
            .iter()
            .any(|child| child.ppid == self.pid && child.pid == pid)
        {
            self.zombies.push(Zombie {
                pid,
                ppid: self.pid,
                status,
            });
        }
    }

    /// True while the emulator is running a forked child.
    pub fn in_child(&self) -> bool {
        !self.frames.is_empty()
    }

    /// True if the current process has any exited, unreaped child.
    pub fn has_children(&self) -> bool {
        self.zombies.iter().any(|z| z.ppid == self.pid)
    }
}

/// Wait status of a normal exit, as decoded by `WEXITSTATUS`.
pub fn exit_status(code: u64) -> u64 {
    (code & 0xff) << 8
}

/// Wait status of a signal death, as decoded by `WTERMSIG`.
pub fn signal_status(sig: u64) -> u64 {
    sig & 0x7f
}

impl Emu {
    /// Current Unix process and the wait status of the root process.
    pub fn process_table(&self) -> &ProcessTable {
        &self.processes
    }

    /// Snapshot the parent and switch to the child. Returns the child's pid.
    pub fn fork_begin(&mut self) -> u64 {
        let child_pid = self.processes.next_pid;
        self.processes.next_pid += 1;

        let frame = self.fork_frame();
        self.processes.frames.push(frame);

        // The child only carries the thread that called fork().
        let caller = self.threads[self.current_thread_id].clone();
        self.threads = vec![caller];
        self.current_thread_id = 0;

        self.processes.ppid = self.processes.pid;
        self.processes.pid = child_pid;
        child_pid
    }

    fn fork_frame(&mut self) -> ForkFrame {
        ForkFrame {
            arch: self.cfg.arch,
            ld_bootstrap: self.ld_bootstrap,
            elf64: self.elf64.clone(),
            pid: self.processes.pid,
            ppid: self.processes.ppid,
            maps: self.maps.clone(),
            linux_files: self.linux_files.as_mut().map(|files| files.begin_child()),
            elf32: self.elf32.clone(),
            heap_addr: self.heap_addr,
            threads: self.threads.clone(),
            current_thread_id: self.current_thread_id,
            // Allocator bookkeeping lives outside guest memory, so it is
            // copied alongside the maps: the child inherits the parent heap.
            heap_arenas: self.heap_arenas.clone(),
            guard_heap: self.kernel.as_ref().map(|k| k.heap.clone()),
            emulated_stdout: self.emulated_stdout.clone(),
            getopt_char_index: self.getopt_char_index,
            atfork: self.processes.atfork.clone(),
        }
    }

    /// On vfork, the old address space still belongs to the parent after exec.
    pub(crate) fn enter_linux_exec(
        &mut self,
        vfork_parent: Option<(usize, u64)>,
    ) -> Result<(), u64> {
        let parent = vfork_parent.map(|(parent_thread, tid)| {
            let mut frame = self.fork_frame();
            frame.threads[self.current_thread_id].suspended = true;
            frame.threads[parent_thread].blocked_on_cs = None;
            frame.threads[parent_thread].wake_tick = 0;
            frame.current_thread_id = parent_thread;
            (frame, tid)
        });
        let files = self.linux_files.as_mut().ok_or(ENOEXEC)?;
        if let Err(errno) = files.enter_exec() {
            if let Some((frame, _)) = parent
                && let Some(context) = frame.linux_files
            {
                files.drop_context(context);
            }
            return Err(errno);
        }
        if let Some((frame, tid)) = parent {
            self.processes.frames.push(frame);
            self.processes.ppid = self.processes.pid;
            self.processes.pid = tid;
        }
        Ok(())
    }

    // The filesystem namespace stays shared. Only the per-process descriptor
    // references, mappings and execution state travel with the address space.
    fn switch_process(&mut self, frame: &mut ForkFrame, keep_current: bool) {
        self.sync_linux_files();
        if let (Some(files), Some(saved)) = (&mut self.linux_files, frame.linux_files.take()) {
            if keep_current {
                frame.linux_files = Some(files.begin_child());
            }
            files.restore_parent(saved);
        }
        mem::swap(&mut self.processes.pid, &mut frame.pid);
        mem::swap(&mut self.processes.ppid, &mut frame.ppid);
        mem::swap(&mut self.cfg.arch, &mut frame.arch);
        mem::swap(&mut self.ld_bootstrap, &mut frame.ld_bootstrap);
        mem::swap(&mut self.elf64, &mut frame.elf64);
        mem::swap(&mut self.maps, &mut frame.maps);
        mem::swap(&mut self.elf32, &mut frame.elf32);
        mem::swap(&mut self.threads, &mut frame.threads);
        mem::swap(&mut self.current_thread_id, &mut frame.current_thread_id);
        mem::swap(&mut self.heap_addr, &mut frame.heap_addr);
        mem::swap(&mut self.heap_arenas, &mut frame.heap_arenas);
        if let (Some(kernel), Some(heap)) = (&mut self.kernel, &mut frame.guard_heap) {
            mem::swap(&mut kernel.heap, heap);
        }
        mem::swap(&mut self.emulated_stdout, &mut frame.emulated_stdout);
        mem::swap(&mut self.getopt_char_index, &mut frame.getopt_char_index);
        mem::swap(&mut self.processes.atfork, &mut frame.atfork);
        self.reset_active_instruction_cache();
    }

    pub(crate) fn sleep_linux_process(&mut self, duration: Duration) {
        if let Some(mut context) = self.processes.frames.pop() {
            let deadline = self.now.elapsed() + self.processes.clock_offset + duration;
            self.switch_process(&mut context, true);
            self.processes
                .sleeping
                .push(SleepingProcess { deadline, context });
            self.force_reload = true;
        } else {
            self.processes.clock_offset += duration;
        }
    }

    // A blocking wait can advance the guest clock when its child is asleep.
    // The interrupted caller re-enters its syscall with the original registers.
    pub(crate) fn resume_sleeping_process(&mut self, index: usize) {
        let SleepingProcess {
            deadline,
            mut context,
        } = self.processes.sleeping.swap_remove(index);
        let now = self.now.elapsed() + self.processes.clock_offset;
        self.processes.clock_offset += deadline.saturating_sub(now);
        self.switch_process(&mut context, true);
        self.processes.frames.push(context);
        self.force_reload = true;
    }

    pub(crate) fn resume_due_linux_process(&mut self) -> bool {
        if self.processes.sleeping.is_empty() {
            return false;
        }
        let now = self.now.elapsed() + self.processes.clock_offset;
        let Some(index) = self
            .processes
            .sleeping
            .iter()
            .position(|task| task.deadline <= now)
        else {
            return false;
        };
        self.resume_sleeping_process(index);
        true
    }

    /// End the current process. A suspended caller resumes with its saved
    /// registers. The root records its wait status and stops the emulator.
    pub fn process_exit(&mut self, status: u64) -> Option<u64> {
        let Some(mut caller) = self.processes.frames.pop() else {
            for task in self.processes.sleeping.drain(..) {
                if let (Some(files), Some(saved)) =
                    (&mut self.linux_files, task.context.linux_files)
                {
                    files.drop_context(saved);
                }
            }
            self.processes.exit_status = Some(status);
            self.stop();
            return None;
        };
        let pid = self.processes.pid;
        let ppid = self.processes.ppid;
        for task in &mut self.processes.sleeping {
            if task.context.ppid == pid {
                task.context.ppid = INITIAL_PPID;
            }
        }
        self.processes.zombies.retain(|child| child.ppid != pid);
        if ppid != INITIAL_PPID {
            self.processes.zombies.push(Zombie { pid, ppid, status });
        }
        self.switch_process(&mut caller, false);
        Some(pid)
    }

    /// Reap an exited child of the current process. `pid <= 0` matches any
    /// child. Returns `(pid, wait status)`.
    pub fn reap_child(&mut self, pid: i64) -> Option<(u64, u64)> {
        let me = self.processes.pid;
        let idx = self
            .processes
            .zombies
            .iter()
            .position(|z| z.ppid == me && (pid <= 0 || z.pid == pid as u64))?;
        let zombie = self.processes.zombies.remove(idx);
        Some((zombie.pid, zombie.status))
    }
}
