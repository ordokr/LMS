use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::RefCell;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};

// Memory usage statistics
static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static DEALLOCATED: AtomicUsize = AtomicUsize::new(0);

// Custom arena allocator for forum post processing
#[derive(Debug)]
pub struct ArenaAllocator {
    arena: RefCell<Arena>,
}

#[derive(Debug)]
struct Arena {
    blocks: Vec<Block>,
    current_block: Option<NonNull<u8>>,
    current_offset: usize,
    current_capacity: usize,
    min_block_size: usize,
}

#[derive(Debug)]
struct Block {
    ptr: NonNull<u8>,
    layout: Layout,
}

impl ArenaAllocator {
    pub fn new(initial_size: usize) -> Self {
        Self {
            arena: RefCell::new(Arena {
                blocks: Vec::new(),
                current_block: None,
                current_offset: 0,
                current_capacity: 0,
                min_block_size: initial_size,
            }),
        }
    }
    
    pub fn allocate(&self, layout: Layout) -> *mut u8 {
        self.arena.borrow_mut().allocate(layout)
    }
    
    pub fn reset(&self) {
        self.arena.borrow_mut().reset();
    }
    
    // Get memory usage statistics
    pub fn get_stats(&self) -> (usize, usize, usize) {
        let arena = self.arena.borrow();
        let total_allocated = ALLOCATED.load(Ordering::Relaxed);
        let total_deallocated = DEALLOCATED.load(Ordering::Relaxed);
        let current_usage = arena.blocks.iter()
            .map(|block| block.layout.size())
            .sum::<usize>();
            
        (total_allocated, total_deallocated, current_usage)
    }
}

impl Arena {
    fn allocate(&mut self, layout: Layout) -> *mut u8 {
        // Ensure proper alignment
        let align = layout.align();
        let size = layout.size();
        
        // Calculate aligned offset
        let aligned_offset = (self.current_offset + align - 1) & !(align - 1);
        
        // Check if we have enough space in current block
        if let Some(current_block) = self.current_block {
            if aligned_offset + size <= self.current_capacity {
                // We have enough space
                let ptr = unsafe { current_block.as_ptr().add(aligned_offset) };
                self.current_offset = aligned_offset + size;
                ALLOCATED.fetch_add(size, Ordering::Relaxed);
                return ptr;
            }
        }
        
        // Allocate new block
        let block_size = std::cmp::max(size, self.min_block_size);
        let block_layout = Layout::from_size_align(block_size, align).unwrap();
        
        let block_ptr = unsafe { 
            let ptr = System.alloc(block_layout);
            NonNull::new(ptr).expect("Failed to allocate memory for arena") 
        };
        
        // Store block for later deallocation
        self.blocks.push(Block {
            ptr: block_ptr,
            layout: block_layout,
        });
        
        // Update current block info
        self.current_block = Some(block_ptr);
        self.current_offset = size;
        self.current_capacity = block_size;
        
        ALLOCATED.fetch_add(size, Ordering::Relaxed);
        block_ptr.as_ptr()
    }
    
    fn reset(&mut self) {
        // Reset the offset but keep blocks allocated
        self.current_offset = 0;
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        // Free all allocated blocks
        for block in &self.blocks {
            unsafe {
                System.dealloc(block.ptr.as_ptr(), block.layout);
                DEALLOCATED.fetch_add(block.layout.size(), Ordering::Relaxed);
            }
        }
    }
}

// Thread-local arena for efficient thread-specific allocations
thread_local! {
    static THREAD_ARENA: RefCell<ArenaAllocator> = RefCell::new(ArenaAllocator::new(1024 * 64)); // 64KB
}

// Helper to use thread-local arena
pub fn with_thread_arena<F, R>(f: F) -> R
where
    F: FnOnce(&ArenaAllocator) -> R,
{
    THREAD_ARENA.with(|arena_cell| {
        let arena = arena_cell.borrow();
        let result = f(&arena);
        // Optional: Reset arena after use
        // arena.reset();
        result
    })
}