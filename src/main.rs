use std::fs::File;
use std::{env, fs};

use async_std::task;
use csv::Writer;
use prettytable::{cell, format, row, Cell, Row, Table};
use structopt::StructOpt;

mod app;
use app::manifest_parser::parser;
mod cliper;
use cliper::apk_cliper::size_reader;
use cliper::cliper_info::CliperInfo;

/**
 * 过滤器
 * 过滤路径，过滤大小，过滤后缀，过滤类型
 * cargo run -- --input /Users/liangrui/Work/liangrui/cliper/build/app.apk --filter-type Res --filter-ext .png --filter-size 10000 --filter-path assets
 * cargo run -- --input ./build/app.apk --filter-type Res --filter-ext .png --filter-size 10000 --filter-path assets
 */
#[derive(Debug, StructOpt)]
struct CliperFilter {
    #[structopt(short, long, help = "debug")]
    debug: bool,
    #[structopt(long, default_value = "", help = "输入文件")]
    input: String,
    #[structopt(long, default_value = "", help = "过滤路径")]
    filter_path: String,
    #[structopt(long, default_value = "0", help = "过滤大小")]
    filter_size: u64,
    #[structopt(long, default_value = "", help = "过滤后缀")]
    filter_ext: String,
    #[structopt(long, default_value = "", help = "过滤类型")]
    filter_type: String,
}
// 添加一个过滤器，过滤掉不需要的文件, 满足条件的返回true
fn cliper_filter(info: &CliperInfo, filter: &CliperFilter) -> bool {
    let path_filter = filter.filter_path.as_str();
    let size_filter = &filter.filter_size;
    let ext_filter = filter.filter_ext.as_str();
    let type_filter = filter.filter_type.as_str();

    let mut result = true;
    // 过滤路径 路径不为空并且不是以过滤路径开头的，为true，不满足条件
    let filter_path_enable = !path_filter.is_empty() && !info.file_path.starts_with(path_filter);
    // 过滤大小 大小不为0并且下载大小小于过滤大小，为true，不满足条件
    let filter_size_enable = *size_filter > 0 && info.download < *size_filter;
    // 过滤后缀 后缀不为空并且不是以过滤后缀结尾的，为true，不满足条件
    let filter_ext_enable = !ext_filter.is_empty() && !info.file_path.ends_with(ext_filter);
    // 过滤类型 类型不为空并且不是过滤类型，为true，不满足条件
    let filter_type_enable = !type_filter.is_empty() && info.file_type != type_filter;
    // 如果有一个条件满足，就返回true
    if filter_path_enable || filter_size_enable || filter_ext_enable || filter_type_enable {
        result = false;
    }
    return result;
}

async fn read_info(filename: &str) {
    match parser::parse(&filename).await {
        Some(value) => {
            let message = format!("APK Information:\nFile: {}\nPackage{}\nIcon: {}\nVersion Code: {}\nVersion Name: {}", 
                filename, value.package_name, value.icon, value.version_code, value.version_name);
            println_message(message.as_str());
        }
        None => {
            println!("");
            printline();
            println!("Failed to read APK information");
            printline();
        }
    }
}

async fn read_detail_info(filename: &str, filter: &CliperFilter) {
    match size_reader::read_detail_info(filename) {
        Ok(value) => {
            let mut table = Table::new();
            let mut line_num = 0;
            table.add_row(row![
                "id",
                "Folder Path",
                "Name",
                "Size",
                "Download",
                "Type",
                "File Type",
                "File Folder"
            ]);
            for cliper_item in &value {
                if !cliper_filter(cliper_item, filter) {
                    continue;
                }
                line_num += 1;
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
            }
            println!("");
            printline();
            println!("Total: {}, Filter: {}", &value.len(), line_num);
            if filter.debug {
                table.printstd();
            } else {
                let limit = 10;
                let mut limited_table = Table::new();
                for row in table.row_iter().take(limit) {
                    limited_table.add_row(Row::new(
                        row.iter()
                            .map(|cell| Cell::new(&cell.get_content()))
                            .collect(),
                    ));
                }
                limited_table.printstd();
                println!("> For more details, please use --debug option or -d option")
            }
            printline();
            let output = get_build_dir() + "/table_detail.csv";
            create_csv(&table, &output);
        }
        Err(e) => {
            println!("");
            printline();
            println!("Failed to read APK information: {}", e);
            printline();
        }
    }
}

async fn read_total(filename: &str) {
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
                Cell::new(&value.convert_size(value.all)),
            ]));
            println!("");
            printline();
            table.printstd();
            printline();
            let output = get_build_dir() + "/table_total.csv";
            create_csv(&table, &output);
        }
        Err(e) => {
            println!("");
            printline();
            println!("Failed to read APK information: {}", e);
            printline();
        }
    }
}

fn printline() {
    println!("##########################################################################################");
}

fn println_message(messge: &str) {
    println!("");
    printline();
    println!("{}", messge);
    printline();
}

fn create_csv(table: &Table, output: &str) {
    let mut wtr = Writer::from_writer(File::create(output).expect("Cannot create file"));
    for row in table.row_iter() {
        let v: Vec<String> = row.iter().map(|cell| cell.get_content()).collect();
        wtr.write_record(&v).expect("Cannot write record");
    }
    wtr.flush().expect("Cannot flush");
}

fn get_current_dir() -> String {
    let project_path = env::current_dir().unwrap();
    return project_path.display().to_string();
}

fn get_build_dir() -> String {
    let project_path = env::current_dir().unwrap();
    let build_path = project_path.join("build");
    return build_path.display().to_string();
}

fn build_file(filename: &str) -> String {
    let project_path = env::current_dir().unwrap();
    let build_path = project_path.join("build");
    let apk_path = build_path.join(filename).to_str().unwrap().to_string();
    return apk_path;
}

fn main() {
    // 解析输入的命令
    let args: Vec<String> = std::env::args().collect();
    // 读取工程根目录
    let current_path = get_current_dir();
    // 根目录下面的build文件
    let build_path = get_build_dir();
    // 解析过滤器
    let filter = CliperFilter::from_args();
    // build目录下的apk文件
    if filter.input.is_empty() {
        println!("Please input the apk file");
        return;
    }
    // let apk_path = build_file("app.apk");
    let mut apk_path = filter.input.clone();
    if apk_path.starts_with(".") {
        apk_path = format!("{}/{}", &current_path, &apk_path);
    }

    let mut system_message = String::from("");
    system_message.push_str(format!("args: {:?}", args).as_str());
    system_message.push_str(format!("\nCurrent Path: {}", current_path).as_str());
    system_message.push_str(format!("\nBuild Path: {}", build_path).as_str());
    system_message.push_str(format!("\ninput Path: {}", apk_path).as_str());
    println_message(system_message.as_str());

    task::block_on(read_info(&apk_path));

    task::block_on(read_total(&apk_path));

    

    task::block_on(read_detail_info(&apk_path, &filter));
}
