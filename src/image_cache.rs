use crate::models::ImageParams;
use lru::LruCache;
use std::num::NonZeroUsize;

pub struct ImageCache {
    cache: LruCache<String, Vec<u8>>,
}

impl ImageCache {
    pub fn new(capacity: usize) -> Self {
        ImageCache {
            cache: LruCache::new(NonZeroUsize::new(capacity).unwrap()),
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&Vec<u8>> {
        self.cache.get(key)
    }

    pub fn insert(&mut self, key: String, value: Vec<u8>) {
        self.cache.put(key, value);
    }

    pub fn get_cache_id(params: &ImageParams) -> String {
        format!(
            "{}_{}_{}",
            params.file_name,
            params.width.unwrap_or(0),
            params.height.unwrap_or(0)
        )
    }
}
