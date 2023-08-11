use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use indexmap::IndexMap;

pub fn write_into_file_from_map(map: &IndexMap<String, bool>, path: &Path) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .expect("Could not open file");

    for (key, value) in map {
        let line = format!("{},{}\n", key, value);
        file.write_all(line.as_bytes())
            .expect("Could not write to file");
    }
}
