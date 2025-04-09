use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use std::ptr;
use std::ffi::c_void;

// Foreign function declarations for Haskell runtime
extern "C" {
    fn hs_init(argc: *mut i32, argv: *mut *mut *mut i8);
    fn hs_exit();
    fn hs_perform_gc();
    fn hs_free_stable_ptr(ptr: *mut c_void);
    fn hs_process_batch(ops: *const c_void, count: u64) -> *mut c_void;
    fn hs_get_result_count(result: *const c_void) -> u64;
    fn hs_get_results(result: *const c_void, out: *mut c_void, count: u64);
    fn hs_free_result(result: *mut c_void);
}

struct HaskellArena {
    haskell_heap: Mutex<*mut c_void>,
    rust_buffer: Arc<Mutex<Vec<u8>>>,
}

impl HaskellArena {
    fn new() -> Self {
        // Initialize Haskell runtime if needed
        unsafe {
            static mut INITIALIZED: bool = false;
            if !INITIALIZED {
                let mut argc = 1;
                let prog = std::ffi::CString::new("lms").unwrap();
                let mut argv = vec![prog.as_ptr() as *mut i8];
                hs_init(&mut argc, &mut argv.as_mut_ptr());
                INITIALIZED = true;
            }
        }
        
        Self {
            haskell_heap: Mutex::new(ptr::null_mut()),
            rust_buffer: Arc::new(Mutex::new(Vec::with_capacity(1024 * 1024))), // 1MB initial
        }
    }
    
    fn heap_ptr(&self) -> *mut c_void {
        *self.haskell_heap.lock().unwrap()
    }
    
    fn ensure_capacity(&self, size_bytes: usize) {
        let mut buffer = self.rust_buffer.lock().unwrap();
        if buffer.capacity() < size_bytes {
            buffer.reserve(size_bytes - buffer.capacity());
        }
    }
    
    fn usage_percent(&self) -> f64 {
        let buffer = self.rust_buffer.lock().unwrap();
        buffer.len() as f64 / buffer.capacity() as f64 * 100.0
    }
}

impl Drop for HaskellArena {
    fn drop(&mut self) {
        let ptr = *self.haskell_heap.lock().unwrap();
        if !ptr.is_null() {
            unsafe { hs_free_stable_ptr(ptr) };
        }
    }
}

struct GCScheduler {
    interval: Duration,
    max_duration: Duration,
    thread_handle: Option<thread::JoinHandle<()>>,
    running: Arc<Mutex<bool>>,
}

impl GCScheduler {
    fn new(interval: Duration, max_duration: Duration) -> Self {
        let running = Arc::new(Mutex::new(true));
        let running_clone = running.clone();
        
        let handle = thread::spawn(move || {
            while *running_clone.lock().unwrap() {
                thread::sleep(interval);
                unsafe { hs_perform_gc(); }
            }
        });
        
        Self {
            interval,
            max_duration,
            thread_handle: Some(handle),
            running,
        }
    }
    
    fn request_collection(&self) {
        unsafe { hs_perform_gc(); }
    }
}

impl Drop for GCScheduler {
    fn drop(&mut self) {
        *self.running.lock().unwrap() = false;
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
}

pub struct HaskellRuntime {
    arena: HaskellArena,
    gc_scheduler: GCScheduler,
}

impl HaskellRuntime {
    pub fn new() -> Self {
        let arena = HaskellArena::new();
        
        // Schedule incremental GC to prevent UI pauses
        let gc_scheduler = GCScheduler::new(
            Duration::from_millis(10),  // Run every 10ms
            Duration::from_millis(1)    // Max 1ms per collection
        );
        
        Self { arena, gc_scheduler }
    }
    
    pub fn new_with_limits(heap_size: usize, stack_size: usize) -> Self {
        // Would set GHC RTS options for heap and stack limits
        // For demonstration purposes, just creates a regular runtime
        Self::new()
    }
    
    pub fn process_batch(&self, operations: &[SyncOperation]) -> Vec<ResolvedOperation> {
        // Pre-allocate memory for operation
        self.arena.ensure_capacity(operations.len() * std::mem::size_of::<SyncOperation>());
        
        // Run Haskell computation
        let result = unsafe { 
            let ops_ptr = operations.as_ptr() as *const c_void;
            let result_ptr = hs_process_batch(ops_ptr, operations.len() as u64);
            
            let count = hs_get_result_count(result_ptr);
            let mut resolved = vec![ResolvedOperation::default(); count as usize];
            hs_get_results(result_ptr, resolved.as_mut_ptr() as *mut c_void, count);
            
            hs_free_result(result_ptr);
            resolved
        };
        
        // Trigger minor GC if we're above threshold
        if self.arena.usage_percent() > 70.0 {
            self.gc_scheduler.request_collection();
        }
        
        result
    }
    
    pub fn peak_memory_mb(&self) -> f64 {
        // In a real implementation, would query GHC RTS for memory stats
        // For demonstration, returning a placeholder
        42.0
    }
}