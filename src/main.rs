use std::process;
use gis_utils::{ check_shp, check_tif, Config };

fn main() {
    let config = Config::build().unwrap_or_else(|e| {
        eprintln!("Problem building config: {e}");
        process::exit(1);
    });

    let shp_found = check_shp(&config.entries);

    let tif_found = check_tif(&config.entries);

    if shp_found {
        println!("Yes, .shp file found in {}", config.current_dir.to_str().unwrap());
    } else {
        println!("No, .shp file not found in {}", config.current_dir.to_str().unwrap());
    }

    if tif_found {
        println!("Yes, .tif file found in {}", config.current_dir.to_str().unwrap());
    } else {
        println!("No, .tif file not found in {}", config.current_dir.to_str().unwrap());
    }

    let mut counter = 1;
    for arg_n in &config.args {
        println!("Argument #{}; {}", counter, arg_n);
        counter += 1;
    }
}