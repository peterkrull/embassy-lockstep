//! Example tasks used by the test suite.

use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer};

// This is where our firmware starts,
// and all other tasks are launched.
#[embassy_executor::task]
pub async fn firmware_main(spawner: Spawner) {
    let initial_tick = Instant::now().as_ticks();
    println!("Hello from main (ticks: {initial_tick})");

    spawner.must_spawn(task0(initial_tick));
    spawner.must_spawn(task1(initial_tick));
    spawner.must_spawn(task2(initial_tick));

    let start_time = Instant::now();
    while start_time.elapsed() < Duration::from_secs(2) {
        let seconds = Instant::now().as_millis() as f32 / 1e3;

        println!("[  ] At time: {seconds} secs ({:?})", Instant::now());
        Timer::after_millis(200).await;
    }
}

// Some task doing some time-dependent stuff
#[embassy_executor::task]
pub async fn task0(intial_tick: u64) {
    let ticks = Instant::now().as_ticks();
    println!("Hello from task0 (ticks: {ticks})");
    assert_eq!(ticks, intial_tick);

    Timer::after_millis(100).await;

    let start_time = Instant::now();
    while start_time.elapsed() < Duration::from_secs(4) {
        let seconds = Instant::now().as_millis() as f32 / 1e3;

        println!("[t0] -- At time: {seconds} secs ({:?})", Instant::now());
        Timer::after_millis(200).await;
    }

    loop {
        Timer::after_millis(200).await;
    }
}

// Some task doing some time-dependent stuff
#[embassy_executor::task]
pub async fn task1(intial_tick: u64) {
    let ticks = Instant::now().as_ticks();
    println!("Hello from task1 (ticks: {ticks})");
    assert_eq!(ticks, intial_tick);

    Timer::after_millis(110).await;

    let start_time = Instant::now();
    while start_time.elapsed() < Duration::from_secs(2) {
        let seconds = Instant::now().as_millis() as f32 / 1e3;

        println!("[t1] -- At time: {seconds} secs ({:?})", Instant::now());
        Timer::after_millis(200).await;
    }
}

// Some task doing some time-dependent stuff
#[embassy_executor::task]
pub async fn task2(intial_tick: u64) {
    let ticks = Instant::now().as_ticks();
    println!("Hello from task2 (ticks: {ticks})");
    assert_eq!(ticks, intial_tick);

    Timer::after_millis(120).await;

    let start_time = Instant::now();
    while start_time.elapsed() < Duration::from_secs(2) {
        let seconds = Instant::now().as_millis() as f32 / 1e3;

        println!("[t2] -- At time: {seconds} secs ({:?})", Instant::now());
        Timer::after_millis(200).await;
    }
}
