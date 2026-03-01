use std::time::Instant;

use bench_core::{foo::Foo, process_bottleneck};
use rand::{self, RngExt};

async fn process_fast_async() {
    let mut rng = rand::rng();
    let numbers: Vec<_> = std::iter::repeat_with(|| rng.random())
        .filter(|x| x != &0)
        .take(1_000_000)
        .collect();
    let _processed = process_bottleneck::process_async(&numbers).await;
}

async fn process_slow_async() {
    let mut rng = rand::rng();
    let numbers: Vec<_> = std::iter::repeat_with(|| rng.random())
        .take(1_000_000)
        .collect();
    let _processed = process_bottleneck::process_async(&numbers).await;
}

fn process_fast() {
    let mut rng = rand::rng();
    let numbers: Vec<_> = std::iter::repeat_with(|| rng.random())
        .filter(|x| x != &0)
        .take(1_000_000)
        .collect();
    let _processed = process_bottleneck::process(&numbers);
}

fn process_slow() {
    let mut rng = rand::rng();
    let numbers: Vec<_> = std::iter::repeat_with(|| rng.random())
        .take(1_000_000)
        .collect();
    let _processed = process_bottleneck::process(&numbers);
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    /*
    let mut rng = rand::rng();
    let foo = Foo::random(&mut rng);
    dbg!(&foo);
    */

    let start = Instant::now();
    process_fast_async().await;
    let elapsed = start.elapsed();
    println!("Fast process async: {elapsed:?}");

    let start = Instant::now();
    process_slow_async().await;
    let elapsed = start.elapsed();
    println!("Slow process async: {elapsed:?}");

    /*
    let start = Instant::now();
    process_fast();
    let elapsed = start.elapsed();
    println!("Fast process: {elapsed:?}");

    let start = Instant::now();
    process_slow();
    let elapsed = start.elapsed();
    println!("Slow process: {elapsed:?}");
    */
}
