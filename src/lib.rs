use std::path::PathBuf;

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

#[cfg(test)]
mod tests {
    use super::*;
    
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