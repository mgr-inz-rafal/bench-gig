#![feature(likely_unlikely)]

use rand::{RngExt, rngs::ThreadRng};
use std::hint::unlikely;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Foo {
    pub id: u8,
    pub value: u64,
    pub dirty_flag: bool,
    pub timestamp: u64,
}

impl Foo {
    pub fn random(rng: &mut ThreadRng) -> Self {
        Self {
            id: rng.random(),
            value: rng.random(),
            // High chance of generating Foo in dirty state.
            dirty_flag: if rng.random::<u8>() != 0 { true } else { false },
            timestamp: rng.random(),
        }
    }
}

pub fn find_oldest(data: &[Foo]) -> &Foo {
    data.iter()
        .filter(|f| !f.dirty_flag)
        .min_by_key(|f| f.timestamp)
        .unwrap()
}

pub fn find_oldest_alternative(data: &[Foo]) -> &Foo {
    let mut oldest: Option<&Foo> = None;
    for item in data.iter() {
        if !item.dirty_flag {
            if oldest.is_none() || item.timestamp < oldest.unwrap().timestamp {
                oldest = Some(item);
            }
        }
    }
    oldest.unwrap()
}

pub fn find_oldest_alternative_with_hint(data: &[Foo]) -> &Foo {
    let mut oldest: Option<&Foo> = None;
    for item in data.iter() {
        if unlikely(!item.dirty_flag) {
            if oldest.is_none() || item.timestamp < oldest.unwrap().timestamp {
                oldest = Some(item);
            }
        }
    }
    oldest.unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn size_of_foo() {
        println!("Foo size: {}", std::mem::size_of::<super::Foo>());
        println!("Foo align: {}", std::mem::align_of::<super::Foo>());
    }
}
