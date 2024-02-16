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
    pub file_folder: String
}

impl CliperInfo {
    // 原始数据转换成如果是1000以上的数据，转换成KB，如果是1000以上的数据，转换成MB
    pub fn convert_size(&self, size: u64) -> String {
        if size > 1000 {
            let kb = size / 1000;
            if kb > 1000 {
                let mb = kb / 1000;
                return format!("{}MB", mb);
            } else {
                return format!("{}KB", kb);
            }
        } else {
            return format!("{}B", size);
        }
    }

    pub fn new() -> CliperInfo {
        CliperInfo {
            id: 0,
            file_path: String::new(),
            name: String::new(),
            size: 0,
            download: 0,
            file_type: String::new(),
            file_ext: String::new(),
            file_folder: String::new()
        }
    }
}
