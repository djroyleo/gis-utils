//! > **A Command Line GIS Utility in Pure Rust**
//! 
use std::fs;
use std::env;
use std::path::PathBuf;
use std::error::Error;

pub fn check_shp(entries: &[PathBuf]) -> bool {
    entries.iter().any(|path| {
        path.is_file()
            && path
                .extension()
                .map(|ext| ext.eq_ignore_ascii_case("shp"))
                .unwrap_or(false)
    })
}

pub fn check_tif(entries: &[PathBuf]) -> bool {
    entries.iter().any(|path| {
        path.is_file()
            && path
                .extension()
                .map(|ext| ext.eq_ignore_ascii_case("tif"))
                .unwrap_or(false)
    })
}

#[derive(Debug)]
pub struct Config {
    pub current_dir: PathBuf,
    pub args: Vec<String>,
    pub entries: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct Command {
    pub main: Option<MainCommand>,
    pub modifier: Option<ModifierCommand>,
    pub sub_modifier: Option<SubModifierCommand>,
}

#[derive(Debug)]
pub enum MainCommand {

}

#[derive(Debug)]
pub enum ModifierCommand {

}

#[derive(Debug)]
pub enum SubModifierCommand {

}

impl Config {
    pub fn build() -> Result<Config, Box<dyn Error>> {
        let current_dir = env::current_dir()?;
        let args = env::args().skip(1).collect();
        let entries = fs::read_dir(&current_dir)
            .unwrap()
            .filter_map(|entry| { entry.ok() })
            .map(|entry| { entry.path() })
            .collect();
        Ok(Config {
            current_dir,
            args,
            entries,
        })
    }

    pub fn arg_to_command() -> Result<Command, Box<dyn Error>> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn shapefile_test() {

    }

    #[test]
    fn tiff_test() {

    }

    #[test]
    fn third_test() {
        panic!("Panicking test three.")
    }
}