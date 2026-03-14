// Media placeholder layout.
// TODO: Integrate with skparagraph getRectsForPlaceholders() for production.

use std::path::{Path, PathBuf};

/// Media asset metadata.
#[derive(Debug, Clone)]
pub struct MediaAsset {
    pub id: usize,
    pub path: PathBuf,
    pub width: f32,
    pub height: f32,
}

/// Media manager for lazy-loading and caching.
pub struct MediaManager {
    assets: Vec<MediaAsset>,
}

impl MediaManager {
    /// Create new media manager.
    pub fn new() -> Self {
        Self {
            assets: Vec::new(),
        }
    }

    /// Register a media file (image or video).
    pub fn register_media(&mut self, path: &Path, width: f32, height: f32) -> usize {
        let id = self.assets.len();
        self.assets.push(MediaAsset {
            id,
            path: path.to_path_buf(),
            width,
            height,
        });
        id
    }

    /// Get asset by ID.
    pub fn get_asset(&self, id: usize) -> Option<&MediaAsset> {
        self.assets.get(id)
    }
}

impl Default for MediaManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_media() {
        let mut manager = MediaManager::new();
        let id = manager.register_media(Path::new("test.png"), 100.0, 100.0);
        assert_eq!(id, 0);
        assert!(manager.get_asset(0).is_some());
    }
}
