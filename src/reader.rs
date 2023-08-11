use indexmap::IndexMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn load_into_map(path: &Path) -> io::Result<IndexMap<String, bool>> {
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut map = IndexMap::new();

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.splitn(2, ',').map(|s| s.trim());

        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            map.insert(key.to_string(), value.parse().unwrap_or(false));
        }
    }

    Ok(map)
}
