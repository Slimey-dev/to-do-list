use std::io::stdin;
use std::path::Path;

mod reader;
mod writer;

fn main() {
    let path = Path::new("list.txt");
    println!("Welcome to your to-do-list!");
    loop {
        println!("Enter a command (help to see all the commands):");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "help" => help(),
            "list" => list(&path),
            "add" => add(&path),
            "check" => check(&path),
            "uncheck" => uncheck(&path),
            "remove" => remove(&path),
            "remove completed" => remove_completed(&path),
            "quit" => break,
            _ => println!("Invalid command!"),
        }
    }
}

fn help() {
    println!("Commands:");
    println!("help - show this message");
    println!("list - show the to-do list");
    println!("add - add an item to the to-do list");
    println!("check - check an item on the to-do list");
    println!("uncheck - uncheck an item on the to-do list");
    println!("remove - remove an item from the to-do list");
    println!("remove completed - remove all completed items from the to-do list");
    println!("quit - quit the program");
}

fn list(path: &Path) {
    let map = reader::load_into_map(path).unwrap();
    let mut i = 1;
    for (key, value) in &map {
        if *value {
            println!("{}. [x] {}", i, key);
        } else {
            println!("{}. [ ] {}", i, key);
        }
        i += 1;
    }
}

fn add(path: &Path) {
    println!("Enter the item you want to add:");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let mut map = reader::load_into_map(path).unwrap();
    map.insert(input.to_string(), false);
    writer::write_into_file_from_map(&map, path);
}

fn process_item(path: &Path, checked: bool, prompt_message: &str) {
    println!("{}", prompt_message);
    list(path);
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let mut map = reader::load_into_map(path).unwrap();
    let mut target = None;
    let mut i = 1;
    for (key, _) in &mut map {
        if i == input.parse::<usize>().unwrap() {
            target = Some(key.to_string()); // Store target key.
            break; // We found our element, so we can stop iterating.
        }
        i += 1;
    }
    // Now that we're no longer borrowing `map`, we can insert to it.
    if let Some(key) = target {
        map.insert(key, checked);
        writer::write_into_file_from_map(&map, path);
    }
}

fn check(path: &Path) {
    let prompt_message = "Enter the index of the item you want to check:";
    process_item(path, true, prompt_message);
}

fn uncheck(path: &Path) {
    let prompt_message = "Enter the index of the item you want to uncheck:";
    process_item(path, false, prompt_message);
}

fn remove(path: &Path) {
    println!("Enter the index of the item you want to remove:");
    list(path);
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let mut map = reader::load_into_map(path).unwrap();
    let mut to_remove = Vec::new();
    let mut i = 1;
    for (key, _) in &map {
        if i == input.parse::<usize>().unwrap() {
            to_remove.push(key.to_string());
        }
        i += 1;
    }
    for key in to_remove {
        map.remove(&key);
    }
    writer::write_into_file_from_map(&map, path);
}

fn remove_completed(path: &Path) {
    let mut map = reader::load_into_map(path).unwrap();
    let mut to_remove = Vec::new();
    for (key, value) in &map {
        if *value {
            to_remove.push(key.to_string());
        }
    }
    for key in to_remove {
        map.remove(&key);
    }
    writer::write_into_file_from_map(&map, path);
}
