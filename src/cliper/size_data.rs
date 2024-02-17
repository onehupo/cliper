#[derive(Debug)]
pub struct SizeData {
    // ... fields for fileName, version, asserts, res, code, native, others, all, etc.
    pub fileName: String,
    pub asserts: u64,
    pub res: u64,
    pub code: u64,
    pub native: u64,
    pub others: u64,
    pub all: u64,
}

impl SizeData {
    // 原始数据转换成如果是1000以上的数据，转换成KB，如果是1000以上的数据，转换成MB, 保留2位小数
    pub fn convert_size(&self, size: u64) -> String {
        if size > 1000 {
            let kb = size as f64 / 1000.00;
            if kb > 1000.00 {
                let mb = kb / 1000.00;
                return format!("{:.2}MB", mb);
            } else {
                // 保留2位小数
                return format!("{:.2}KB", kb);
            }
        } else {
            return format!("{}B", size);
        }
    }

    pub fn new() -> SizeData {
        SizeData {
            fileName: String::new(),
            asserts: 0,
            res: 0,
            code: 0,
            native: 0,
            others: 0,
            all: 0,
        }
    }
}
