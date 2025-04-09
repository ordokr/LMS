use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::marker::PhantomData;

// Generic object pool for reusing expensive objects
pub struct ObjectPool<T: Default + Clone> {
    objects: Arc<Mutex<VecDeque<T>>>,
    max_size: usize,
}

impl<T: Default + Clone> ObjectPool<T> {
    pub fn new(initial_size: usize, max_size: usize) -> Self {
        let mut objects = VecDeque::with_capacity(initial_size);
        for _ in 0..initial_size {
            objects.push_back(T::default());
        }
        
        Self {
            objects: Arc::new(Mutex::new(objects)),
            max_size,
        }
    }
    
    pub fn acquire(&self) -> PooledObject<T> {
        let mut pool = self.objects.lock().unwrap();
        let object = pool.pop_front().unwrap_or_default();
        
        PooledObject {
            object: Some(object),
            pool: self.objects.clone(),
            max_size: self.max_size,
        }
    }
}

// RAII wrapper for automatically returning objects to pool
pub struct PooledObject<T: Default + Clone> {
    object: Option<T>,
    pool: Arc<Mutex<VecDeque<T>>>,
    max_size: usize,
}

impl<T: Default + Clone> Drop for PooledObject<T> {
    fn drop(&mut self) {
        if let Some(obj) = self.object.take() {
            let mut pool = self.pool.lock().unwrap();
            if pool.len() < self.max_size {
                pool.push_back(obj);
            }
        }
    }
}

impl<T: Default + Clone> std::ops::Deref for PooledObject<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        self.object.as_ref().unwrap()
    }
}

impl<T: Default + Clone> std::ops::DerefMut for PooledObject<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.object.as_mut().unwrap()
    }
}

// Example: Buffer pool for efficient memory reuse
pub type BufferPool = ObjectPool<Vec<u8>>;

#[derive(Default, Clone)]
pub struct FormattedMessage {
    buffer: Vec<String>,
    capacity: usize,
}

impl FormattedMessage {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            capacity,
        }
    }
    
    pub fn add(&mut self, msg: String) {
        self.buffer.push(msg);
    }
    
    pub fn clear(&mut self) {
        self.buffer.clear();
    }
    
    pub fn as_string(&self) -> String {
        self.buffer.join("\n")
    }
}

// Example: Message formatter pool
pub type MessageFormatterPool = ObjectPool<FormattedMessage>;

// Usage example for forum post rendering
pub fn create_post_formatter_pool() -> MessageFormatterPool {
    MessageFormatterPool::new(20, 100)
}

pub fn format_post(pool: &MessageFormatterPool, content: &str) -> String {
    let mut formatter = pool.acquire();
    formatter.clear();
    
    // Split and process content
    for line in content.lines() {
        formatter.add(line.to_string());
    }
    
    formatter.as_string()
}