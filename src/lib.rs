use core::task::Waker;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{LazyLock, Mutex};

use embassy_executor::{SpawnToken, Spawner, raw};
use embassy_time::Duration;
use embassy_time_driver::Driver;
use embassy_time_queue_utils::Queue;

const CLAIM_ERROR_MSG: &'static str = "
------------------------------------
Attempt to claim LockstepExecutor twice!
Tests should only run on a single thread
>> cargo test -- --test-threads=1 <<
------------------------------------
\n";

pub struct LockstepExecutor {
    executor: LazyLock<raw::Executor>,
    claimed: AtomicBool,
}

impl LockstepExecutor {
    pub const fn new() -> Self {
        LockstepExecutor {
            executor: LazyLock::new(|| raw::Executor::new(null_mut())),
            claimed: AtomicBool::new(false),
        }
    }

    /// Run the lockstep executor with an entry function and a world simulation.
    pub unsafe fn with<S>(
        &'static self,
        entry: impl Fn(Spawner) -> SpawnToken<S>,
        mut world: impl FnMut() -> Option<Duration>,
    ) {
        if self.claimed.swap(true, Ordering::AcqRel) {
            panic!("{}", CLAIM_ERROR_MSG)
        }
        
        // Ensure the spawner is not tainted from a prior run
        assert_eq!(
            as_slice(&*self.executor),
            as_slice(&raw::Executor::new(null_mut())),
            "The executor was left in a dirty state"
        );
        
        // Reset time driver and spawn entry task
        DRIVER.reset();
        let spawner = self.executor.spawner();
        spawner.spawn(entry(spawner)).unwrap();

        while let Some(dt) = world() {
            unsafe { DRIVER.advance(dt.as_ticks(), &self.executor) };
        }

        self.claimed.store(false, Ordering::Release)
    }
}

fn as_slice<T>(data: &T) -> &[u8] {
    unsafe {
        let len = core::mem::size_of_val(data);
        let ptr = core::ptr::addr_of!(*data);
        core::slice::from_raw_parts(ptr as *const u8, len)
    }
}

unsafe impl Send for LockstepExecutor {}
unsafe impl Sync for LockstepExecutor {}

static PENDING: AtomicBool = AtomicBool::new(true);

#[unsafe(no_mangle)]
fn __pender(_context: *mut ()) {
    PENDING.store(true, Ordering::Release);
}
pub struct LockstepDriver {
    ticks: AtomicU64,
    queue: Mutex<Queue>,
}

impl LockstepDriver {
    const fn new() -> Self {
        Self {
            ticks: AtomicU64::new(0),
            queue: Mutex::new(Queue::new()),
        }
    }

    fn reset(&self) {
        PENDING.store(true, Ordering::Release);
        self.ticks.store(0, Ordering::Release);
        *self.queue.lock().unwrap() = Queue::new();
    }

    fn next_expiration(&self) -> u64 {
        self.queue.lock().unwrap().next_expiration(self.now())
    }

    /// Advances the simulation clock by a specific number of ticks,
    /// while completing all pending work within the executor.
    unsafe fn advance(&self, delta_ticks: u64, executor: &'static raw::Executor) {
        let target = self.now() + delta_ticks;

        loop {
            // Deque expired timers and poll tasks to completion
            self.next_expiration();
            unsafe { executor.poll() };

            // Keep polling while work is being pended
            while PENDING.swap(false, Ordering::AcqRel) {
                continue;
            }

            // Bump the tick count up to the next alarm (or target)
            let alarm = self.next_expiration();
            self.ticks.store(alarm.min(target), Ordering::Release);

            // We are done if the alarm exceeds target
            if alarm >= target {
                break;
            }
        }
    }
}

impl Driver for LockstepDriver {
    fn now(&self) -> u64 {
        self.ticks.load(Ordering::Acquire)
    }

    fn schedule_wake(&self, at: u64, waker: &Waker) {
        self.queue.lock().unwrap().schedule_wake(at, waker);
    }
}

embassy_time_driver::time_driver_impl!(static DRIVER: LockstepDriver = LockstepDriver::new());
