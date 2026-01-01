//! WASM memory optimization and management
//!
//! This module provides memory management utilities optimized for WASM constraints,
//! including memory pooling, allocation tracking, and garbage collection helpers.

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Memory pool for efficient WASM allocation
pub struct WasmMemoryPool {
    max_memory_mb: usize,
    current_usage: AtomicUsize,
    allocation_count: AtomicUsize,
    deallocation_count: AtomicUsize,
}

impl WasmMemoryPool {
    /// Create a new memory pool with specified max memory in MB
    pub fn new(max_memory_mb: usize) -> Self {
        WasmMemoryPool {
            max_memory_mb,
            current_usage: AtomicUsize::new(0),
            allocation_count: AtomicUsize::new(0),
            deallocation_count: AtomicUsize::new(0),
        }
    }

    /// Check if allocation is allowed
    pub fn check_allocation(&self, size: usize) -> Result<(), String> {
        let current = self.current_usage.load(Ordering::Relaxed);
        let max_bytes = self.max_memory_mb * 1024 * 1024;

        if current + size > max_bytes {
            return Err(format!(
                "Memory limit exceeded: requested {} bytes, {} bytes used of {} bytes max",
                size, current, max_bytes
            ));
        }

        Ok(())
    }

    /// Track an allocation
    pub fn allocate(&self, size: usize) -> Result<AllocationToken, String> {
        self.check_allocation(size)?;

        self.current_usage.fetch_add(size, Ordering::Relaxed);
        self.allocation_count.fetch_add(1, Ordering::Relaxed);

        Ok(AllocationToken {
            size,
            id: self.allocation_count.load(Ordering::Relaxed),
        })
    }

    /// Track a deallocation
    pub fn deallocate(&self, token: AllocationToken) {
        self.current_usage.fetch_sub(token.size, Ordering::Relaxed);
        self.deallocation_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Get current memory usage in bytes
    pub fn current_usage(&self) -> usize {
        self.current_usage.load(Ordering::Relaxed)
    }

    /// Get memory statistics
    pub fn stats(&self) -> MemoryStats {
        MemoryStats {
            current_usage_bytes: self.current_usage.load(Ordering::Relaxed),
            max_usage_bytes: self.max_memory_mb * 1024 * 1024,
            allocation_count: self.allocation_count.load(Ordering::Relaxed),
            deallocation_count: self.deallocation_count.load(Ordering::Relaxed),
            current_usage_mb: self.current_usage.load(Ordering::Relaxed) as f64 / (1024.0 * 1024.0),
            max_usage_mb: self.max_memory_mb as f64,
            usage_percent: (self.current_usage.load(Ordering::Relaxed) as f64
                / (self.max_memory_mb * 1024 * 1024) as f64) * 100.0,
        }
    }

    /// Clear all tracked allocations (for cleanup)
    pub fn clear(&self) {
        self.current_usage.store(0, Ordering::Relaxed);
    }
}

/// Token representing an allocation
#[derive(Debug, Clone)]
pub struct AllocationToken {
    size: usize,
    id: usize,
}

/// Memory statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub current_usage_bytes: usize,
    pub max_usage_bytes: usize,
    pub allocation_count: usize,
    pub deallocation_count: usize,
    pub current_usage_mb: f64,
    pub max_usage_mb: f64,
    pub usage_percent: f64,
}

/// Compact buffer for storing embeddings efficiently
pub struct CompactEmbeddingBuffer {
    data: Vec<u8>,
    embedding_dim: usize,
    count: usize,
}

impl CompactEmbeddingBuffer {
    /// Create new buffer with specified embedding dimension
    pub fn new(embedding_dim: usize) -> Self {
        CompactEmbeddingBuffer {
            data: Vec::new(),
            embedding_dim,
            count: 0,
        }
    }

    /// Add an embedding (f32 values)
    pub fn add_embedding(&mut self, embedding: &[f32]) -> Result<(), String> {
        if embedding.len() != self.embedding_dim {
            return Err(format!(
                "Embedding dimension mismatch: expected {}, got {}",
                self.embedding_dim,
                embedding.len()
            ));
        }

        // Convert f32 to bytes
        for &value in embedding {
            self.data.extend_from_slice(&value.to_le_bytes());
        }

        self.count += 1;
        Ok(())
    }

    /// Get an embedding by index
    pub fn get_embedding(&self, index: usize) -> Option<Vec<f32>> {
        if index >= self.count {
            return None;
        }

        let start = index * self.embedding_dim * 4; // 4 bytes per f32
        let end = start + self.embedding_dim * 4;

        if end > self.data.len() {
            return None;
        }

        let mut embedding = Vec::with_capacity(self.embedding_dim);
        for i in (start..end).step_by(4) {
            let bytes = [
                self.data[i],
                self.data[i + 1],
                self.data[i + 2],
                self.data[i + 3],
            ];
            embedding.push(f32::from_le_bytes(bytes));
        }

        Some(embedding)
    }

    /// Get buffer size in bytes
    pub fn size_bytes(&self) -> usize {
        self.data.len()
    }

    /// Get number of embeddings stored
    pub fn count(&self) -> usize {
        self.count
    }

    /// Clear all embeddings
    pub fn clear(&mut self) {
        self.data.clear();
        self.count = 0;
    }
}

/// Quantize f32 embeddings to i8 for 4x compression
pub fn quantize_embedding(embedding: &[f32]) -> Vec<i8> {
    // Find min/max for normalization
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;

    for &val in embedding {
        if val < min {
            min = val;
        }
        if val > max {
            max = val;
        }
    }

    let range = max - min;
    if range < 1e-6 {
        return vec![0; embedding.len()];
    }

    // Quantize to i8 range [-127, 127]
    embedding
        .iter()
        .map(|&val| {
            let normalized = (val - min) / range; // 0..1
            let quantized = (normalized * 254.0 - 127.0).round();
            quantized.max(-127.0).min(127.0) as i8
        })
        .collect()
}

/// Dequantize i8 back to f32
pub fn dequantize_embedding(quantized: &[i8], min: f32, max: f32) -> Vec<f32> {
    let range = max - min;

    quantized
        .iter()
        .map(|&val| {
            let normalized = (val as f32 + 127.0) / 254.0; // 0..1
            normalized * range + min
        })
        .collect()
}

/// Efficient sparse vector storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SparseVector {
    indices: Vec<usize>,
    values: Vec<f32>,
    dim: usize,
}

impl SparseVector {
    /// Create from dense vector (only stores non-zero values)
    pub fn from_dense(dense: &[f32], threshold: f32) -> Self {
        let mut indices = Vec::new();
        let mut values = Vec::new();

        for (i, &val) in dense.iter().enumerate() {
            if val.abs() > threshold {
                indices.push(i);
                values.push(val);
            }
        }

        SparseVector {
            indices,
            values,
            dim: dense.len(),
        }
    }

    /// Convert to dense vector
    pub fn to_dense(&self) -> Vec<f32> {
        let mut dense = vec![0.0; self.dim];
        for (&idx, &val) in self.indices.iter().zip(self.values.iter()) {
            if idx < self.dim {
                dense[idx] = val;
            }
        }
        dense
    }

    /// Get compression ratio
    pub fn compression_ratio(&self) -> f32 {
        (self.indices.len() * 2) as f32 / self.dim as f32
    }

    /// Get memory size in bytes
    pub fn size_bytes(&self) -> usize {
        self.indices.len() * 4 + self.values.len() * 4
    }
}

/// Ring buffer for streaming data
pub struct RingBuffer<T> {
    data: Vec<Option<T>>,
    head: usize,
    tail: usize,
    capacity: usize,
    count: usize,
}

impl<T> RingBuffer<T> {
    /// Create new ring buffer with specified capacity
    pub fn new(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            data.push(None);
        }

        RingBuffer {
            data,
            head: 0,
            tail: 0,
            capacity,
            count: 0,
        }
    }

    /// Push item to buffer (overwrites oldest if full)
    pub fn push(&mut self, item: T) {
        self.data[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.capacity;

        if self.count < self.capacity {
            self.count += 1;
        } else {
            self.head = (self.head + 1) % self.capacity;
        }
    }

    /// Pop oldest item
    pub fn pop(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }

        let item = self.data[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        self.count -= 1;

        item
    }

    /// Get current count
    pub fn len(&self) -> usize {
        self.count
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Check if full
    pub fn is_full(&self) -> bool {
        self.count == self.capacity
    }

    /// Clear buffer
    pub fn clear(&mut self) {
        for item in &mut self.data {
            *item = None;
        }
        self.head = 0;
        self.tail = 0;
        self.count = 0;
    }
}

/// Memory-efficient string interning for repeated strings
pub struct StringInterner {
    strings: Vec<String>,
    indices: std::collections::HashMap<String, usize>,
}

impl StringInterner {
    pub fn new() -> Self {
        StringInterner {
            strings: Vec::new(),
            indices: std::collections::HashMap::new(),
        }
    }

    /// Intern a string and get its index
    pub fn intern(&mut self, s: &str) -> usize {
        if let Some(&idx) = self.indices.get(s) {
            return idx;
        }

        let idx = self.strings.len();
        self.strings.push(s.to_string());
        self.indices.insert(s.to_string(), idx);
        idx
    }

    /// Get string by index
    pub fn get(&self, idx: usize) -> Option<&str> {
        self.strings.get(idx).map(|s| s.as_str())
    }

    /// Get memory savings
    pub fn memory_saved(&self) -> usize {
        let total_string_bytes: usize = self.strings.iter().map(|s| s.len()).sum();
        let unique_count = self.strings.len();
        let total_references = self.indices.len();

        if total_references <= unique_count {
            return 0;
        }

        // Estimate: each reference saves (string_length - pointer_size) bytes
        let avg_string_len = if unique_count > 0 {
            total_string_bytes / unique_count
        } else {
            0
        };

        (total_references - unique_count) * avg_string_len.saturating_sub(8)
    }

    /// Clear all interned strings
    pub fn clear(&mut self) {
        self.strings.clear();
        self.indices.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_pool() {
        let pool = WasmMemoryPool::new(1); // 1 MB limit

        // Should succeed
        let token = pool.allocate(1024).unwrap();
        assert_eq!(pool.current_usage(), 1024);

        // Should fail (exceeds limit)
        let result = pool.allocate(2 * 1024 * 1024);
        assert!(result.is_err());

        // Deallocate
        pool.deallocate(token);
        assert_eq!(pool.current_usage(), 0);
    }

    #[test]
    fn test_compact_buffer() {
        let mut buffer = CompactEmbeddingBuffer::new(4);

        let emb1 = vec![1.0, 2.0, 3.0, 4.0];
        let emb2 = vec![5.0, 6.0, 7.0, 8.0];

        buffer.add_embedding(&emb1).unwrap();
        buffer.add_embedding(&emb2).unwrap();

        assert_eq!(buffer.count(), 2);
        assert_eq!(buffer.get_embedding(0).unwrap(), emb1);
        assert_eq!(buffer.get_embedding(1).unwrap(), emb2);
    }

    #[test]
    fn test_quantization() {
        let embedding = vec![0.5, -0.3, 0.8, -0.1, 0.0];
        let quantized = quantize_embedding(&embedding);

        assert_eq!(quantized.len(), embedding.len());

        // Check quantized values are in i8 range
        for &val in &quantized {
            assert!(val >= -127 && val <= 127);
        }
    }

    #[test]
    fn test_sparse_vector() {
        let dense = vec![0.0, 1.5, 0.0, 0.0, 2.3, 0.0, -1.2];
        let sparse = SparseVector::from_dense(&dense, 0.1);

        assert!(sparse.indices.len() < dense.len());
        assert_eq!(sparse.to_dense(), dense);
    }

    #[test]
    fn test_ring_buffer() {
        let mut buffer = RingBuffer::new(3);

        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        assert!(buffer.is_full());

        buffer.push(4); // Overwrites 1
        assert_eq!(buffer.pop(), Some(2));
        assert_eq!(buffer.pop(), Some(3));
        assert_eq!(buffer.pop(), Some(4));
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_string_interner() {
        let mut interner = StringInterner::new();

        let idx1 = interner.intern("hello");
        let idx2 = interner.intern("world");
        let idx3 = interner.intern("hello"); // Should return same index

        assert_eq!(idx1, idx3);
        assert_ne!(idx1, idx2);

        assert_eq!(interner.get(idx1), Some("hello"));
        assert_eq!(interner.get(idx2), Some("world"));
    }
}
