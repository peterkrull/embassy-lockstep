use core::sync::atomic::{AtomicBool, Ordering};

use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer};

use embassy_lockstep::{lockstep_with, tasks::firmware_main};

pub static TEST_RUNNING: AtomicBool = AtomicBool::new(true);

#[test]
fn test() {
    lockstep_with(test_entry, || {
        assert!(Instant::now().as_secs() < 60);
        TEST_RUNNING
            .load(Ordering::Relaxed)
            .then_some(Duration::from_millis(10))
    });
}

// The "runtime" task for this test.
// It is what looks into the firmware
// to determine if the test case passes.
#[embassy_executor::task]
async fn test_entry(spawner: Spawner) {
    spawner.must_spawn(firmware_main(spawner));
    Timer::after_secs(2).await;
    TEST_RUNNING.store(false, Ordering::Release);
}
