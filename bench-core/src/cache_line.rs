use crossbeam_utils::CachePadded;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

pub struct SharedCacheLine {
    pub a: AtomicU64,
    pub b: AtomicU64,
}

impl SharedCacheLine {
    pub fn new() -> Self {
        Self {
            a: AtomicU64::new(0),
            b: AtomicU64::new(0),
        }
    }
}

pub struct PaddedCacheLine {
    pub a: AtomicU64,
    _pad: [u8; 56],
    pub b: AtomicU64,
}

impl PaddedCacheLine {
    pub fn new() -> Self {
        Self {
            a: AtomicU64::new(0),
            _pad: [0u8; 56],
            b: AtomicU64::new(0),
        }
    }
}

pub struct CrossBeamCacheLine {
    pub a: CachePadded<AtomicU64>,
    pub b: CachePadded<AtomicU64>,
}

impl CrossBeamCacheLine {
    pub fn new() -> Self {
        Self {
            a: CachePadded::new(AtomicU64::new(0)),
            b: CachePadded::new(AtomicU64::new(0)),
        }
    }
}

const ITERATIONS: u64 = 10_000_000;

pub fn run_shared(counters: Arc<SharedCacheLine>) {
    let c1 = Arc::clone(&counters);
    let c2 = Arc::clone(&counters);

    let t1 = thread::spawn(move || {
        for _ in 0..ITERATIONS {
            let v = c1.a.load(Ordering::Relaxed);
            c1.a.store(v.wrapping_add(1), Ordering::Relaxed);
        }
    });

    let t2 = thread::spawn(move || {
        for _ in 0..ITERATIONS {
            let v = c2.b.load(Ordering::Relaxed);
            c2.b.store(v.wrapping_add(1), Ordering::Relaxed);
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
            let v = c1.a.load(Ordering::Relaxed);
            c1.a.store(v.wrapping_add(1), Ordering::Relaxed);
        }
    });

    let t2 = thread::spawn(move || {
        for _ in 0..ITERATIONS {
            let v = c2.b.load(Ordering::Relaxed);
            c2.b.store(v.wrapping_add(1), Ordering::Relaxed);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

pub fn run_crossbeam_padded(counters: Arc<CrossBeamCacheLine>) {
    let c1 = Arc::clone(&counters);
    let c2 = Arc::clone(&counters);

    let t1 = thread::spawn(move || {
        for _ in 0..ITERATIONS {
            let v = c1.a.load(Ordering::Relaxed);
            c1.a.store(v.wrapping_add(1), Ordering::Relaxed);
        }
    });

    let t2 = thread::spawn(move || {
        for _ in 0..ITERATIONS {
            let v = c2.b.load(Ordering::Relaxed);
            c2.b.store(v.wrapping_add(1), Ordering::Relaxed);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
