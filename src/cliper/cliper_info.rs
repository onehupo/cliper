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
    pub fn clone(&self) -> CliperInfo {
        CliperInfo {
            id: self.id,
            file_path: self.file_path.clone(),
            name: self.name.clone(),
            size: self.size,
            download: self.download,
            file_type: self.file_type.clone(),
            file_ext: self.file_ext.clone(),
            file_folder: self.file_folder.clone(),
            md5: self.md5.clone(),
        }
    }
}
