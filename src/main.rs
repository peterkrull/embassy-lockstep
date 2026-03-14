use std::sync::{
    atomic::{AtomicBool, Ordering},
};

use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer};

#[cfg(test)]
mod test {
    use std::sync::atomic::Ordering;

    use embassy_lockstep::LockstepExecutor;
    use embassy_time::{Duration, Instant};

    use crate::TEST_COMPLETE;

    static EXECUTOR: LockstepExecutor = LockstepExecutor::new();

    #[test]
    fn system_integration_test_0() {
        TEST_COMPLETE.store(false, Ordering::Relaxed);

        unsafe {
            EXECUTOR.with(super::firmware_main, || {
                if Instant::now().as_secs() < 3 || !TEST_COMPLETE.load(Ordering::Relaxed) {
                    Some(Duration::from_millis(10))
                } else {
                    None
                }
            })
        };

        if !TEST_COMPLETE.load(Ordering::Relaxed) {
            panic!("Test did not complete in the required amount of time")
        }
    }

    #[test]
    fn system_integration_test_1() {
        TEST_COMPLETE.store(false, Ordering::Relaxed);

        unsafe {
            EXECUTOR.with(super::firmware_main, || {
                if Instant::now().as_secs() < 3 || !TEST_COMPLETE.load(Ordering::Relaxed) {
                    Some(Duration::from_millis(10))
                } else {
                    None
                }
            })
        };

        if !TEST_COMPLETE.load(Ordering::Relaxed) {
            panic!("Test did not complete in the required amount of time")
        }
    }

    #[test]
    fn system_integration_test_2() {
        TEST_COMPLETE.store(false, Ordering::Relaxed);

        unsafe {
            EXECUTOR.with(super::firmware_main, || {
                if Instant::now().as_secs() < 3 || !TEST_COMPLETE.load(Ordering::Relaxed) {
                    Some(Duration::from_millis(10))
                } else {
                    None
                }
            })
        };

        if !TEST_COMPLETE.load(Ordering::Relaxed) {
            panic!("Test did not complete in the required amount of time")
        }
    }
}

fn main() {}

// This is where our firmware starts,
// and all other tasks are launched.
#[embassy_executor::task]
async fn firmware_main(spawner: Spawner) {
    println!("Hello from main");

    spawner.must_spawn(task0());
    spawner.must_spawn(task1());
    spawner.must_spawn(task2());

    let start_time = Instant::now();
    while start_time.elapsed() < Duration::from_secs(2) {
        let seconds = Instant::now().as_millis() as f32 / 1e3;

        println!("[  ] At time: {seconds} secs ({:?})", Instant::now());
        Timer::after_millis(200).await;
    }
}

// This is where our firmware starts,
// and all other tasks are launched.
#[embassy_executor::task]
async fn task0() {
    println!("Hello from task0 (ticks: {})", Instant::now().as_ticks());

    Timer::after_millis(100).await;

    let start_time = Instant::now();
    while start_time.elapsed() < Duration::from_secs(2) {
        let seconds = Instant::now().as_millis() as f32 / 1e3;

        println!("[t0] -- At time: {seconds} secs ({:?})", Instant::now());
        Timer::after_millis(200).await;
    }
    
    loop {
        Timer::after_millis(200).await;
    }
}

// This is where our firmware starts,
// and all other tasks are launched.
#[embassy_executor::task]
async fn task1() {
    println!("Hello from task1 (ticks: {})", Instant::now().as_ticks());

    Timer::after_millis(110).await;

    let start_time = Instant::now();
    while start_time.elapsed() < Duration::from_secs(2) {
        let seconds = Instant::now().as_millis() as f32 / 1e3;

        println!("[t1] -- At time: {seconds} secs ({:?})", Instant::now());
        Timer::after_millis(200).await;
    }
}

// This is where our firmware starts,
// and all other tasks are launched.
#[embassy_executor::task]
async fn task2() {
    println!("Hello from task2 (ticks: {})", Instant::now().as_ticks());

    Timer::after_millis(120).await;

    let start_time = Instant::now();
    while start_time.elapsed() < Duration::from_secs(2) {
        let seconds = Instant::now().as_millis() as f32 / 1e3;

        println!("[t2] -- At time: {seconds} secs ({:?})", Instant::now());
        Timer::after_millis(200).await;
    }

    TEST_COMPLETE.store(true, Ordering::Relaxed);

}

pub static TEST_COMPLETE: AtomicBool = AtomicBool::new(false);
