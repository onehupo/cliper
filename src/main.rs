use std::env;
use std::fs::File;
use zip::read::ZipArchive;
use std::collections::HashMap;
use prettytable::{Table, Row, Cell, row, cell};
use async_std::task;

mod app;
use app::manifest_parser::parser;

#[derive(Debug)]
struct SizeData {
    // ... fields for fileName, version, asserts, res, code, native, others, all, etc.
    fileName: String,
    asserts: u64,
    res: u64,
    code: u64,
    native: u64,
    others: u64,
    all: u64
}

impl SizeData {
    // 原始数据转换成如果是1000以上的数据，转换成KB，如果是1000以上的数据，转换成MB
    fn convert_size(&self, size: u64) -> String {
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

    fn new() -> SizeData {
        SizeData {
            fileName: String::new(),
            asserts: 0,
            res: 0,
            code: 0,
            native: 0,
            others: 0,
            all: 0
        }
    }
}

fn read_size(filename: &str) -> zip::result::ZipResult<SizeData> {
    let file = File::open(filename)?;
    let mut archive = ZipArchive::new(file)?;

    let mut file_info = SizeData::new();

    file_info.fileName = String::from(filename);

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        let name = file.name().to_string();
        let size = file.size();
        let download = file.compressed_size();

        if name.starts_with("assets/") || name.starts_with("base/assets/") {
            file_info.asserts += download;
        } else if name.starts_with("res/") || name.starts_with("base/res/") {
            file_info.res += download;
        } else if name.starts_with("resources.arsc") || name.ends_with("resources.arsc") {
            file_info.res += download;
        } else if name.starts_with("classes") || name.starts_with("base/dex/classes") {
            file_info.code += download;
        } else if name.starts_with("lib/") || name.starts_with("base/lib") {
            file_info.native += download;
        } else {
            file_info.others += download;
        }
    }

    file_info.all = file_info.asserts + file_info.res + file_info.code + file_info.native + file_info.others;

    Ok(file_info)
}

async fn parse(filename: &str) {
    match parser::parse(&filename).await {
        Some(value) => {
            println!("Icon: {}", value.icon);
            println!("Version Code: {}", value.version_code);
            println!("Version Name: {}", value.version_name);
        },
        None => println!("Failed to parse APK information")
    }
}

fn main() {
    // 解析输入的命令
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    println!("-----------------------------------------");
    // 读取工程根目录
    let project_path = env::current_dir().unwrap();
    println!("Current Path: {}", project_path.display());
    // 根目录下面的build文件
    let build_path = project_path.join("build");
    println!("Build Path: {}", build_path.display());
    // build目录下的apk文件
    let apk_path = build_path.join("app.apk");
    // let apk_paht = "/Users/liangrui/Work/liangrui/cliper/build/app.apk";
    // Using async-std
    println!("-----------------------------------------");
    println!("File Name : {}", &apk_path.to_string_lossy());

    task::block_on(parse(&apk_path.to_string_lossy()));

    // read_apk_info(&apk_path.to_string_lossy());

    match read_size(&apk_path.to_string_lossy()) {
        Ok(value) => {
            let mut table = Table::new();
            table.add_row(row!["Asserts", "Res", "Code", "Native", "Others", "All"]);
            table.add_row(Row::new(vec![
                Cell::new(&value.convert_size(value.asserts)),
                Cell::new(&value.convert_size(value.res)),
                Cell::new(&value.convert_size(value.code)),
                Cell::new(&value.convert_size(value.native)),
                Cell::new(&value.convert_size(value.others)),
                Cell::new(&value.convert_size(value.all))
            ]));
            table.printstd();  
        },
        Err(e) => println!("Failed to read APK information: {}", e)
    }
}



