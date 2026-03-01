use std::time::Duration;

use futures::{StreamExt, stream};

pub async fn special_handler_0_async() -> u16 {
    tokio::time::sleep(Duration::from_nanos(5000)).await;
    0
}

pub fn special_handler_0() -> u16 {
    std::thread::sleep(Duration::from_nanos(5000));
    0
}

pub fn normal_handler_non_0(number: u8) -> u16 {
    (number as u16) * 2
}

pub async fn process_number_async(input: u8) -> u16 {
    if input == 0 {
        special_handler_0_async().await
    } else {
        normal_handler_non_0(input)
    }
}

pub fn process_number(input: u8) -> u16 {
    if input == 0 {
        special_handler_0()
    } else {
        normal_handler_non_0(input)
    }
}

pub async fn process_async(input: &[u8]) -> Vec<u16> {
    let x = stream::iter(input)
        .map(|n| process_number_async(*n))
        .buffer_unordered(4)
        .collect()
        .await;
    x
}

pub fn process(input: &[u8]) -> Vec<u16> {
    let x = input.iter().map(|n| process_number(*n)).collect();
    x
}
