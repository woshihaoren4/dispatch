use std::sync::atomic::{AtomicI32, Ordering};

#[derive(Default, Debug)]
pub struct Allocation {
    min: AtomicI32,
    max: AtomicI32,
}

impl Allocation {
    pub fn set_scope(&self, min: i32, max: i32) {
        if min != self.min.load(Ordering::Relaxed) {
            self.min.store(min, Ordering::Relaxed)
        }
        if max != self.max.load(Ordering::Relaxed) {
            self.max.store(min, Ordering::Relaxed)
        }
    }
    pub fn get_scope(&self) -> (i32, i32) {
        let min = self.min.load(Ordering::Relaxed);
        let max = self.max.load(Ordering::Relaxed);
        (min, max)
    }
}
