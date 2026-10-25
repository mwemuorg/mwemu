# Multithreading support

Status: partially implemented. The infrastructure (scheduler, context switch,
per-thread registers) exists. What's missing is the synchronisation layer and
a handful of fields that need to move from `Emu` to `ThreadContext`.

## Current architecture

```
Emu (single instance)
├── maps: Maps                      shared virtual memory
├── threads: Vec<ThreadContext>     per-thread register state
│   ├── [0] { id, regs, flags, fpu, seh, tls, fls, call_stack, ... }
│   ├── [1] { id, regs, flags, fpu, seh, tls, fls, call_stack, ... }
│   └── ...
├── current_thread_id: usize        index of the running thread
├── global_locks: GlobalLocks       critical-section ownership table
└── tick: usize                     global clock for sleep/wake
```

`emu.regs()`, `emu.flags()`, `emu.fpu()` etc. read from
`threads[current_thread_id]`. Context switch only changes the index — no copy.

The scheduler (`threading/scheduler.rs`) implements instruction-level
round-robin with `is_runnable()` checks (suspended, wake_tick, blocked_on_cs,
exit_value, joining, blocked_on_lock, cond_wait).

## What already works

- `CreateThread` (32-bit and 64-bit) creates a real `ThreadContext` with its
  own registers and stack.
- `step_isa()` dispatches to `step_multi_threaded()` when
  `cfg.enable_threading && threads.len() > 1`.
- Thread states: runnable, suspended, sleeping (wake_tick), blocked on
  critical section, exited.
- Deadlock detection and time-advance when all threads sleep.
- TID prefix in `log_red!` / `log_orange!` output: `** {tid}> {pos}:{rip}`.

## Phase 1 — connect the wires (~70 lines)

These are the minimum changes to make multi-threading actually run.

### 1.1 Enable threading by default

Add `--threading` CLI flag or set `cfg.enable_threading = true` when
`CreateThread` is called for the first time. Currently the thread is
created but the scheduler ignores it unless `enable_threading` is on.

### 1.2 Default stack when stack_sz == 0

Go (and some C runtimes) pass `stack_sz=0` to `CreateThread`, meaning
"use default". Allocate 1 MB when `stack_sz == 0`:

```rust
let stack_sz = if stack_sz == 0 { 0x100000 } else { stack_sz };
```

### 1.3 ExitThread vs ExitProcess

`ExitThread` should mark `exit_value = Some(code)` on the current thread
and let the scheduler switch to another. Only `ExitProcess` stops the
whole emulator. Currently both call `emu.stop()`.

### 1.4 WaitForSingleObject — basic thread wait

Minimal implementation: if the handle is a thread handle and that thread
has `exit_value.is_some()`, return `WAIT_OBJECT_0` immediately. Otherwise
set `suspended = true` on the calling thread and record what it's waiting
for, then let the scheduler switch.

### 1.5 Verify TID in logs

Already done — `log_red!` and `log_orange!` print `** {tid:x}> ...`.

## Phase 2 — synchronisation primitives (~150 lines)

This is the hard part. Each primitive needs a state object tracked by
handle or address, and the scheduler must wake blocked threads when
another thread signals the object.

### 2.1 Events (CreateEvent / SetEvent / ResetEvent / WaitForSingleObject)

State per event:
```rust
struct EmulatedEvent {
    signaled: bool,
    manual_reset: bool,
    waiters: Vec<usize>,   // thread indices waiting on this event
}
```

- `CreateEvent` → allocate handle, store `EmulatedEvent`.
- `SetEvent` → set `signaled = true`, wake all waiters (manual) or
  first waiter (auto-reset, then `signaled = false`).
- `ResetEvent` → `signaled = false`.
- `WaitForSingleObject(event)` → if signaled, return (auto-reset:
  clear flag). If not, add to waiters + suspend thread.

Go uses events heavily for M parking (`runtime.semasleep` /
`runtime.semawakeup`).

### 2.2 Critical sections (EnterCriticalSection / LeaveCriticalSection)

`GlobalLocks` already exists. Needs:
- Owner thread tracking (allow recursion by same thread).
- `blocked_on_cs` on the waiter (already in ThreadContext).
- `LeaveCriticalSection` must check if any thread is blocked on this CS
  and wake it.

### 2.3 Mutexes (CreateMutex / ReleaseMutex)

Similar to critical sections but handle-based. Lower priority — most
Win32 malware uses CriticalSections for in-process sync.

### 2.4 Semaphores (CreateSemaphore / ReleaseSemaphore)

Count-based. `WaitForSingleObject` decrements, `ReleaseSemaphore`
increments and wakes waiters. Lower priority.

### 2.5 WaitForMultipleObjects

Needs to check all handles and wake when any (WaitAny) or all (WaitAll)
are signaled. Higher complexity. Can defer to just WaitAny first.

## Phase 3 — move per-thread state out of Emu

These fields in `Emu` are logically per-thread and will cause bugs when
threads interleave. Move them into `ThreadContext`:

### Critical (will cause wrong behavior):

| Field | Why per-thread |
|-------|----------------|
| `last_error: u64` | `GetLastError`/`SetLastError` is per-thread in Windows |
| `gateway_return: u64` | Return addr from API trampoline — belongs to calling thread |
| `its_apicall: Option<u64>` | API being dispatched by this thread |
| `skip_apicall: bool` | Skip flag for current thread's API |
| `is_api_run: bool` | Inside API handler flag for current thread |
| `call_depth: u32` | Call nesting depth — each thread has its own |

### Important (mid-instruction state):

| Field | Why per-thread |
|-------|----------------|
| `rep: Option<u64>` | REP prefix counter — context switch during rep movsb loses state |
| `force_reload: bool` | Decode state of current thread |
| `run_until_ret: bool` | Debugger step-over targets one thread |

### OK as global:

| Field | Why global is fine |
|-------|-------------------|
| `count_read/write/send/recv` | Process-wide counters in real Windows |
| `memory_operations` | Trace log — TID in output already distinguishes |
| `last_decoded` / `instruction_state` | Cache invalidation on switch is enough |
| `handle_management` | Handle table is process-wide |
| `heap_arenas` | Heap is process-wide (thread safety is via CriticalSection) |

### Migration approach

Add fields to `ThreadContext`, add accessor methods that read from
`threads[current_thread_id]`, update callers. The `last_error` one is
the most impactful — many API stubs call `SetLastError`.

## Go runtime specifics

The Go runtime (`runtime.newm` → `newosproc` → `CreateThread`) needs:

1. `CreateThread` with real scheduling (Phase 1).
2. Events for M park/unpark: `runtime.semasleep` calls
   `WaitForSingleObject(event)`, `runtime.semawakeup` calls
   `SetEvent(event)` (Phase 2.1).
3. Interlocked operations (`InterlockedCompareExchange`,
   `InterlockedIncrement`) — already atomic for free in single-core
   emulation since Maps is shared and execution is sequential.

The Vidar crash (`runtime.gcenable` panic at 624k instructions) happens
because Go creates a GC thread that never runs. With Phase 1 complete,
the GC thread would execute and Go would get past initialization.

## Risk: debugging deadlocks

The real cost of Phase 2 is not the ~150 lines of code but the debugging
when synchronisation semantics are slightly wrong:

- Auto-reset event that doesn't reset → spurious wakeup.
- Manual-reset event that resets too early → missed wakeup.
- Critical section with wrong owner tracking → deadlock.
- WaitForSingleObject timeout not advancing tick → thread starves.

Each of these manifests as a silent hang, not a crash. Recommend adding
a deadlock watchdog: if no thread makes progress for N ticks, dump all
thread states and break.

## Execution order

```
Phase 1 (connect the wires)     ~70 lines, low risk
    ↓
Phase 3 (move per-thread state) ~100 lines, mechanical
    ↓
Phase 2 (sync primitives)       ~150 lines, high debug cost
```

Phase 1 and 3 are independent. Phase 2 depends on Phase 1.
Phase 3 can happen before or after Phase 2 but doing it before avoids
having to debug per-thread state bugs on top of synchronisation bugs.
