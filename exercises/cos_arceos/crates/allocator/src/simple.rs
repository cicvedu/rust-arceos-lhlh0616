//! Simple memory allocation.
//!
//! TODO: more efficient

use core::alloc::Layout;
use core::num::NonZeroUsize;

pub trait AllocResult<T> {
    type Error;
    fn result(value: T) -> Result<T, Self::Error>;
}

pub trait AbstractAllocator {
    fn init(&mut self, start: usize, size: usize);
}

impl<T> AllocResult<T> for T {
    type Error = ();
    fn result(value: T) -> Result<T, Self::Error> {
        Ok(value)
    }
}

pub struct SimpleByteAllocator {
    start: usize,
    next: usize,
    allocations: usize,
    end: usize,
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            start: 0,
            next: 0,
            allocations: 0,
            end: 0,
        }
    }
}

impl AbstractAllocator for SimpleByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.next = self.start;
        self.end = self.start + size;
        self.allocations = 0;
    }
}

pub trait ByteAllocator: AbstractAllocator {
    fn alloc(&mut self, layout: Layout) -> Result<NonZeroUsize, ()>;
    fn dealloc(&mut self, pos: NonZeroUsize, layout: Layout);
    fn total_bytes(&self) -> usize;
    fn used_bytes(&self) -> usize;
    fn free_bytes(&self) -> usize {
        self.total_bytes() - self.used_bytes()
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, layout: Layout) -> Result<NonZeroUsize, ()> {
        let size = layout.size();
        let align = layout.align();
        let align_mask = !(align - 1);

        let start = (self.next + align - 1) & align_mask;

        if start + size > self.end {
            Err(())
        } else {
            self.allocations += 1;
            self.next = start + size;
            Ok(unsafe { NonZeroUsize::new_unchecked(start) })
        }
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        self.allocations -= 1;
        if self.allocations == 0 {
            self.next = self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    fn used_bytes(&self) -> usize {
        self.next - self.start
    }
}
