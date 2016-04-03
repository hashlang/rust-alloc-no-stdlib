#![no_std]

mod allocated_memory;
mod stack_allocator;
mod allocated_stack_memory;
pub mod init;
mod tests;
pub use allocated_memory::SliceWrapper;
pub use allocated_memory::SliceWrapperMut;
pub use allocated_memory::AllocatedSlice;

pub use allocated_stack_memory::AllocatedStackMemory;
pub use stack_allocator::Allocator;
pub use stack_allocator::StackAllocator;

