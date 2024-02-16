use std::env;
use prettytable::{Table, Row, Cell, row, cell};
use async_std::task;

mod app;
use app::manifest_parser::parser;
mod cliper;
use cliper::apk_cliper::size_reader;

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

async fn read_size(filename: &str) {
    match size_reader::read_detail_info(filename) {
        Ok(value) => {
            let mut table = Table::new();
            let mut line_num = 0;
            table.add_row(row!["id", "Folder Path", "Name", "Size", "Download", "Type", "File Type", "File Folder"]);
            let length = value.len();
            println!("Total: {}", length);
            for cliper_item in value {
                table.add_row(Row::new(vec![
                    Cell::new(&cliper_item.id.to_string()),
                    Cell::new(&cliper_item.file_path),
                    Cell::new(&cliper_item.name),
                    Cell::new(&cliper_item.size.to_string()),
                    Cell::new(&cliper_item.download.to_string()),
                    Cell::new(&cliper_item.file_ext),
                    Cell::new(&cliper_item.file_type.to_string()),
                    Cell::new(&cliper_item.file_folder),
                ]));
                line_num += 1;
                if line_num > 10 {
                    break;
                }
            }
            table.printstd();
        },
        Err(e) => println!("Failed to read APK information: {}", e)
    }

    match size_reader::read_size(filename) {
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

    task::block_on(read_size(&apk_path.to_string_lossy()));
}



