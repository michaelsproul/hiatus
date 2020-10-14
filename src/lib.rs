//! Hiatus is a concurrency debugging library for Rust. It allows you to sprinkle breakpoints
//! in your programs so that blocks of code execute in the order you choose. If you suspect that a
//! specific interleaving of blocks is buggy, you can use Hiatus to invoke that ordering and
//! confirm the existence of the bug.
//!
//! This library is **experimental**!
use lazy_static::lazy_static;
use parking_lot::{Condvar, Mutex, MutexGuard};
use std::sync::atomic::{AtomicBool, Ordering};

lazy_static! {
    static ref CURRENT_STEP: Mutex<u64> = Mutex::new(1);
    static ref CONDVAR: Condvar = Condvar::new();
    static ref ENABLED: AtomicBool = AtomicBool::new(false);
}

/// Breakpoint object returned by `step`, with drop semantics.
///
/// See the docs for [`step`](./fn.step.html) for usage.
#[must_use]
pub enum Step<'a> {
    /// Step variant used when Hiatus is enabled.
    Real {
        n: u64,
        current_step: MutexGuard<'a, u64>,
    },
    /// Step variant used when Hiatus is disabled.
    Dummy,
}

/// Enable Hiatus (it is disabled by default).
///
/// You should call `enable` when your program has initialised and you are ready for it
/// to start executing according to the breakpoints you inserted using [`step`](./fn.step.html).
pub fn enable() {
    ENABLED.store(true, Ordering::SeqCst)
}

/// Disable Hiatus, causing future `step` calls to do nothing.
pub fn disable() {
    ENABLED.store(false, Ordering::SeqCst)
}

/// Check whether Hiatus is currently enabled.
///
/// Hiatus is disabled by default and needs to be enabled by calling [`enable`](./fn.enable.html).
pub fn is_enabled() -> bool {
    ENABLED.load(Ordering::SeqCst)
}

/// Set a breakpoint in your program to control its execution.
///
/// Calling `step(n)` will block the program until all previous `step` calls have resolved.
///
/// The first step of your program should be `step(1)`.
///
/// ## How it works
///
/// Hiatus maintains a global step counter, which is incremented each time a `Step` object
/// returned by `step` is dropped. When `step(n)` is called, it blocks the current thread
/// until the global step count is equal to `n`, at which point that thread is unblocked
/// and allowed to execute. To signal to other threads that the current step is complete,
/// you should drop the `Step` object. You _can_ do this immediately if you want, or you can
/// wait until some block of code has finished executing.
/// See [`Step::then`](./enum.Step.html#method.then).
///
/// ## Warnings
///
/// Make sure you enable Hiatus by calling [`enable`](./fn.enable.html) first!
///
/// It's probably not a good idea to have multiple calls for the same step count in the
/// same program. It likely won't do anything sensible (I haven't tried it).
pub fn step<'a>(n: u64) -> Step<'a> {
    assert_ne!(n, 0, "steps start from 1");
    if is_enabled() {
        real_step(n)
    } else {
        Step::Dummy
    }
}

fn real_step<'a>(n: u64) -> Step<'a> {
    // Use the condition variable to wait for the step count to reach `n`.
    let mut current_step = CURRENT_STEP.lock();
    while *current_step != n {
        CONDVAR.wait(&mut current_step);
    }
    // Step count has reached `n`, and we hold the mutex.
    // Return the value and let the caller execute their critical section.
    // When they're done, they should drop the `Step` to indicate that the next step is
    // allowed to run.
    Step::Real { n, current_step }
}

impl<'a> Step<'a> {
    /// Shorthand for dropping this step and moving to a new step `n`.
    pub fn then(self, n: u64) -> Step<'a> {
        drop(self);
        step(n)
    }
}

impl<'a> Drop for Step<'a> {
    /// Increment the global step count, and signal the condition variable to wake up waiters.
    fn drop(&mut self) {
        if let Step::Real { current_step, .. } = self {
            // Increment the step count.
            **current_step += 1;
            // Signal all the other waiters (a little inefficient -- but the alternative is one
            // condition variable per step, which seems unwieldy).
            CONDVAR.notify_all();
        }
    }
}
