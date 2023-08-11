mod reader;
mod writer;

fn main() {
    let path = std::path::Path::new("hello.txt");
    let mut map = reader::load_into_map(&path).unwrap();

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
    map.insert("helllo".to_string(), true);

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    writer::write_into_file_from_map(&map, &path);
    let map = reader::load_into_map(&path).unwrap();

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}
