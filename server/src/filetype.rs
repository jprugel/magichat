use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum FileType {
    Png,
    Svg,
    Jpg,
    Json,
    Unknown,
}

impl FileType {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_ascii_lowercase().as_str() {
            "png" => FileType::Png,
            "svg" => FileType::Svg,
            "jpg" | "jpeg" => FileType::Jpg,
            "json" => FileType::Json,
            _ => FileType::Unknown,
        }
    }
}

pub fn get_file_type<P: AsRef<Path>>(path: P) -> FileType {
    path.as_ref()
        .extension()
        .and_then(|e| e.to_str())
        .map(FileType::from_extension)
        .unwrap_or(FileType::Unknown)
}
