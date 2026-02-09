use crate::hashing::validate_hash;
use crate::models::{Algorithm, HashEntry};
use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

pub struct HashCache {
    cache: HashMap<String, String>,
    csv_path: std::path::PathBuf,
    base_path: std::path::PathBuf,
}

impl HashCache {
    /// Create a new HashCache
    pub fn new(csv_path: std::path::PathBuf, base_path: std::path::PathBuf) -> Self {
        Self {
            cache: HashMap::new(),
            csv_path,
            base_path,
        }
    }

    /// Load a hash CSV file and merge entries into the cache
    pub fn load_csv(&mut self, csv_path: &Path) -> Result<usize> {
        let mut loaded = 0;
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_path(csv_path)?;

        // Get the directory containing the CSV file
        let csv_dir = csv_path.parent().unwrap_or(&self.base_path);

        for result in rdr.deserialize() {
            if let Ok(entry) = result {
                let entry: HashEntry = entry;
                // Validate hash before adding to cache
                if validate_hash(&entry.hash, entry.algo) {
                    // Adjust path relative to the CSV's location
                    let adjusted_path = if entry.path.starts_with('/') || entry.path.starts_with('\\') {
                        entry.path.clone()
                    } else {
                        csv_dir
                            .join(&entry.path)
                            .strip_prefix(&self.base_path)
                            .unwrap_or(Path::new(&entry.path))
                            .to_string_lossy()
                            .into_owned()
                    };

                    let key = format!(
                        "{}|{}|{}|{:?}",
                        adjusted_path, entry.size, entry.time, entry.algo
                    );
                    self.cache.insert(key, entry.hash);
                    loaded += 1;
                }
            }
        }

        Ok(loaded)
    }

    /// Get a hash from the cache
    pub fn get(&self, path: &str, size: u64, mtime: u64, algo: Algorithm) -> Option<&String> {
        let key = format!("{}|{}|{}|{:?}", path, size, mtime, algo);
        self.cache.get(&key)
    }

    /// Check if cache contains an entry
    #[allow(dead_code)]
    pub fn contains(&self, path: &str, size: u64, mtime: u64, algo: Algorithm) -> bool {
        let key = format!("{}|{}|{}|{:?}", path, size, mtime, algo);
        self.cache.contains_key(&key)
    }

    /// Append a new hash entry to the CSV file
    pub fn append(&self, entry: &HashEntry) -> Result<()> {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.csv_path)?;

        let needs_header = !self.csv_path.exists() 
            || std::fs::metadata(&self.csv_path)?.len() == 0;

        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b';')
            .has_headers(needs_header)
            .from_writer(file);

        wtr.serialize(entry)?;
        wtr.flush()?;
        Ok(())
    }

    /// Get the number of cached hashes
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if cache is empty
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Get a reference to the internal cache map
    #[allow(dead_code)]
    pub fn inner(&self) -> &HashMap<String, String> {
        &self.cache
    }
}
