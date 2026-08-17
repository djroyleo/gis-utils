use std::env;
use std::fs;

fn main() {
    // Get the current working directory
    let current_dir = env::current_dir().expect("Unable to determine current directory");

    // Read the entries in that directory
    let entries = fs::read_dir(&current_dir).expect("Unable to read directory contents");

    // Check whether any entry is a file with a .shp extension
    let found = entries
        .filter_map(|entry| entry.ok()) // ignore entries that errored out
        .any(|entry| {
            let path = entry.path();
            path.is_file()
                && path
                    .extension()
                    .map(|ext| ext.eq_ignore_ascii_case("shp"))
                    .unwrap_or(false)
        });

    if found {
        println!("Yes");
    } else {
        println!("No");
    }
}