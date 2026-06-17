//! Network statistics tracking.

use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// Network statistics structure
#[derive(Debug, Clone, Copy, Default)]
pub struct NetStats {
    /// Total bytes received
    pub bytes_in: u64,
    /// Total bytes sent
    pub bytes_out: u64,
    /// Total messages received
    pub msg_in: u64,
    /// Total messages sent
    pub msg_out: u64,
    /// Current round-trip time in milliseconds
    pub rtt_ms: u32,
}

impl NetStats {
    /// Create new empty statistics
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate bytes per second
    pub fn bytes_per_second(&self, duration_secs: f64) -> (f64, f64) {
        let in_rate = self.bytes_in as f64 / duration_secs;
        let out_rate = self.bytes_out as f64 / duration_secs;
        (in_rate, out_rate)
    }

    /// Calculate messages per second
    pub fn messages_per_second(&self, duration_secs: f64) -> (f64, f64) {
        let in_rate = self.msg_in as f64 / duration_secs;
        let out_rate = self.msg_out as f64 / duration_secs;
        (in_rate, out_rate)
    }

    /// Get total bytes transferred
    pub fn total_bytes(&self) -> u64 {
        self.bytes_in + self.bytes_out
    }

    /// Get total messages transferred
    pub fn total_messages(&self) -> u64 {
        self.msg_in + self.msg_out
    }
}

/// Atomic network statistics for concurrent access
pub struct AtomicNetStats {
    bytes_in: AtomicU64,
    bytes_out: AtomicU64,
    msg_in: AtomicU64,
    msg_out: AtomicU64,
    rtt_ms: AtomicU32,
}

impl AtomicNetStats {
    /// Create new atomic statistics
    pub fn new() -> Self {
        Self {
            bytes_in: AtomicU64::new(0),
            bytes_out: AtomicU64::new(0),
            msg_in: AtomicU64::new(0),
            msg_out: AtomicU64::new(0),
            rtt_ms: AtomicU32::new(0),
        }
    }

    /// Record incoming bytes
    pub fn record_in(&self, bytes: u64) {
        self.bytes_in.fetch_add(bytes, Ordering::Relaxed);
        self.msg_in.fetch_add(1, Ordering::Relaxed);
    }

    /// Record outgoing bytes
    pub fn record_out(&self, bytes: u64) {
        self.bytes_out.fetch_add(bytes, Ordering::Relaxed);
        self.msg_out.fetch_add(1, Ordering::Relaxed);
    }

    /// Update RTT
    pub fn update_rtt(&self, rtt_ms: u32) {
        self.rtt_ms.store(rtt_ms, Ordering::Relaxed);
    }

    /// Get snapshot of current statistics
    pub fn snapshot(&self) -> NetStats {
        NetStats {
            bytes_in: self.bytes_in.load(Ordering::Relaxed),
            bytes_out: self.bytes_out.load(Ordering::Relaxed),
            msg_in: self.msg_in.load(Ordering::Relaxed),
            msg_out: self.msg_out.load(Ordering::Relaxed),
            rtt_ms: self.rtt_ms.load(Ordering::Relaxed),
        }
    }

    /// Reset all statistics
    pub fn reset(&self) {
        self.bytes_in.store(0, Ordering::Relaxed);
        self.bytes_out.store(0, Ordering::Relaxed);
        self.msg_in.store(0, Ordering::Relaxed);
        self.msg_out.store(0, Ordering::Relaxed);
        self.rtt_ms.store(0, Ordering::Relaxed);
    }
}

impl Default for AtomicNetStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_net_stats() {
        let stats = NetStats {
            bytes_in: 1000,
            bytes_out: 500,
            msg_in: 10,
            msg_out: 5,
            rtt_ms: 50,
        };

        assert_eq!(stats.total_bytes(), 1500);
        assert_eq!(stats.total_messages(), 15);

        let (in_rate, out_rate) = stats.bytes_per_second(1.0);
        assert_eq!(in_rate, 1000.0);
        assert_eq!(out_rate, 500.0);
    }

    #[test]
    fn test_atomic_net_stats() {
        let stats = AtomicNetStats::new();

        stats.record_in(100);
        stats.record_out(200);
        stats.update_rtt(30);

        let snapshot = stats.snapshot();
        assert_eq!(snapshot.bytes_in, 100);
        assert_eq!(snapshot.bytes_out, 200);
        assert_eq!(snapshot.msg_in, 1);
        assert_eq!(snapshot.msg_out, 1);
        assert_eq!(snapshot.rtt_ms, 30);

        stats.reset();
        let snapshot = stats.snapshot();
        assert_eq!(snapshot.bytes_in, 0);
    }
}
