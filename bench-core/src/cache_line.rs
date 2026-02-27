use std::sync::Arc;
use std::thread;

pub struct SharedCacheLine {
    pub a: u64,
    pub b: u64,
}

pub struct PaddedCacheLine {
    pub a: u64,
    _pad: [u8; 56],
    pub b: u64,
}

impl SharedCacheLine {
    pub fn new() -> Self {
        Self { a: 0, b: 0 }
    }
}

impl PaddedCacheLine {
    pub fn new() -> Self {
        Self {
            a: 0,
            _pad: [0u8; 56],
            b: 0,
        }
    }
}

const ITERATIONS: u64 = 10_000_000;

pub fn run_shared(counters: Arc<SharedCacheLine>) {
    let c1 = Arc::clone(&counters);
    let c2 = Arc::clone(&counters);

    let t1 = thread::spawn(move || {
        for _ in 0..ITERATIONS {
            unsafe {
                let ptr = &c1.a as *const u64 as *mut u64;
                ptr.write_volatile(ptr.read_volatile().wrapping_add(1));
            }
        }
    });

    let t2 = thread::spawn(move || {
        for _ in 0..ITERATIONS {
            unsafe {
                let ptr = &c2.b as *const u64 as *mut u64;
                ptr.write_volatile(ptr.read_volatile().wrapping_add(1));
            }
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

pub fn run_padded(counters: Arc<PaddedCacheLine>) {
    let c1 = Arc::clone(&counters);
    let c2 = Arc::clone(&counters);

    let t1 = thread::spawn(move || {
        for _ in 0..ITERATIONS {
            unsafe {
                let ptr = &c1.a as *const u64 as *mut u64;
                ptr.write_volatile(ptr.read_volatile().wrapping_add(1));
            }
        }
    });

    let t2 = thread::spawn(move || {
        for _ in 0..ITERATIONS {
            unsafe {
                let ptr = &c2.b as *const u64 as *mut u64;
                ptr.write_volatile(ptr.read_volatile().wrapping_add(1));
            }
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
