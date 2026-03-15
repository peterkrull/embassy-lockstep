# Embassy Lockstep
> Not affiliated with the [Embassy](https://github.com/embassy-rs/embassy) project

This repository demonstrates a way to do whole-program testing of Embassy programs in a deterministic and nearly instant manner. This is accomplished by implementing a mock [time driver](https://github.com/embassy-rs/embassy/tree/main/embassy-time-driver) which does not depend on wall-time. Instead, the time driver in conjunction with careful executor polling, is able to run all pending work to completion, before jumping forward in time to when the next timer will fire.

## The world

However, most embedded devices are not just little time-keeping machines. They also interact with the real world, and the program logic we want to test is likely to depend on the interaction with the outside world. Therefore, we also support running a "world" alongside the firmware, which is expected to indicate how large time steps it is able to make.

This "world" could for example be various mock devices such as sensors and motors, which interact with a physics simulation.

## Structure

Due to how most Embassy programs are structured, there is bound to be a bunch of "global" state. This includes stuff like Embassy tasks or statically allocated synchronization primitives. It is therefore important that each and every test which relies on this global state runs in its on process. Otherwise the state of the statics may be tainted by other tests, even when running on a single thread.

To make tests run in their own process, they should be put in the `/tests` folder, and each test file should contain a single test. This ensures each test starts with a known-good state every time.

# Example

This is a minimal example, where the world simply defines a constant time step size.

```rust
use core::sync::atomic::{AtomicBool, Ordering};
use embassy_time::{Duration, Instant, Timer};
use embassy_lockstep::{lockstep_with_task, tasks::firmware_main};

pub static TEST_RUNNING: AtomicBool = AtomicBool::new(true);

#[test]
fn test() {
    lockstep_with_task(test_entry, || {
        assert!(
            Instant::now().as_secs() < 60,
            "Test did not complete in less than 60 seconds"
        );
        TEST_RUNNING
            .load(Ordering::Relaxed)
            .then_some(Duration::from_millis(10))
    });
}

// The "runtime" task for this test.
// It is what looks into the firmware
// to determine if the test case passes.
#[embassy_executor::task]
async fn test_entry(spawner: embassy_executor::Spawner) {
    spawner.must_spawn(firmware_main(spawner));
    Timer::after_secs(2).await;
    TEST_RUNNING.store(false, Ordering::Release);
}
```