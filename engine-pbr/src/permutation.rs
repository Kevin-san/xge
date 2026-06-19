//! Shader Permutation - Shader variant management with hashing and caching
//!
//! Provides a hash-based key system for identifying shader permutations
//! based on material flags, render state, and feature toggles.

use std::collections::HashMap;

use crate::PbrMaterialFlags;

/// Shader permutation key that uniquely identifies a shader variant
///
/// Combines material flags, alpha mode, and other feature toggles
/// into a single hashable key for cache lookup.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ShaderPermutationKey {
    /// Material feature flags
    pub flags: u32,
    /// Alpha mode (0 = Opaque, 1 = Mask, 2 = Blend)
    pub alpha_mode: u8,
    /// Number of lights supported
    pub light_count: u8,
    /// Whether shadow mapping is enabled
    pub shadows_enabled: bool,
    /// Whether IBL is enabled
    pub ibl_enabled: bool,
    /// Tone mapping mode (0 = None, 1 = ACES, 2 = Reinhard, 3 = Filmic)
    pub tonemap_mode: u8,
    /// Number of cascades for shadow mapping (0 = no CSM)
    pub cascade_count: u8,
}

impl Default for ShaderPermutationKey {
    fn default() -> Self {
        Self {
            flags: 0,
            alpha_mode: 0,
            light_count: 1,
            shadows_enabled: false,
            ibl_enabled: false,
            tonemap_mode: 1, // ACES by default
            cascade_count: 0,
        }
    }
}

impl ShaderPermutationKey {
    /// Create a new shader permutation key
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a key from material flags
    pub fn from_flags(flags: PbrMaterialFlags) -> Self {
        Self {
            flags: flags.bits(),
            ..Self::default()
        }
    }

    /// Set the material flags
    pub fn with_flags(mut self, flags: PbrMaterialFlags) -> Self {
        self.flags = flags.bits();
        self
    }

    /// Set the alpha mode
    pub fn with_alpha_mode(mut self, alpha_mode: u8) -> Self {
        self.alpha_mode = alpha_mode;
        self
    }

    /// Set the light count
    pub fn with_light_count(mut self, count: u8) -> Self {
        self.light_count = count;
        self
    }

    /// Enable or disable shadows
    pub fn with_shadows(mut self, enabled: bool) -> Self {
        self.shadows_enabled = enabled;
        self
    }

    /// Enable or disable IBL
    pub fn with_ibl(mut self, enabled: bool) -> Self {
        self.ibl_enabled = enabled;
        self
    }

    /// Set the tone mapping mode
    pub fn with_tonemap(mut self, mode: u8) -> Self {
        self.tonemap_mode = mode;
        self
    }

    /// Set the cascade count for CSM
    pub fn with_cascades(mut self, count: u8) -> Self {
        self.cascade_count = count;
        self
    }

    /// Compute a 64-bit hash of this permutation key
    ///
    /// Uses a simple FNV-1a inspired hash for fast cache lookups.
    pub fn hash(&self) -> u64 {
        // FNV-1a 64-bit
        const FNV_OFFSET: u64 = 14695981039346656037;
        const FNV_PRIME: u64 = 1099511628211;

        let mut hash = FNV_OFFSET;

        // Hash flags
        hash ^= self.flags as u64;
        hash = hash.wrapping_mul(FNV_PRIME);

        // Hash alpha_mode
        hash ^= self.alpha_mode as u64;
        hash = hash.wrapping_mul(FNV_PRIME);

        // Hash light_count
        hash ^= self.light_count as u64;
        hash = hash.wrapping_mul(FNV_PRIME);

        // Hash shadows_enabled
        hash ^= self.shadows_enabled as u64;
        hash = hash.wrapping_mul(FNV_PRIME);

        // Hash ibl_enabled
        hash ^= self.ibl_enabled as u64;
        hash = hash.wrapping_mul(FNV_PRIME);

        // Hash tonemap_mode
        hash ^= self.tonemap_mode as u64;
        hash = hash.wrapping_mul(FNV_PRIME);

        // Hash cascade_count
        hash ^= self.cascade_count as u64;
        hash = hash.wrapping_mul(FNV_PRIME);

        hash
    }

    /// Check if this permutation uses any textures
    pub fn has_textures(&self) -> bool {
        PbrMaterialFlags::from_bits(self.flags).has_any_texture()
    }

    /// Check if this permutation uses IBL
    pub fn uses_ibl(&self) -> bool {
        self.ibl_enabled || PbrMaterialFlags::from_bits(self.flags).contains(PbrMaterialFlags::USE_IBL)
    }

    /// Check if this permutation uses shadows
    pub fn uses_shadows(&self) -> bool {
        self.shadows_enabled
    }

    /// Get the material flags
    pub fn flags(&self) -> PbrMaterialFlags {
        PbrMaterialFlags::from_bits(self.flags)
    }

    /// Generate a human-readable description of this permutation
    pub fn describe(&self) -> String {
        let mut parts: Vec<String> = Vec::new();

        let flags = PbrMaterialFlags::from_bits(self.flags);
        if flags.contains(PbrMaterialFlags::HAS_ALBEDO_MAP) {
            parts.push("AlbedoMap".to_string());
        }
        if flags.contains(PbrMaterialFlags::HAS_NORMAL_MAP) {
            parts.push("NormalMap".to_string());
        }
        if flags.contains(PbrMaterialFlags::HAS_METALLIC_MAP) {
            parts.push("MetallicMap".to_string());
        }
        if flags.contains(PbrMaterialFlags::HAS_ROUGHNESS_MAP) {
            parts.push("RoughnessMap".to_string());
        }
        if flags.contains(PbrMaterialFlags::HAS_AO_MAP) {
            parts.push("AoMap".to_string());
        }
        if flags.contains(PbrMaterialFlags::HAS_EMISSIVE_MAP) {
            parts.push("EmissiveMap".to_string());
        }
        if flags.contains(PbrMaterialFlags::HAS_HEIGHT_MAP) {
            parts.push("HeightMap".to_string());
        }

        match self.alpha_mode {
            0 => parts.push("Opaque".to_string()),
            1 => parts.push("Mask".to_string()),
            2 => parts.push("Blend".to_string()),
            _ => parts.push("UnknownAlpha".to_string()),
        }

        parts.push(format!("Lights:{}", self.light_count));

        if self.shadows_enabled {
            parts.push(format!("CSM:{}", self.cascade_count));
        }

        if self.uses_ibl() {
            parts.push("IBL".to_string());
        }

        match self.tonemap_mode {
            0 => parts.push("TonemapNone".to_string()),
            1 => parts.push("TonemapACES".to_string()),
            2 => parts.push("TonemapReinhard".to_string()),
            3 => parts.push("TonemapFilmic".to_string()),
            _ => {}
        }

        parts.join("|")
    }
}

/// Cached shader program data
#[derive(Clone, Debug)]
pub struct CachedShader {
    /// The permutation key this shader was compiled for
    pub key: ShaderPermutationKey,
    /// Compiled vertex shader source (or binary)
    pub vertex_source: String,
    /// Compiled fragment shader source (or binary)
    pub fragment_source: String,
    /// Hash of the shader source for cache validation
    pub source_hash: u64,
    /// Reference count for cache management
    pub ref_count: u32,
}

impl CachedShader {
    /// Create a new cached shader entry
    pub fn new(key: ShaderPermutationKey, vertex: impl Into<String>, fragment: impl Into<String>) -> Self {
        let vertex = vertex.into();
        let fragment = fragment.into();
        let source_hash = Self::compute_hash(&vertex, &fragment);
        Self {
            key,
            vertex_source: vertex,
            fragment_source: fragment,
            source_hash,
            ref_count: 1,
        }
    }

    /// Compute a hash of the shader sources
    fn compute_hash(vertex: &str, fragment: &str) -> u64 {
        const FNV_OFFSET: u64 = 14695981039346656037;
        const FNV_PRIME: u64 = 1099511628211;

        let mut hash = FNV_OFFSET;
        for byte in vertex.as_bytes() {
            hash ^= *byte as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        for byte in fragment.as_bytes() {
            hash ^= *byte as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        hash
    }

    /// Increment the reference count
    pub fn add_ref(&mut self) {
        self.ref_count += 1;
    }

    /// Decrement the reference count, returns true if count reached zero
    pub fn release(&mut self) -> bool {
        if self.ref_count > 0 {
            self.ref_count -= 1;
        }
        self.ref_count == 0
    }
}

/// Shader permutation cache for managing compiled shader variants
#[derive(Clone, Debug, Default)]
pub struct PermutationCache {
    /// Map from permutation key hash to cached shader
    shaders: HashMap<u64, CachedShader>,
    /// Maximum cache size (0 = unlimited)
    max_size: usize,
    /// Total number of cache hits
    cache_hits: u64,
    /// Total number of cache misses
    cache_misses: u64,
}

impl PermutationCache {
    /// Create a new empty permutation cache
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new cache with a maximum size
    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            shaders: HashMap::new(),
            max_size,
            cache_hits: 0,
            cache_misses: 0,
        }
    }

    /// Get a cached shader by permutation key
    ///
    /// Returns a reference to the cached shader if it exists.
    pub fn get(&mut self, key: &ShaderPermutationKey) -> Option<&CachedShader> {
        let hash = key.hash();
        if self.shaders.contains_key(&hash) {
            self.cache_hits += 1;
            if let Some(shader) = self.shaders.get_mut(&hash) {
                shader.add_ref();
            }
            self.shaders.get(&hash)
        } else {
            self.cache_misses += 1;
            None
        }
    }

    /// Insert a shader into the cache
    pub fn insert(&mut self, shader: CachedShader) {
        // Check cache size limit
        if self.max_size > 0 && self.shaders.len() >= self.max_size {
            // Evict the shader with lowest reference count
            if let Some(evict_hash) = self
                .shaders
                .iter()
                .min_by_key(|(_, s)| s.ref_count)
                .map(|(h, _)| *h)
            {
                self.shaders.remove(&evict_hash);
            }
        }

        let hash = shader.key.hash();
        self.shaders.insert(hash, shader);
    }

    /// Remove a shader from the cache
    pub fn remove(&mut self, key: &ShaderPermutationKey) -> Option<CachedShader> {
        let hash = key.hash();
        self.shaders.remove(&hash)
    }

    /// Release a reference to a cached shader
    ///
    /// If the reference count reaches zero, the shader is removed from the cache.
    pub fn release(&mut self, key: &ShaderPermutationKey) {
        let hash = key.hash();
        let mut should_remove = false;
        if let Some(shader) = self.shaders.get_mut(&hash) {
            should_remove = shader.release();
        }
        if should_remove {
            self.shaders.remove(&hash);
        }
    }

    /// Clear all cached shaders
    pub fn clear(&mut self) {
        self.shaders.clear();
    }

    /// Get the number of cached shaders
    pub fn len(&self) -> usize {
        self.shaders.len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.shaders.is_empty()
    }

    /// Check if a permutation key is in the cache
    pub fn contains(&self, key: &ShaderPermutationKey) -> bool {
        self.shaders.contains_key(&key.hash())
    }

    /// Get the cache hit rate (0.0 to 1.0)
    pub fn hit_rate(&self) -> f32 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f32 / total as f32
        }
    }

    /// Get the total number of cache hits
    pub fn cache_hits(&self) -> u64 {
        self.cache_hits
    }

    /// Get the total number of cache misses
    pub fn cache_misses(&self) -> u64 {
        self.cache_misses
    }

    /// Reset the hit/miss counters
    pub fn reset_stats(&mut self) {
        self.cache_hits = 0;
        self.cache_misses = 0;
    }

    /// Get all cached permutation hashes
    pub fn keys(&self) -> Vec<u64> {
        self.shaders.keys().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation_key_default() {
        let key = ShaderPermutationKey::default();
        assert_eq!(key.flags, 0);
        assert_eq!(key.alpha_mode, 0);
        assert_eq!(key.light_count, 1);
        assert!(!key.shadows_enabled);
        assert!(!key.ibl_enabled);
        assert_eq!(key.tonemap_mode, 1);
        assert_eq!(key.cascade_count, 0);
    }

    #[test]
    fn test_permutation_key_new() {
        let key = ShaderPermutationKey::new();
        assert_eq!(key, ShaderPermutationKey::default());
    }

    #[test]
    fn test_permutation_key_from_flags() {
        let flags = PbrMaterialFlags::HAS_ALBEDO_MAP | PbrMaterialFlags::HAS_NORMAL_MAP;
        let key = ShaderPermutationKey::from_flags(flags);
        assert_eq!(key.flags, flags.bits());
    }

    #[test]
    fn test_permutation_key_builder() {
        let key = ShaderPermutationKey::new()
            .with_flags(PbrMaterialFlags::HAS_ALBEDO_MAP)
            .with_alpha_mode(2)
            .with_light_count(4)
            .with_shadows(true)
            .with_ibl(true)
            .with_tonemap(2)
            .with_cascades(4);

        assert_eq!(key.flags, PbrMaterialFlags::HAS_ALBEDO_MAP.bits());
        assert_eq!(key.alpha_mode, 2);
        assert_eq!(key.light_count, 4);
        assert!(key.shadows_enabled);
        assert!(key.ibl_enabled);
        assert_eq!(key.tonemap_mode, 2);
        assert_eq!(key.cascade_count, 4);
    }

    #[test]
    fn test_permutation_key_hash_consistent() {
        let key1 = ShaderPermutationKey::new().with_flags(PbrMaterialFlags::HAS_ALBEDO_MAP);
        let key2 = ShaderPermutationKey::new().with_flags(PbrMaterialFlags::HAS_ALBEDO_MAP);
        assert_eq!(key1.hash(), key2.hash());
    }

    #[test]
    fn test_permutation_key_hash_different() {
        let key1 = ShaderPermutationKey::new().with_flags(PbrMaterialFlags::HAS_ALBEDO_MAP);
        let key2 = ShaderPermutationKey::new().with_flags(PbrMaterialFlags::HAS_NORMAL_MAP);
        assert_ne!(key1.hash(), key2.hash());
    }

    #[test]
    fn test_permutation_key_hash_different_alpha() {
        let key1 = ShaderPermutationKey::new().with_alpha_mode(0);
        let key2 = ShaderPermutationKey::new().with_alpha_mode(1);
        assert_ne!(key1.hash(), key2.hash());
    }

    #[test]
    fn test_permutation_key_has_textures() {
        let key_with = ShaderPermutationKey::from_flags(PbrMaterialFlags::HAS_ALBEDO_MAP);
        let key_without = ShaderPermutationKey::new();
        assert!(key_with.has_textures());
        assert!(!key_without.has_textures());
    }

    #[test]
    fn test_permutation_key_uses_ibl() {
        let key1 = ShaderPermutationKey::new().with_ibl(true);
        let key2 = ShaderPermutationKey::from_flags(PbrMaterialFlags::USE_IBL);
        let key3 = ShaderPermutationKey::new();
        assert!(key1.uses_ibl());
        assert!(key2.uses_ibl());
        assert!(!key3.uses_ibl());
    }

    #[test]
    fn test_permutation_key_uses_shadows() {
        let key1 = ShaderPermutationKey::new().with_shadows(true);
        let key2 = ShaderPermutationKey::new();
        assert!(key1.uses_shadows());
        assert!(!key2.uses_shadows());
    }

    #[test]
    fn test_permutation_key_flags_getter() {
        let flags = PbrMaterialFlags::HAS_ALBEDO_MAP | PbrMaterialFlags::USE_IBL;
        let key = ShaderPermutationKey::from_flags(flags);
        assert_eq!(key.flags(), flags);
    }

    #[test]
    fn test_permutation_key_describe() {
        let key = ShaderPermutationKey::new()
            .with_flags(PbrMaterialFlags::HAS_ALBEDO_MAP)
            .with_alpha_mode(0)
            .with_light_count(2)
            .with_shadows(true)
            .with_cascades(4)
            .with_ibl(true)
            .with_tonemap(1);

        let desc = key.describe();
        assert!(desc.contains("AlbedoMap"));
        assert!(desc.contains("Opaque"));
        assert!(desc.contains("Lights:2"));
        assert!(desc.contains("CSM:4"));
        assert!(desc.contains("IBL"));
        assert!(desc.contains("TonemapACES"));
    }

    #[test]
    fn test_permutation_key_describe_empty() {
        let key = ShaderPermutationKey::new();
        let desc = key.describe();
        assert!(desc.contains("Opaque"));
        assert!(desc.contains("Lights:1"));
    }

    #[test]
    fn test_permutation_key_eq() {
        let key1 = ShaderPermutationKey::new().with_flags(PbrMaterialFlags::HAS_ALBEDO_MAP);
        let key2 = ShaderPermutationKey::new().with_flags(PbrMaterialFlags::HAS_ALBEDO_MAP);
        let key3 = ShaderPermutationKey::new().with_flags(PbrMaterialFlags::HAS_NORMAL_MAP);
        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_permutation_key_clone() {
        let key1 = ShaderPermutationKey::new().with_flags(PbrMaterialFlags::HAS_ALBEDO_MAP);
        let key2 = key1;
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_permutation_key_copy() {
        let key1 = ShaderPermutationKey::new().with_flags(PbrMaterialFlags::HAS_ALBEDO_MAP);
        let key2 = key1; // Copy
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_cached_shader_new() {
        let key = ShaderPermutationKey::new();
        let shader = CachedShader::new(key, "vertex code", "fragment code");
        assert_eq!(shader.vertex_source, "vertex code");
        assert_eq!(shader.fragment_source, "fragment code");
        assert_eq!(shader.ref_count, 1);
        assert!(shader.source_hash > 0);
    }

    #[test]
    fn test_cached_shader_add_ref() {
        let key = ShaderPermutationKey::new();
        let mut shader = CachedShader::new(key, "v", "f");
        assert_eq!(shader.ref_count, 1);
        shader.add_ref();
        assert_eq!(shader.ref_count, 2);
    }

    #[test]
    fn test_cached_shader_release() {
        let key = ShaderPermutationKey::new();
        let mut shader = CachedShader::new(key, "v", "f");
        assert!(shader.release()); // 1 -> 0, returns true when reaches 0
        assert_eq!(shader.ref_count, 0);
    }

    #[test]
    fn test_cached_shader_release_multiple() {
        let key = ShaderPermutationKey::new();
        let mut shader = CachedShader::new(key, "v", "f");
        shader.add_ref();
        shader.add_ref();
        assert_eq!(shader.ref_count, 3);
        assert!(!shader.release()); // 3 -> 2
        assert!(!shader.release()); // 2 -> 1
        assert!(shader.release()); // 1 -> 0, returns true
    }

    #[test]
    fn test_cached_shader_release_below_zero() {
        let key = ShaderPermutationKey::new();
        let mut shader = CachedShader::new(key, "v", "f");
        shader.release(); // 1 -> 0
        shader.release(); // already 0, should stay 0
        assert_eq!(shader.ref_count, 0);
    }

    #[test]
    fn test_cached_shader_source_hash_consistent() {
        let key = ShaderPermutationKey::new();
        let shader1 = CachedShader::new(key, "vertex", "fragment");
        let shader2 = CachedShader::new(key, "vertex", "fragment");
        assert_eq!(shader1.source_hash, shader2.source_hash);
    }

    #[test]
    fn test_cached_shader_source_hash_different() {
        let key = ShaderPermutationKey::new();
        let shader1 = CachedShader::new(key, "vertex1", "fragment");
        let shader2 = CachedShader::new(key, "vertex2", "fragment");
        assert_ne!(shader1.source_hash, shader2.source_hash);
    }

    #[test]
    fn test_permutation_cache_new() {
        let cache = PermutationCache::new();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
        assert_eq!(cache.cache_hits(), 0);
        assert_eq!(cache.cache_misses(), 0);
    }

    #[test]
    fn test_permutation_cache_insert_get() {
        let mut cache = PermutationCache::new();
        let key = ShaderPermutationKey::new().with_flags(PbrMaterialFlags::HAS_ALBEDO_MAP);
        let shader = CachedShader::new(key, "vertex", "fragment");

        cache.insert(shader);
        assert_eq!(cache.len(), 1);

        let result = cache.get(&key);
        assert!(result.is_some());
        assert_eq!(cache.cache_hits(), 1);
    }

    #[test]
    fn test_permutation_cache_miss() {
        let mut cache = PermutationCache::new();
        let key = ShaderPermutationKey::new();

        let result = cache.get(&key);
        assert!(result.is_none());
        assert_eq!(cache.cache_misses(), 1);
    }

    #[test]
    fn test_permutation_cache_remove() {
        let mut cache = PermutationCache::new();
        let key = ShaderPermutationKey::new();
        cache.insert(CachedShader::new(key, "v", "f"));

        let removed = cache.remove(&key);
        assert!(removed.is_some());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_permutation_cache_remove_nonexistent() {
        let mut cache = PermutationCache::new();
        let key = ShaderPermutationKey::new();
        let removed = cache.remove(&key);
        assert!(removed.is_none());
    }

    #[test]
    fn test_permutation_cache_release() {
        let mut cache = PermutationCache::new();
        let key = ShaderPermutationKey::new();
        cache.insert(CachedShader::new(key, "v", "f"));

        // Get adds a reference (ref_count = 2)
        cache.get(&key);
        // Release one (ref_count = 1, not removed)
        cache.release(&key);
        assert_eq!(cache.len(), 1);

        // Release again (ref_count = 0, removed)
        cache.release(&key);
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_permutation_cache_clear() {
        let mut cache = PermutationCache::new();
        cache.insert(CachedShader::new(ShaderPermutationKey::new(), "v", "f"));
        cache.insert(CachedShader::new(
            ShaderPermutationKey::new().with_flags(PbrMaterialFlags::HAS_ALBEDO_MAP),
            "v2",
            "f2",
        ));
        assert_eq!(cache.len(), 2);

        cache.clear();
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_permutation_cache_contains() {
        let mut cache = PermutationCache::new();
        let key = ShaderPermutationKey::new();
        cache.insert(CachedShader::new(key, "v", "f"));

        assert!(cache.contains(&key));
        assert!(!cache.contains(&ShaderPermutationKey::new().with_flags(PbrMaterialFlags::HAS_NORMAL_MAP)));
    }

    #[test]
    fn test_permutation_cache_hit_rate() {
        let mut cache = PermutationCache::new();
        let key = ShaderPermutationKey::new();
        cache.insert(CachedShader::new(key, "v", "f"));

        // 1 hit
        cache.get(&key);
        // 1 miss
        cache.get(&ShaderPermutationKey::new().with_flags(PbrMaterialFlags::HAS_NORMAL_MAP));

        let rate = cache.hit_rate();
        assert!((rate - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_permutation_cache_hit_rate_empty() {
        let cache = PermutationCache::new();
        assert_eq!(cache.hit_rate(), 0.0);
    }

    #[test]
    fn test_permutation_cache_reset_stats() {
        let mut cache = PermutationCache::new();
        let key = ShaderPermutationKey::new();
        cache.insert(CachedShader::new(key, "v", "f"));
        cache.get(&key);
        cache.get(&ShaderPermutationKey::new().with_flags(PbrMaterialFlags::HAS_NORMAL_MAP));

        cache.reset_stats();
        assert_eq!(cache.cache_hits(), 0);
        assert_eq!(cache.cache_misses(), 0);
    }

    #[test]
    fn test_permutation_cache_with_capacity_eviction() {
        let mut cache = PermutationCache::with_capacity(2);

        let key1 = ShaderPermutationKey::new();
        let key2 = ShaderPermutationKey::new().with_flags(PbrMaterialFlags::HAS_ALBEDO_MAP);
        let key3 = ShaderPermutationKey::new().with_flags(PbrMaterialFlags::HAS_NORMAL_MAP);

        cache.insert(CachedShader::new(key1, "v1", "f1"));
        cache.insert(CachedShader::new(key2, "v2", "f2"));
        assert_eq!(cache.len(), 2);

        // Inserting a third should evict one (key1 has lowest ref_count since we never got it)
        cache.insert(CachedShader::new(key3, "v3", "f3"));
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_permutation_cache_keys() {
        let mut cache = PermutationCache::new();
        let key1 = ShaderPermutationKey::new();
        let key2 = ShaderPermutationKey::new().with_flags(PbrMaterialFlags::HAS_ALBEDO_MAP);

        cache.insert(CachedShader::new(key1, "v1", "f1"));
        cache.insert(CachedShader::new(key2, "v2", "f2"));

        let keys = cache.keys();
        assert_eq!(keys.len(), 2);
    }

    #[test]
    fn test_permutation_cache_clone() {
        let mut cache = PermutationCache::new();
        cache.insert(CachedShader::new(ShaderPermutationKey::new(), "v", "f"));

        let cache2 = cache.clone();
        assert_eq!(cache.len(), cache2.len());
    }

    #[test]
    fn test_permutation_key_hash_all_fields() {
        // Verify that changing each field changes the hash
        let base = ShaderPermutationKey::new();

        let variants = [
            ShaderPermutationKey::new().with_flags(PbrMaterialFlags::HAS_ALBEDO_MAP),
            ShaderPermutationKey::new().with_alpha_mode(1),
            ShaderPermutationKey::new().with_light_count(2),
            ShaderPermutationKey::new().with_shadows(true),
            ShaderPermutationKey::new().with_ibl(true),
            ShaderPermutationKey::new().with_tonemap(2),
            ShaderPermutationKey::new().with_cascades(4),
        ];

        let base_hash = base.hash();
        for variant in &variants {
            assert_ne!(base_hash, variant.hash(), "Hash should differ for variant");
        }
    }

    #[test]
    fn test_permutation_key_describe_all_alpha_modes() {
        let key_opaque = ShaderPermutationKey::new().with_alpha_mode(0);
        let key_mask = ShaderPermutationKey::new().with_alpha_mode(1);
        let key_blend = ShaderPermutationKey::new().with_alpha_mode(2);

        assert!(key_opaque.describe().contains("Opaque"));
        assert!(key_mask.describe().contains("Mask"));
        assert!(key_blend.describe().contains("Blend"));
    }

    #[test]
    fn test_permutation_key_describe_all_tonemaps() {
        assert!(ShaderPermutationKey::new().with_tonemap(0).describe().contains("TonemapNone"));
        assert!(ShaderPermutationKey::new().with_tonemap(1).describe().contains("TonemapACES"));
        assert!(ShaderPermutationKey::new().with_tonemap(2).describe().contains("TonemapReinhard"));
        assert!(ShaderPermutationKey::new().with_tonemap(3).describe().contains("TonemapFilmic"));
    }
}
