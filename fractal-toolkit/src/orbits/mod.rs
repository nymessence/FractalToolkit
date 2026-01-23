//! # Orbits Module
//!
//! This module contains orbit debugging functionality
//! for visualizing iteration paths in fractal generation.

use num_complex::Complex;
use std::collections::HashMap;

/// Orbit debugger for tracking iteration paths
pub struct OrbitDebugger {
    /// Store the sequence of points in the orbit
    pub points: Vec<Complex<f64>>,
    /// Maximum number of points to store
    pub max_points: usize,
    /// Whether orbit debugging is enabled
    pub enabled: bool,
}

impl OrbitDebugger {
    /// Create a new orbit debugger
    pub fn new(enabled: bool) -> Self {
        Self {
            points: Vec::new(),
            max_points: 1000, // Limit to prevent memory issues
            enabled,
        }
    }

    /// Add a point to the orbit
    pub fn add_point(&mut self, point: Complex<f64>) {
        if !self.enabled {
            return;
        }
        
        if self.points.len() >= self.max_points {
            // Remove oldest point if we've reached the limit
            self.points.remove(0);
        }
        self.points.push(point);
    }

    /// Clear all points from the orbit
    pub fn clear(&mut self) {
        self.points.clear();
    }

    /// Get statistics about the orbit
    pub fn stats(&self) -> OrbitStats {
        if self.points.is_empty() {
            return OrbitStats {
                count: 0,
                min_distance: 0.0,
                max_distance: 0.0,
                avg_distance: 0.0,
            };
        }

        let mut distances = Vec::new();
        let mut min_dist = f64::MAX;
        let mut max_dist = 0.0;

        for i in 1..self.points.len() {
            let dist = (self.points[i] - self.points[i-1]).norm();
            distances.push(dist);
            if dist < min_dist {
                min_dist = dist;
            }
            if dist > max_dist {
                max_dist = dist;
            }
        }

        let avg_distance = if !distances.is_empty() {
            distances.iter().sum::<f64>() / distances.len() as f64
        } else {
            0.0
        };

        OrbitStats {
            count: self.points.len(),
            min_distance: min_dist,
            max_distance: max_dist,
            avg_distance,
        }
    }
}

/// Statistics about an orbit
pub struct OrbitStats {
    /// Number of points in the orbit
    pub count: usize,
    /// Minimum distance between consecutive points
    pub min_distance: f64,
    /// Maximum distance between consecutive points
    pub max_distance: f64,
    /// Average distance between consecutive points
    pub avg_distance: f64,
}

/// Global orbit storage for multiple orbits
pub struct OrbitStorage {
    /// Collection of orbits keyed by identifier
    pub orbits: HashMap<String, OrbitDebugger>,
}

impl OrbitStorage {
    /// Create a new orbit storage
    pub fn new() -> Self {
        Self {
            orbits: HashMap::new(),
        }
    }

    /// Get or create an orbit debugger by ID
    pub fn get_or_create(&mut self, id: &str, enabled: bool) -> &mut OrbitDebugger {
        if !self.orbits.contains_key(id) {
            self.orbits.insert(id.to_string(), OrbitDebugger::new(enabled));
        }
        self.orbits.get_mut(id).unwrap()
    }

    /// Add a point to a specific orbit
    pub fn add_point(&mut self, id: &str, point: Complex<f64>) {
        if let Some(orbit) = self.orbits.get_mut(id) {
            orbit.add_point(point);
        }
    }

    /// Clear a specific orbit
    pub fn clear_orbit(&mut self, id: &str) {
        if let Some(orbit) = self.orbits.get_mut(id) {
            orbit.clear();
        }
    }

    /// Clear all orbits
    pub fn clear_all(&mut self) {
        for orbit in self.orbits.values_mut() {
            orbit.clear();
        }
    }
}