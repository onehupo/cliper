#[derive(Debug)]
pub struct CliperInfo {
    // | id | file Path | Name  | Size | Download | Type  | File Type | File Folder |
    pub id : u64,
    pub file_path: String,
    pub name: String,
    pub size: u64,
    pub download: u64,
    pub file_type: String,
    pub file_ext: String,
    pub file_folder: String,
    pub md5: String,
}

impl CliperInfo {
    pub fn new() -> CliperInfo {
        CliperInfo {
            id: 0,
            file_path: String::new(),
            name: String::new(),
            size: 0,
            download: 0,
            file_type: String::new(),
            file_ext: String::new(),
            file_folder: String::new(),
            md5: String::new(),
        }
    }
}
