use crate::prelude::*;

fn is_csv_file(file_path: &Path) -> bool {
    file_path.extension().map_or(false, |ext| ext == "csv")
}

fn path_to_string(path: &Path) -> String {
    format!("{}", path.to_str().unwrap_or("invalid path"))
}


pub fn validate_file_paths(paths: Vec<&Path>) -> Result<Vec<String>, (Vec<String>, Vec<String>)> {
    let mut ok_paths: Vec<String> = vec![];
    let mut errors: Vec<String> = vec![];

    for path in paths {
        if let Ok(metadata) = fs::metadata(path) {
            if metadata.is_file() {
                if is_csv_file(path) {
                    ok_paths.push(format!("{}", path_to_string(path).on_green()));
                } else {
                    errors.push(format!("{} - not a CSV file", path_to_string(path).on_red()));
                }
            } else {
                errors.push(format!("{} - not a file", path_to_string(path).on_red()));
            }
        } else {
            errors.push(format!("{} - invalid file path", path_to_string(path).on_red()));
        }
    }

    if errors.len() > 0 {
        return Err((ok_paths, errors));
    }

    return Ok(ok_paths);
}
