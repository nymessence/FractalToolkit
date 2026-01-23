use std::time::Instant;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Utility functions for the fractal toolkit
pub struct Utils;

impl Utils {
    /// Measure execution time of a function
    pub fn measure_time<T>(f: impl FnOnce() -> T) -> (T, std::time::Duration) {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        (result, duration)
    }

    /// Clamp a value between min and max
    pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T 
    where T: Copy 
    {
        if value < min { min } 
        else if value > max { max } 
        else { value }
    }

    /// Linear interpolation between two values
    pub fn lerp(start: f64, end: f64, t: f64) -> f64 {
        start + t * (end - start)
    }

    /// Map a value from one range to another
    pub fn map_range(value: f64, from_start: f64, from_end: f64, to_start: f64, to_end: f64) -> f64 {
        let t = (value - from_start) / (from_end - from_start);
        Self::lerp(to_start, to_end, t)
    }

    /// Check if a number is approximately equal to another
    pub fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() < epsilon
    }
}

/// Thread-safe counter for tracking progress
pub struct Counter {
    value: AtomicUsize,
}

impl Counter {
    /// Create a new counter with initial value 0
    pub fn new() -> Self {
        Self {
            value: AtomicUsize::new(0),
        }
    }

    /// Increment the counter and return the new value
    pub fn inc(&self) -> usize {
        self.value.fetch_add(1, Ordering::SeqCst) + 1
    }

    /// Get the current value of the counter
    pub fn get(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }

    /// Reset the counter to 0
    pub fn reset(&self) {
        self.value.store(0, Ordering::SeqCst);
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

/// Progress tracker for long-running operations
pub struct ProgressTracker {
    total: usize,
    current: Counter,
}

impl ProgressTracker {
    /// Create a new progress tracker
    pub fn new(total: usize) -> Self {
        Self {
            total,
            current: Counter::new(),
        }
    }

    /// Increment progress and return percentage
    pub fn inc(&self) -> f64 {
        let current = self.current.inc();
        (current as f64 / self.total as f64) * 100.0
    }

    /// Get current progress as percentage
    pub fn progress(&self) -> f64 {
        (self.current.get() as f64 / self.total as f64) * 100.0
    }

    /// Check if the operation is complete
    pub fn is_complete(&self) -> bool {
        self.current.get() >= self.total
    }
}