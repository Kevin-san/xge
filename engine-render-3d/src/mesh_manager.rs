//! Mesh resource manager with LRU cache and hot reload
//!
//! Provides mesh caching with least-recently-used eviction, access tracking,
//! and file-based hot reload support for development workflows.

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use engine_utils::Handle;

use crate::mesh::Mesh3D;

/// Handle to a cached mesh resource.
pub type MeshHandle = Handle<Mesh3D>;

/// Internal cache entry tracking a mesh and its access metadata.
#[derive(Debug)]
pub struct MeshEntry {
    /// The cached mesh data.
    pub mesh: Mesh3D,
    /// Optional source file path for hot reload.
    pub path: Option<String>,
    /// Frame index of the most recent access.
    pub last_used: u64,
    /// Total number of times this entry has been accessed.
    pub access_count: u64,
    /// Content hash of the source file, used for change detection.
    pub file_hash: u64,
    /// Whether the mesh has finished loading.
    pub loaded: bool,
}

impl MeshEntry {
    /// Create a new entry without a file path.
    pub fn new(mesh: Mesh3D) -> Self {
        Self {
            mesh,
            path: None,
            last_used: 0,
            access_count: 0,
            file_hash: 0,
            loaded: false,
        }
    }

    /// Create a new entry associated with a source file path.
    pub fn with_path(mesh: Mesh3D, path: &str) -> Self {
        Self {
            mesh,
            path: Some(path.to_string()),
            last_used: 0,
            access_count: 0,
            file_hash: 0,
            loaded: false,
        }
    }

    /// Mark this entry as accessed at the given frame, updating LRU metadata.
    pub fn touch(&mut self, frame: u64) {
        self.last_used = frame;
        self.access_count += 1;
    }

    /// Mark this entry as fully loaded.
    pub fn mark_loaded(&mut self) {
        self.loaded = true;
    }
}

/// LRU cache manager for mesh resources.
#[derive(Debug)]
pub struct MeshManager {
    entries: BTreeMap<u32, MeshEntry>,
    next_id: u32,
    max_cache_size: usize,
    current_frame: u64,
    cache_hits: u64,
    cache_misses: u64,
}

impl MeshManager {
    /// Create a manager with the default cache capacity of 256 meshes.
    pub fn new() -> Self {
        Self::with_capacity(256)
    }

    /// Create a manager with a custom cache capacity.
    pub fn with_capacity(max_cache_size: usize) -> Self {
        Self {
            entries: BTreeMap::new(),
            next_id: 0,
            max_cache_size,
            current_frame: 0,
            cache_hits: 0,
            cache_misses: 0,
        }
    }

    /// Load a mesh into the cache, returning a handle.
    pub fn load(&mut self, mesh: Mesh3D) -> MeshHandle {
        let id = self.next_id;
        self.next_id += 1;
        let mut entry = MeshEntry::new(mesh);
        entry.last_used = self.current_frame;
        self.entries.insert(id, entry);
        self.evict_if_needed();
        Handle::new(id, 0)
    }

    /// Load a mesh with an associated file path for hot reload support.
    pub fn load_with_path(&mut self, mesh: Mesh3D, path: &str) -> MeshHandle {
        let id = self.next_id;
        self.next_id += 1;
        let mut entry = MeshEntry::with_path(mesh, path);
        entry.file_hash = compute_mesh_hash(&entry.mesh);
        entry.last_used = self.current_frame;
        self.entries.insert(id, entry);
        self.evict_if_needed();
        Handle::new(id, 0)
    }

    /// Get a mesh by handle, updating LRU metadata. Records a cache hit or miss.
    pub fn get(&mut self, handle: MeshHandle) -> Option<&Mesh3D> {
        let index = handle.index();
        if !self.entries.contains_key(&index) {
            self.cache_misses += 1;
            return None;
        }
        self.cache_hits += 1;
        let frame = self.current_frame;
        let entry = self.entries.get_mut(&index).unwrap();
        entry.touch(frame);
        Some(&entry.mesh)
    }

    /// Get a mutable mesh by handle, updating LRU metadata. Records a cache hit or miss.
    pub fn get_mut(&mut self, handle: MeshHandle) -> Option<&mut Mesh3D> {
        let index = handle.index();
        if !self.entries.contains_key(&index) {
            self.cache_misses += 1;
            return None;
        }
        self.cache_hits += 1;
        let frame = self.current_frame;
        let entry = self.entries.get_mut(&index).unwrap();
        entry.touch(frame);
        Some(&mut entry.mesh)
    }

    /// Remove a mesh from the cache. Returns true if the mesh was present.
    pub fn remove(&mut self, handle: MeshHandle) -> bool {
        self.entries.remove(&handle.index()).is_some()
    }

    /// Evict the least recently used entry, returning its id if any was evicted.
    pub fn evict_lru(&mut self) -> Option<u32> {
        let lru_id = self
            .entries
            .iter()
            .min_by_key(|(_, e)| e.last_used)
            .map(|(id, _)| *id)?;
        self.entries.remove(&lru_id);
        Some(lru_id)
    }

    /// Evict entries while the cache exceeds its capacity.
    pub fn evict_if_needed(&mut self) {
        while self.entries.len() > self.max_cache_size {
            if self.evict_lru().is_none() {
                break;
            }
        }
    }

    /// Current number of cached meshes.
    pub fn cache_size(&self) -> usize {
        self.entries.len()
    }

    /// Total number of cache hits.
    pub fn cache_hits(&self) -> u64 {
        self.cache_hits
    }

    /// Total number of cache misses.
    pub fn cache_misses(&self) -> u64 {
        self.cache_misses
    }

    /// Hit rate as hits / (hits + misses). Returns 0.0 when there are no accesses.
    pub fn cache_hit_rate(&self) -> f32 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f32 / total as f32
        }
    }

    /// Advance the internal frame counter, used for LRU ordering.
    pub fn advance_frame(&mut self) {
        self.current_frame += 1;
    }

    /// Returns true if the given handle refers to a cached mesh.
    pub fn contains(&self, handle: MeshHandle) -> bool {
        self.entries.contains_key(&handle.index())
    }

    /// Clear all cached meshes.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl Default for MeshManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Tracks file hashes to detect changes for hot reload.
#[derive(Debug)]
pub struct HotReloadTracker {
    file_hashes: BTreeMap<String, u64>,
    changed_paths: Vec<String>,
}

impl HotReloadTracker {
    /// Create an empty tracker.
    pub fn new() -> Self {
        Self {
            file_hashes: BTreeMap::new(),
            changed_paths: Vec::new(),
        }
    }

    /// Register a file's content hash.
    pub fn register(&mut self, path: &str, hash: u64) {
        self.file_hashes.insert(path.to_string(), hash);
    }

    /// Check whether a file's hash differs from the stored value.
    ///
    /// Returns true if the file has changed (or records a change). Unknown paths
    /// return false. Changed paths are recorded until [`clear_changed`] is called.
    pub fn check_changed(&mut self, path: &str, current_hash: u64) -> bool {
        match self.file_hashes.get(path) {
            Some(&stored) => {
                if stored != current_hash {
                    self.changed_paths.push(path.to_string());
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    /// Update the stored hash for a file.
    pub fn update_hash(&mut self, path: &str, hash: u64) {
        self.file_hashes.insert(path.to_string(), hash);
    }

    /// Paths that have been detected as changed since the last clear.
    pub fn changed_paths(&self) -> &[String] {
        &self.changed_paths
    }

    /// Clear the recorded list of changed paths.
    pub fn clear_changed(&mut self) {
        self.changed_paths.clear();
    }
}

impl Default for HotReloadTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Combines a [`MeshManager`] with a [`HotReloadTracker`] for path-based hot reload.
#[derive(Debug)]
pub struct MeshHotReloader {
    manager: MeshManager,
    tracker: HotReloadTracker,
    path_to_handle: BTreeMap<String, MeshHandle>,
}

impl MeshHotReloader {
    /// Create a new hot reloader with a default-capacity manager.
    pub fn new() -> Self {
        Self {
            manager: MeshManager::new(),
            tracker: HotReloadTracker::new(),
            path_to_handle: BTreeMap::new(),
        }
    }

    /// Load a mesh, tracking its path and content hash for later reload checks.
    pub fn load_mesh(&mut self, mesh: Mesh3D, path: &str) -> MeshHandle {
        let hash = compute_mesh_hash(&mesh);
        let handle = self.manager.load_with_path(mesh, path);
        self.tracker.register(path, hash);
        self.path_to_handle.insert(path.to_string(), handle.clone());
        handle
    }

    /// Check whether the file at `path` changed and, if so, reload `new_mesh`.
    ///
    /// Returns true if a change was detected (and the mesh reloaded).
    pub fn check_and_reload(&mut self, path: &str, new_mesh: Mesh3D) -> bool {
        let new_hash = compute_mesh_hash(&new_mesh);
        if !self.tracker.check_changed(path, new_hash) {
            return false;
        }
        if let Some(handle) = self.path_to_handle.get(path).cloned() {
            if let Some(mesh) = self.manager.get_mut(handle) {
                *mesh = new_mesh;
            }
        }
        self.tracker.update_hash(path, new_hash);
        true
    }

    /// Force a reload of `new_mesh` at `path` regardless of hash comparison.
    ///
    /// Returns true if the path was known and reloaded.
    pub fn force_reload(&mut self, path: &str, new_mesh: Mesh3D) -> bool {
        let new_hash = compute_mesh_hash(&new_mesh);
        let handle = match self.path_to_handle.get(path) {
            Some(h) => h.clone(),
            None => return false,
        };
        if let Some(mesh) = self.manager.get_mut(handle) {
            *mesh = new_mesh;
        }
        self.tracker.update_hash(path, new_hash);
        true
    }

    /// Access the underlying mesh manager.
    pub fn manager(&self) -> &MeshManager {
        &self.manager
    }

    /// Mutably access the underlying mesh manager.
    pub fn manager_mut(&mut self) -> &mut MeshManager {
        &mut self.manager
    }
}

impl Default for MeshHotReloader {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute a lightweight FNV-1a content hash over a mesh's vertex data.
fn compute_mesh_hash(mesh: &Mesh3D) -> u64 {
    const FNV_OFFSET: u64 = 14695981039346656037;
    const FNV_PRIME: u64 = 1099511628211;

    let mut hash = FNV_OFFSET;
    hash ^= mesh.vertex_count() as u64;
    hash = hash.wrapping_mul(FNV_PRIME);
    hash ^= mesh.triangle_count() as u64;
    hash = hash.wrapping_mul(FNV_PRIME);
    for v in mesh.vertices().iter().take(16) {
        hash ^= v.position.x.to_bits() as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
        hash ^= v.position.y.to_bits() as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
        hash ^= v.position.z.to_bits() as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::Mesh3D;

    // --- MeshEntry tests ---

    #[test]
    fn test_mesh_entry_new() {
        let entry = MeshEntry::new(Mesh3D::cube(1.0));
        assert!(entry.path.is_none());
        assert_eq!(entry.last_used, 0);
        assert_eq!(entry.access_count, 0);
        assert_eq!(entry.file_hash, 0);
        assert!(!entry.loaded);
    }

    #[test]
    fn test_mesh_entry_with_path() {
        let entry = MeshEntry::with_path(Mesh3D::cube(1.0), "assets/cube.mesh");
        assert_eq!(entry.path, Some("assets/cube.mesh".to_string()));
        assert_eq!(entry.last_used, 0);
        assert_eq!(entry.access_count, 0);
    }

    #[test]
    fn test_mesh_entry_touch() {
        let mut entry = MeshEntry::new(Mesh3D::cube(1.0));
        entry.touch(42);
        assert_eq!(entry.last_used, 42);
        assert_eq!(entry.access_count, 1);
        entry.touch(50);
        assert_eq!(entry.last_used, 50);
        assert_eq!(entry.access_count, 2);
    }

    #[test]
    fn test_mesh_entry_mark_loaded() {
        let mut entry = MeshEntry::new(Mesh3D::cube(1.0));
        assert!(!entry.loaded);
        entry.mark_loaded();
        assert!(entry.loaded);
    }

    // --- MeshManager tests ---

    #[test]
    fn test_mesh_manager_new() {
        let manager = MeshManager::new();
        assert_eq!(manager.cache_size(), 0);
        assert_eq!(manager.cache_hits(), 0);
        assert_eq!(manager.cache_misses(), 0);
        assert_eq!(manager.max_cache_size, 256);
    }

    #[test]
    fn test_mesh_manager_with_capacity() {
        let manager = MeshManager::with_capacity(64);
        assert_eq!(manager.max_cache_size, 64);
        assert_eq!(manager.cache_size(), 0);
    }

    #[test]
    fn test_mesh_manager_load() {
        let mut manager = MeshManager::new();
        let handle = manager.load(Mesh3D::cube(1.0));
        assert!(!handle.is_null());
        assert_eq!(manager.cache_size(), 1);
    }

    #[test]
    fn test_mesh_manager_load_increments_id() {
        let mut manager = MeshManager::new();
        let h1 = manager.load(Mesh3D::cube(1.0));
        let h2 = manager.load(Mesh3D::cube(2.0));
        assert_ne!(h1.index(), h2.index());
        assert_eq!(manager.cache_size(), 2);
    }

    #[test]
    fn test_mesh_manager_get() {
        let mut manager = MeshManager::new();
        let handle = manager.load(Mesh3D::cube(1.0));
        let mesh = manager.get(handle);
        assert!(mesh.is_some());
        assert_eq!(mesh.unwrap().vertex_count(), 24);
        assert_eq!(manager.cache_hits(), 1);
    }

    #[test]
    fn test_mesh_manager_get_miss() {
        let mut manager = MeshManager::new();
        let result = manager.get(MeshHandle::null());
        assert!(result.is_none());
        assert_eq!(manager.cache_misses(), 1);
        assert_eq!(manager.cache_hits(), 0);
    }

    #[test]
    fn test_mesh_manager_get_mut() {
        let mut manager = MeshManager::new();
        let handle = manager.load(Mesh3D::cube(1.0));
        {
            let mesh = manager.get_mut(handle);
            assert!(mesh.is_some());
            if let Some(m) = mesh {
                m.recalculate_aabb();
            }
        }
        assert_eq!(manager.cache_hits(), 1);
    }

    #[test]
    fn test_mesh_manager_get_mut_miss() {
        let mut manager = MeshManager::new();
        assert!(manager.get_mut(MeshHandle::null()).is_none());
        assert_eq!(manager.cache_misses(), 1);
    }

    #[test]
    fn test_mesh_manager_remove() {
        let mut manager = MeshManager::new();
        let handle = manager.load(Mesh3D::cube(1.0));
        assert!(manager.remove(handle.clone()));
        assert_eq!(manager.cache_size(), 0);
        assert!(!manager.contains(handle));
    }

    #[test]
    fn test_mesh_manager_remove_missing() {
        let mut manager = MeshManager::new();
        assert!(!manager.remove(MeshHandle::null()));
    }

    #[test]
    fn test_mesh_manager_evict_lru() {
        let mut manager = MeshManager::new();
        let h1 = manager.load(Mesh3D::cube(1.0));
        manager.advance_frame();
        let _h2 = manager.load(Mesh3D::cube(2.0));
        // h1 has last_used=0, h2 has last_used=1
        let evicted = manager.evict_lru();
        assert_eq!(evicted, Some(h1.index()));
        assert!(!manager.contains(h1));
        assert_eq!(manager.cache_size(), 1);
    }

    #[test]
    fn test_mesh_manager_evict_lru_empty() {
        let mut manager = MeshManager::new();
        assert_eq!(manager.evict_lru(), None);
    }

    #[test]
    fn test_mesh_manager_evict_if_needed() {
        let mut manager = MeshManager::with_capacity(2);
        manager.load(Mesh3D::cube(1.0));
        manager.load(Mesh3D::cube(2.0));
        manager.load(Mesh3D::cube(3.0));
        // load already triggers evict_if_needed, so size should be 2
        assert_eq!(manager.cache_size(), 2);
        // explicit call is a no-op when already at capacity
        manager.evict_if_needed();
        assert_eq!(manager.cache_size(), 2);
    }

    #[test]
    fn test_mesh_manager_evict_if_needed_under_capacity() {
        let mut manager = MeshManager::with_capacity(10);
        manager.load(Mesh3D::cube(1.0));
        manager.evict_if_needed();
        assert_eq!(manager.cache_size(), 1);
    }

    #[test]
    fn test_mesh_manager_cache_size() {
        let mut manager = MeshManager::new();
        assert_eq!(manager.cache_size(), 0);
        manager.load(Mesh3D::cube(1.0));
        assert_eq!(manager.cache_size(), 1);
        manager.load(Mesh3D::cube(2.0));
        assert_eq!(manager.cache_size(), 2);
    }

    #[test]
    fn test_mesh_manager_cache_hits() {
        let mut manager = MeshManager::new();
        let h = manager.load(Mesh3D::cube(1.0));
        let _ = manager.get(h.clone());
        let _ = manager.get(h);
        assert_eq!(manager.cache_hits(), 2);
    }

    #[test]
    fn test_mesh_manager_cache_misses() {
        let mut manager = MeshManager::new();
        let _ = manager.get(MeshHandle::null());
        let _ = manager.get(MeshHandle::null());
        assert_eq!(manager.cache_misses(), 2);
    }

    #[test]
    fn test_mesh_manager_cache_hit_rate() {
        let mut manager = MeshManager::new();
        let h = manager.load(Mesh3D::cube(1.0));
        let _ = manager.get(h); // hit
        let _ = manager.get(MeshHandle::null()); // miss
        let _ = manager.get(MeshHandle::null()); // miss
        let rate = manager.cache_hit_rate();
        assert!((rate - 1.0 / 3.0).abs() < 1e-5);
    }

    #[test]
    fn test_mesh_manager_cache_hit_rate_no_access() {
        let manager = MeshManager::new();
        assert_eq!(manager.cache_hit_rate(), 0.0);
    }

    #[test]
    fn test_mesh_manager_cache_hit_rate_all_hits() {
        let mut manager = MeshManager::new();
        let h = manager.load(Mesh3D::cube(1.0));
        let _ = manager.get(h.clone());
        let _ = manager.get(h);
        assert!((manager.cache_hit_rate() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_mesh_manager_advance_frame() {
        let mut manager = MeshManager::new();
        assert_eq!(manager.current_frame, 0);
        manager.advance_frame();
        assert_eq!(manager.current_frame, 1);
        manager.advance_frame();
        assert_eq!(manager.current_frame, 2);
    }

    #[test]
    fn test_mesh_manager_contains() {
        let mut manager = MeshManager::new();
        let h = manager.load(Mesh3D::cube(1.0));
        assert!(manager.contains(h));
        assert!(!manager.contains(MeshHandle::null()));
    }

    #[test]
    fn test_mesh_manager_clear() {
        let mut manager = MeshManager::new();
        manager.load(Mesh3D::cube(1.0));
        manager.load(Mesh3D::cube(2.0));
        manager.clear();
        assert_eq!(manager.cache_size(), 0);
    }

    #[test]
    fn test_mesh_manager_lru_eviction_order() {
        let mut manager = MeshManager::with_capacity(2);
        let h1 = manager.load(Mesh3D::cube(1.0));
        let h2 = manager.load(Mesh3D::cube(2.0));
        let _h3 = manager.load(Mesh3D::cube(3.0));
        // h1 should be evicted (oldest, last_used=0)
        assert!(!manager.contains(h1));
        assert!(manager.contains(h2));
        assert_eq!(manager.cache_size(), 2);
    }

    #[test]
    fn test_mesh_manager_lru_with_get() {
        let mut manager = MeshManager::with_capacity(2);
        let h1 = manager.load(Mesh3D::cube(1.0));
        let h2 = manager.load(Mesh3D::cube(2.0));
        manager.advance_frame();
        let _ = manager.get(h1.clone()); // h1 now last_used=1, h2 last_used=0
        let _h3 = manager.load(Mesh3D::cube(3.0));
        // h2 should be evicted
        assert!(manager.contains(h1));
        assert!(!manager.contains(h2));
    }

    #[test]
    fn test_mesh_manager_load_with_path() {
        let mut manager = MeshManager::new();
        let handle = manager.load_with_path(Mesh3D::cube(1.0), "assets/cube.mesh");
        assert!(!handle.is_null());
        let entry = manager.entries.get(&handle.index()).unwrap();
        assert_eq!(entry.path, Some("assets/cube.mesh".to_string()));
        assert_ne!(entry.file_hash, 0);
    }

    #[test]
    fn test_mesh_manager_get_updates_lru() {
        let mut manager = MeshManager::new();
        let h = manager.load(Mesh3D::cube(1.0));
        manager.advance_frame();
        manager.advance_frame();
        let _ = manager.get(h.clone());
        let entry = manager.entries.get(&h.index()).unwrap();
        assert_eq!(entry.last_used, 2);
        assert_eq!(entry.access_count, 1);
    }

    #[test]
    fn test_mesh_manager_default() {
        let manager = MeshManager::default();
        assert_eq!(manager.max_cache_size, 256);
        assert_eq!(manager.cache_size(), 0);
    }

    // --- HotReloadTracker tests ---

    #[test]
    fn test_hot_reload_tracker_register_and_check_unchanged() {
        let mut tracker = HotReloadTracker::new();
        tracker.register("cube.mesh", 12345);
        assert!(!tracker.check_changed("cube.mesh", 12345));
        assert!(tracker.changed_paths().is_empty());
    }

    #[test]
    fn test_hot_reload_tracker_check_changed() {
        let mut tracker = HotReloadTracker::new();
        tracker.register("cube.mesh", 12345);
        assert!(tracker.check_changed("cube.mesh", 99999));
    }

    #[test]
    fn test_hot_reload_tracker_check_unknown_path() {
        let mut tracker = HotReloadTracker::new();
        assert!(!tracker.check_changed("unknown.mesh", 1));
    }

    #[test]
    fn test_hot_reload_tracker_update_hash() {
        let mut tracker = HotReloadTracker::new();
        tracker.register("cube.mesh", 100);
        tracker.update_hash("cube.mesh", 200);
        assert!(!tracker.check_changed("cube.mesh", 200));
        assert!(tracker.check_changed("cube.mesh", 100));
    }

    #[test]
    fn test_hot_reload_tracker_changed_paths() {
        let mut tracker = HotReloadTracker::new();
        tracker.register("a.mesh", 1);
        tracker.register("b.mesh", 2);
        tracker.check_changed("a.mesh", 10);
        tracker.check_changed("b.mesh", 2); // not changed
        let changed = tracker.changed_paths();
        assert_eq!(changed.len(), 1);
        assert_eq!(changed[0], "a.mesh");
    }

    #[test]
    fn test_hot_reload_tracker_clear_changed() {
        let mut tracker = HotReloadTracker::new();
        tracker.register("a.mesh", 1);
        tracker.check_changed("a.mesh", 10);
        assert_eq!(tracker.changed_paths().len(), 1);
        tracker.clear_changed();
        assert_eq!(tracker.changed_paths().len(), 0);
    }

    #[test]
    fn test_hot_reload_tracker_default() {
        let tracker = HotReloadTracker::default();
        assert!(tracker.changed_paths().is_empty());
    }

    // --- MeshHotReloader tests ---

    #[test]
    fn test_mesh_hot_reloader_new() {
        let reloader = MeshHotReloader::new();
        assert_eq!(reloader.manager().cache_size(), 0);
    }

    #[test]
    fn test_mesh_hot_reloader_load_mesh() {
        let mut reloader = MeshHotReloader::new();
        let handle = reloader.load_mesh(Mesh3D::cube(1.0), "cube.mesh");
        assert!(!handle.is_null());
        assert_eq!(reloader.manager().cache_size(), 1);
        assert!(reloader.manager().contains(handle));
    }

    #[test]
    fn test_mesh_hot_reloader_check_and_reload_unchanged() {
        let mut reloader = MeshHotReloader::new();
        let mesh = Mesh3D::cube(1.0);
        reloader.load_mesh(mesh.clone(), "cube.mesh");
        let changed = reloader.check_and_reload("cube.mesh", mesh);
        assert!(!changed);
    }

    #[test]
    fn test_mesh_hot_reloader_check_and_reload_changed() {
        let mut reloader = MeshHotReloader::new();
        reloader.load_mesh(Mesh3D::cube(1.0), "cube.mesh");
        let changed = reloader.check_and_reload("cube.mesh", Mesh3D::sphere(1.0, 16, 8));
        assert!(changed);
    }

    #[test]
    fn test_mesh_hot_reloader_check_and_reload_replaces_mesh() {
        let mut reloader = MeshHotReloader::new();
        let handle = reloader.load_mesh(Mesh3D::cube(1.0), "cube.mesh");
        // cube has 24 vertices
        assert_eq!(
            reloader.manager_mut().get(handle.clone()).unwrap().vertex_count(),
            24
        );
        reloader.check_and_reload("cube.mesh", Mesh3D::sphere(1.0, 16, 8));
        // sphere(16, 8) has (16+1)*(8+1) = 153 vertices
        assert_eq!(
            reloader.manager_mut().get(handle).unwrap().vertex_count(),
            153
        );
    }

    #[test]
    fn test_mesh_hot_reloader_check_and_reload_unknown_path() {
        let mut reloader = MeshHotReloader::new();
        let changed = reloader.check_and_reload("unknown.mesh", Mesh3D::cube(1.0));
        assert!(!changed);
    }

    #[test]
    fn test_mesh_hot_reloader_force_reload() {
        let mut reloader = MeshHotReloader::new();
        let handle = reloader.load_mesh(Mesh3D::cube(1.0), "cube.mesh");
        let result = reloader.force_reload("cube.mesh", Mesh3D::sphere(1.0, 8, 4));
        assert!(result);
        // sphere(8, 4) has (8+1)*(4+1) = 45 vertices
        assert_eq!(
            reloader.manager_mut().get(handle).unwrap().vertex_count(),
            45
        );
    }

    #[test]
    fn test_mesh_hot_reloader_force_reload_unknown_path() {
        let mut reloader = MeshHotReloader::new();
        let result = reloader.force_reload("unknown.mesh", Mesh3D::cube(1.0));
        assert!(!result);
    }

    #[test]
    fn test_mesh_hot_reloader_manager_access() {
        let mut reloader = MeshHotReloader::new();
        reloader.load_mesh(Mesh3D::cube(1.0), "cube.mesh");
        {
            let m = reloader.manager();
            assert_eq!(m.cache_size(), 1);
        }
        let handle = reloader.manager_mut().load(Mesh3D::cube(2.0));
        assert!(reloader.manager().contains(handle));
    }

    #[test]
    fn test_mesh_hot_reloader_force_reload_updates_hash() {
        let mut reloader = MeshHotReloader::new();
        reloader.load_mesh(Mesh3D::cube(1.0), "cube.mesh");
        let new_mesh = Mesh3D::sphere(1.0, 16, 8);
        reloader.force_reload("cube.mesh", new_mesh.clone());
        // after force reload, hash updated; same mesh should report no change
        let changed = reloader.check_and_reload("cube.mesh", new_mesh);
        assert!(!changed);
    }

    #[test]
    fn test_mesh_hot_reloader_default() {
        let reloader = MeshHotReloader::default();
        assert_eq!(reloader.manager().cache_size(), 0);
    }
}
