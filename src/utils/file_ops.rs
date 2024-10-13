use std::fs;

pub fn get_maps_list() -> Vec<String> {
    let paths = fs::read_dir("maps").unwrap();
    let mut maps = Vec::new();
    for path in paths {
        let path = path.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension == "bin" {
                if let Some(name) = path.file_name() {
                    maps.push(name.to_string_lossy().into_owned());
                }
            }
        }
    }
    maps
}
