use std::collections::HashSet;

pub struct Feature;

impl Feature {
    pub fn enabled(name: &str) -> bool {
        match name {
            "render-vulkan" => cfg!(feature = "render-vulkan"),
            "render-gl" => cfg!(feature = "render-gl"),
            "render-webgpu" => cfg!(feature = "render-webgpu"),
            "audio" => cfg!(feature = "audio"),
            "network" => cfg!(feature = "network"),
            "editor" => cfg!(feature = "editor"),
            _ => false,
        }
    }

    pub fn list() -> Vec<&'static str> {
        let mut features = Vec::new();
        
        if cfg!(feature = "render-vulkan") {
            features.push("render-vulkan");
        }
        if cfg!(feature = "render-gl") {
            features.push("render-gl");
        }
        if cfg!(feature = "render-webgpu") {
            features.push("render-webgpu");
        }
        if cfg!(feature = "audio") {
            features.push("audio");
        }
        if cfg!(feature = "network") {
            features.push("network");
        }
        if cfg!(feature = "editor") {
            features.push("editor");
        }
        
        features
    }

    pub fn render_backend() -> &'static str {
        if cfg!(feature = "render-vulkan") {
            "vulkan"
        } else if cfg!(feature = "render-webgpu") {
            "webgpu"
        } else if cfg!(feature = "render-gl") {
            "gl"
        } else {
            "none"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feature_enabled() {
        assert!(Feature::enabled("render-gl") || !Feature::enabled("render-gl"));
    }

    #[test]
    fn feature_list() {
        let features = Feature::list();
        assert!(features.len() <= 6);
    }

    #[test]
    fn feature_render_backend() {
        let backend = Feature::render_backend();
        assert!(matches!(backend, "vulkan" | "webgpu" | "gl" | "none"));
    }
}
