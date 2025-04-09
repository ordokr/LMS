use leptos::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use wasm_bindgen::prelude::*;

// Memory profiling system for Leptos apps
#[derive(Default)]
pub struct MemoryProfile {
    allocated_objects: Rc<RefCell<HashMap<String, usize>>>,
    allocation_stack: Rc<RefCell<Vec<String>>>,
    total_allocated: Rc<RefCell<usize>>,
    snapshot_history: Rc<RefCell<Vec<(Instant, HashMap<String, usize>)>>>,
    
    // CPU timing information
    timings: Rc<RefCell<HashMap<String, Vec<Duration>>>>,
    ongoing_operations: Rc<RefCell<HashMap<String, Instant>>>,
}

impl MemoryProfile {
    pub fn new() -> Self {
        Default::default()
    }
    
    // Track allocations by type
    pub fn track_allocation(&self, type_name: &str, size: usize) {
        let mut objects = self.allocated_objects.borrow_mut();
        *objects.entry(type_name.to_string()).or_insert(0) += 1;
        
        let mut total = self.total_allocated.borrow_mut();
        *total += size;
    }
    
    // Release tracked allocations
    pub fn track_deallocation(&self, type_name: &str, size: usize) {
        let mut objects = self.allocated_objects.borrow_mut();
        if let Some(count) = objects.get_mut(type_name) {
            if *count > 0 {
                *count -= 1;
            }
        }
        
        let mut total = self.total_allocated.borrow_mut();
        if *total >= size {
            *total -= size;
        }
    }
    
    // Enter a memory allocation context
    pub fn enter_context(&self, context_name: &str) {
        let mut stack = self.allocation_stack.borrow_mut();
        stack.push(context_name.to_string());
    }
    
    // Exit current memory allocation context
    pub fn exit_context(&self) {
        let mut stack = self.allocation_stack.borrow_mut();
        stack.pop();
    }
    
    // Get current allocation context
    pub fn get_context(&self) -> String {
        let stack = self.allocation_stack.borrow();
        if stack.is_empty() {
            "global".to_string()
        } else {
            stack.last().unwrap().clone()
        }
    }
    
    // Take a memory snapshot
    pub fn take_snapshot(&self) {
        let objects = self.allocated_objects.borrow().clone();
        let mut snapshots = self.snapshot_history.borrow_mut();
        
        snapshots.push((Instant::now(), objects));
        
        // Keep only the last 10 snapshots
        if snapshots.len() > 10 {
            snapshots.remove(0);
        }
    }
    
    // Start timing an operation
    pub fn start_operation(&self, name: &str) {
        let mut operations = self.ongoing_operations.borrow_mut();
        operations.insert(name.to_string(), Instant::now());
    }
    
    // End timing an operation
    pub fn end_operation(&self, name: &str) {
        let mut operations = self.ongoing_operations.borrow_mut();
        
        if let Some(start_time) = operations.remove(name) {
            let duration = start_time.elapsed();
            
            let mut timings = self.timings.borrow_mut();
            timings.entry(name.to_string())
                .or_insert_with(Vec::new)
                .push(duration);
        }
    }
    
    // Get timing statistics
    pub fn get_timing_stats(&self) -> HashMap<String, (Duration, Duration, Duration)> {
        let timings = self.timings.borrow();
        let mut result = HashMap::new();
        
        for (name, durations) in timings.iter() {
            if durations.is_empty() {
                continue;
            }
            
            // Calculate min, max, avg
            let min = durations.iter().min().cloned().unwrap_or(Duration::from_secs(0));
            let max = durations.iter().max().cloned().unwrap_or(Duration::from_secs(0));
            
            let total = durations.iter()
                .fold(Duration::from_secs(0), |acc, &d| acc + d);
                
            let avg = if !durations.is_empty() {
                Duration::from_nanos(total.as_nanos() as u64 / durations.len() as u64)
            } else {
                Duration::from_secs(0)
            };
            
            result.insert(name.clone(), (min, avg, max));
        }
        
        result
    }
    
    // Get memory statistics
    pub fn get_memory_stats(&self) -> HashMap<String, usize> {
        self.allocated_objects.borrow().clone()
    }
    
    // Get memory overview
    pub fn get_memory_overview(&self) -> MemoryOverview {
        let objects = self.allocated_objects.borrow();
        let total = *self.total_allocated.borrow();
        
        // Find top allocations
        let mut sorted_types: Vec<_> = objects.iter().collect();
        sorted_types.sort_by(|a, b| b.1.cmp(a.1));
        
        let top_types = sorted_types.iter()
            .take(5)
            .map(|(k, v)| (k.clone(), *v))
            .collect();
            
        MemoryOverview {
            total_allocated: total,
            allocation_count: objects.values().sum(),
            top_allocation_types: top_types,
        }
    }
}

// Memory overview structure for UI display
pub struct MemoryOverview {
    pub total_allocated: usize,
    pub allocation_count: usize,
    pub top_allocation_types: Vec<(String, usize)>,
}

// Global instance
thread_local! {
    static MEMORY_PROFILER: MemoryProfile = MemoryProfile::new();
}

// Wrapper for objects to track memory usage
pub struct Tracked<T> {
    value: T,
    type_name: String,
    size: usize,
}

impl<T> Tracked<T> {
    pub fn new(value: T) -> Self {
        let type_name = std::any::type_name::<T>().to_string();
        let size = std::mem::size_of::<T>();
        
        MEMORY_PROFILER.with(|profiler| {
            profiler.track_allocation(&type_name, size);
        });
        
        Self {
            value,
            type_name,
            size,
        }
    }
    
    pub fn get(&self) -> &T {
        &self.value
    }
    
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }
    
    pub fn into_inner(self) -> T {
        // Will be tracked for deallocation in drop
        self.value
    }
}

impl<T> std::ops::Deref for Tracked<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> std::ops::DerefMut for Tracked<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> Drop for Tracked<T> {
    fn drop(&mut self) {
        MEMORY_PROFILER.with(|profiler| {
            profiler.track_deallocation(&self.type_name, self.size);
        });
    }
}

// Operation timing helper
pub struct OperationTimer<'a> {
    name: &'a str,
}

impl<'a> OperationTimer<'a> {
    pub fn new(name: &'a str) -> Self {
        MEMORY_PROFILER.with(|profiler| {
            profiler.start_operation(name);
        });
        
        Self { name }
    }
}

impl<'a> Drop for OperationTimer<'a> {
    fn drop(&mut self) {
        MEMORY_PROFILER.with(|profiler| {
            profiler.end_operation(self.name);
        });
    }
}

// WASM bindings for browser usage
#[wasm_bindgen]
pub fn take_memory_snapshot() {
    MEMORY_PROFILER.with(|profiler| {
        profiler.take_snapshot();
    });
}

#[wasm_bindgen]
pub fn get_memory_overview() -> JsValue {
    let overview = MEMORY_PROFILER.with(|profiler| {
        profiler.get_memory_overview()
    });
    
    let result = serde_wasm_bindgen::to_value(&JsOverview {
        total_allocated: overview.total_allocated,
        allocation_count: overview.allocation_count,
        top_allocation_types: overview.top_allocation_types
            .into_iter()
            .map(|(name, count)| JsAllocation { name, count })
            .collect(),
    }).unwrap_or(JsValue::NULL);
    
    result
}

// JS-compatible structures for WASM export
#[derive(serde::Serialize)]
struct JsOverview {
    total_allocated: usize,
    allocation_count: usize,
    top_allocation_types: Vec<JsAllocation>,
}

#[derive(serde::Serialize)]
struct JsAllocation {
    name: String,
    count: usize,
}

// Convenience macros
#[macro_export]
macro_rules! track {
    ($expr:expr) => {
        $crate::utils::profiler::Tracked::new($expr)
    };
}

#[macro_export]
macro_rules! time_operation {
    ($name:expr, $expr:expr) => {{
        let _timer = $crate::utils::profiler::OperationTimer::new($name);
        $expr
    }};
}

// Hook for component profiling
#[hook]
pub fn use_profiled_component(name: &str) {
    let name = name.to_string();
    
    // Track renders
    create_effect(move |_| {
        let _timer = OperationTimer::new(&format!("render_{}", name));
        
        // Track memory during component lifecycle
        MEMORY_PROFILER.with(|profiler| {
            profiler.enter_context(&format!("component_{}", name));
        });
        
        on_cleanup(move || {
            MEMORY_PROFILER.with(|profiler| {
                profiler.exit_context();
            });
        });
    });
}