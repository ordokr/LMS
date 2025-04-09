use std::sync::atomic::{AtomicUsize, Ordering};
use log::debug;

// Track memory usage
static ALLOCATED_MEMORY: AtomicUsize = AtomicUsize::new(0);
static ALLOCATION_COUNT: AtomicUsize = AtomicUsize::new(0);
static DEALLOCATION_COUNT: AtomicUsize = AtomicUsize::new(0);

// Memory optimization utilities
pub fn optimize_memory() {
    // Force garbage collection
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::global().dyn_ref::<js_sys::Function>()
            .and_then(|gc| {
                let gc = js_sys::Function::new_no_args("if (window.gc) { window.gc(); }");
                let _ = gc.call0(&JsValue::NULL);
                Some(())
            });
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // For native Rust, we can't force GC, but we can provide memory stats
        debug!("Memory stats - Allocated: {}kb, Allocations: {}, Deallocations: {}", 
            ALLOCATED_MEMORY.load(Ordering::Relaxed) / 1024,
            ALLOCATION_COUNT.load(Ordering::Relaxed),
            DEALLOCATION_COUNT.load(Ordering::Relaxed)
        );
    }
}

// Memory-efficient string storage for repeated strings
pub struct StringInterner {
    strings: dashmap::DashMap<u64, String>,
}

impl StringInterner {
    pub fn new() -> Self {
        Self {
            strings: dashmap::DashMap::new(),
        }
    }
    
    // Get or intern a string
    pub fn intern(&self, s: &str) -> u64 {
        use std::hash::{Hash, Hasher};
        
        let mut hasher = ahash::AHasher::default();
        s.hash(&mut hasher);
        let hash = hasher.finish();
        
        if !self.strings.contains_key(&hash) {
            self.strings.insert(hash, s.to_string());
        }
        
        hash
    }
    
    // Get a string by its hash
    pub fn get(&self, hash: u64) -> Option<String> {
        self.strings.get(&hash).map(|s| s.clone())
    }
}

lazy_static::lazy_static! {
    static ref STRING_INTERNER: StringInterner = StringInterner::new();
}

pub fn get_string_interner() -> &'static StringInterner {
    &STRING_INTERNER
}