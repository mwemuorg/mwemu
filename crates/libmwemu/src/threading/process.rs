//! Unix process model for `fork()`.
//!
//! mwemu emulates a single address space, so a forked child does not run
//! concurrently with its parent. Instead `fork_begin` snapshots the parent and
//! execution continues as the child; when the child exits, `process_exit`
//! records its wait status and restores the parent, which then sees `fork()`
//! return the child's pid. Nested forks stack naturally.
//!
//! Limitation: a child that blocks waiting on its parent (e.g. reading a pipe
//! the parent writes after `fork()` returns) cannot make progress.

use crate::emu::Emu;
use crate::kernel::heap::KernelHeap;
use crate::maps::Maps;
use crate::maps::heap_allocation::O1Heap;
use crate::threading::context::ThreadContext;

pub const INITIAL_PID: u64 = 1234;
const INITIAL_PPID: u64 = 1;

/// Parent state that a child may clobber, saved at `fork()` time.
struct ForkFrame {
    pid: u64,
    ppid: u64,
    maps: Maps,
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

pub struct ProcessTable {
    pub pid: u64,
    pub ppid: u64,
    pub exit_status: Option<u64>, // wait status of the root process once it has ended
    pub atfork: Vec<AtFork>,      // pthread_atfork handlers, in registration order
    next_pid: u64,
    frames: Vec<ForkFrame>,
    zombies: Vec<Zombie>,
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
    /// Snapshot the parent and switch to the child. Returns the child's pid.
    pub fn fork_begin(&mut self) -> u64 {
        let child_pid = self.processes.next_pid;
        self.processes.next_pid += 1;

        let frame = ForkFrame {
            pid: self.processes.pid,
            ppid: self.processes.ppid,
            maps: self.maps.clone(),
            threads: self.threads.clone(),
            current_thread_id: self.current_thread_id,
            // Allocator bookkeeping lives outside guest memory, so it is
            // copied alongside the maps: the child inherits the parent heap.
            heap_arenas: self.heap_arenas.clone(),
            guard_heap: self.kernel.as_ref().map(|k| k.heap.clone()),
            emulated_stdout: self.emulated_stdout.clone(),
            getopt_char_index: self.getopt_char_index,
            atfork: self.processes.atfork.clone(),
        };
        self.processes.frames.push(frame);

        // The child only carries the thread that called fork().
        let caller = self.threads[self.current_thread_id].clone();
        self.threads = vec![caller];
        self.current_thread_id = 0;

        self.processes.ppid = self.processes.pid;
        self.processes.pid = child_pid;
        child_pid
    }

    /// End the current process with wait `status`. A forked child resumes its
    /// parent and returns the child's pid; the root process records the status,
    /// stops the emulator and returns `None`.
    pub fn process_exit(&mut self, status: u64) -> Option<u64> {
        if !self.processes.in_child() {
            self.processes.exit_status = Some(status);
            self.stop();
            return None;
        }
        let frame = self.processes.frames.pop()?;
        let child_pid = self.processes.pid;
        self.processes.zombies.push(Zombie {
            pid: child_pid,
            ppid: frame.pid,
            status,
        });

        self.processes.pid = frame.pid;
        self.processes.ppid = frame.ppid;
        self.maps = frame.maps;
        self.threads = frame.threads;
        self.current_thread_id = frame.current_thread_id;
        self.heap_arenas = frame.heap_arenas;
        // Only the ledger rolls back; findings raised in the child are kept.
        if let (Some(kernel), Some(heap)) = (self.kernel.as_mut(), frame.guard_heap) {
            kernel.heap = heap;
        }
        self.emulated_stdout = frame.emulated_stdout;
        self.getopt_char_index = frame.getopt_char_index;
        self.processes.atfork = frame.atfork;
        self.reset_active_instruction_cache();
        Some(child_pid)
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
