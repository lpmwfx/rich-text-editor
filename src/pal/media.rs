#![allow(non_camel_case_types)]
// Media placeholder layout.

use std::path::{Path, PathBuf};

/// Media asset metadata.
#[derive(Debug, Clone)]
pub struct MediaAsset_pal {
    pub id: usize,
    pub path: PathBuf,
    pub width: f32,
    pub height: f32,
}

/// Media manager for lazy-loading and caching.
pub struct MediaManager_pal {
    assets: Vec<MediaAsset_pal>,
}

impl MediaManager_pal {
    /// Create new media manager.
    pub fn new() -> Self {
        Self {
            assets: Vec::new(),
        }
    }

    /// Register a media file (image or video).
    pub fn register_media(&mut self, path: &Path, width: f32, height: f32) -> usize {
        let id = self.assets.len();
        self.assets.push(MediaAsset_pal {
            id,
            path: path.to_path_buf(),
            width,
            height,
        });
        id
    }

    /// Get asset by ID.
    pub fn get_asset(&self, id: usize) -> Option<&MediaAsset_pal> {
        self.assets.get(id)
    }
}

impl Default for MediaManager_pal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_media() {
        let mut manager = MediaManager_pal::new();
        let id = manager.register_media(Path::new("test.png"), 100.0, 100.0);
        assert_eq!(id, 0);
        assert!(manager.get_asset(0).is_some());
    }
}
