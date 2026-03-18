use std::cell::Cell;

pub(crate) use rust_cc::weak::Weak;
pub(crate) use rust_cc::{collect_cycles, Cc, Context, Finalize, Trace};

pub const DEFAULT_ALLOCATION_THRESHOLD: usize = 1024;

thread_local! {
    static ALLOCATION_DEBT: Cell<usize> = const { Cell::new(0) };
    static ALLOCATION_THRESHOLD: Cell<usize> = const { Cell::new(DEFAULT_ALLOCATION_THRESHOLD) };
}

#[inline]
pub(crate) fn alloc<T: Trace>(value: T) -> Cc<T> {
    let gc_value = Cc::new(value);
    charge_allocation();
    gc_value
}

#[inline]
pub(crate) fn alloc_cyclic<T: Trace, F>(f: F) -> Cc<T>
where
    F: FnOnce(&Weak<T>) -> T,
{
    let gc_value = Cc::new_cyclic(f);
    charge_allocation();
    gc_value
}

#[inline]
fn charge_allocation() {
    ALLOCATION_DEBT.with(|debt| {
        let next = debt.get().saturating_add(1);
        debt.set(next);
        if next >= allocation_threshold() {
            collect();
        }
    });
}

#[inline]
pub fn collect() {
    collect_cycles();
    ALLOCATION_DEBT.with(|debt| debt.set(0));
}

#[inline]
pub(crate) fn allocation_threshold() -> usize {
    ALLOCATION_THRESHOLD.with(|threshold| threshold.get().max(1))
}

pub(crate) struct AllocationThresholdGuard {
    previous_threshold: usize,
}

impl AllocationThresholdGuard {
    #[inline]
    pub(crate) fn new(threshold: usize) -> Self {
        let threshold = threshold.max(1);
        let previous_threshold = ALLOCATION_THRESHOLD.with(|cell| cell.replace(threshold));
        Self { previous_threshold }
    }
}

impl Drop for AllocationThresholdGuard {
    fn drop(&mut self) {
        ALLOCATION_THRESHOLD.with(|cell| {
            cell.set(self.previous_threshold);
        });
    }
}
