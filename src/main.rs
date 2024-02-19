use std::fs::File;
use std::{env, fs};

use async_std::task;
use csv::Writer;
use prettytable::{row, Cell, Row, Table};
use std::collections::HashMap;
use std::path::Path;
use structopt::StructOpt;

mod app;
use app::{apk_info::ApkParsedInfo, manifest_parser::parser};
mod cliper;
use cliper::cmds::{Args, CommonOpts, DetailOpts};
use cliper::{apk_cliper::size_reader, cliper_info::CliperInfo};

// 添加一个过滤器，过滤掉不需要的文件, 满足条件的返回true
fn cliper_filter(info: &CliperInfo, filter: &DetailOpts) -> bool {
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

async fn read_info(filename: &str) -> ApkParsedInfo {
    match parser::parse(&filename).await {
        Some(value) => {
            let message = format!(
                "APK Information:\nFile: {}\nPackage Name: {}\nVersion Code: {}\nVersion Name: {}",
                filename, value.package_name, value.version_code, value.version_name
            );
            println_message(message.as_str());
            return value;
        }
        None => {
            println!("");
            printline();
            println!("Failed to read APK information");
            printline();
            return ApkParsedInfo::new();
        }
    }
}

async fn read_total(filename: &str, filter: &CommonOpts) {
    read_info(&filename).await;
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
            if filter.output_csv {
                let output = build_file("csv/table_total.csv");
                create_csv(&table, &output);
            }
        }
        Err(e) => {
            println!("");
            printline();
            println!("Failed to read APK information: {}", e);
            printline();
        }
    }
}

async fn read_detail_info(filename: &str, filter: &CommonOpts, detail: &DetailOpts) {
    read_info(&filename).await;
    match size_reader::read_detail_info(filename) {
        Ok(value) => {
            // 对value进行排序，以donwload大小进行排序
            let mut value = value;
            value.sort_by(|a, b| b.download.cmp(&a.download));

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
                if !cliper_filter(cliper_item, detail) {
                    continue;
                }
                line_num += 1;
                table.add_row(Row::new(vec![
                    // Cell::new(&cliper_item.id.to_string()),
                    Cell::new(&line_num.to_string()),
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
            let limit = detail.limit;
            if limit <= 0 || limit >= line_num {
                table.printstd();
            } else {
                let mut limited_table = Table::new();
                for row in table.row_iter().take(limit + 1) {
                    limited_table.add_row(Row::new(
                        row.iter()
                            .map(|cell| Cell::new(&cell.get_content()))
                            .collect(),
                    ));
                }
                limited_table.printstd();
            }
            printline();
            if filter.output_csv {
                let output = build_file("csv/table_detail.csv");
                create_csv(&table, &output);
            }
        }
        Err(e) => {
            println!("");
            printline();
            println!("Failed to read APK information: {}", e);
            printline();
        }
    }
}

#[derive(Debug)]
pub struct Md5SizeInfo {
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

pub struct Md5Group {
    pub md5: String,
    pub file_names: String,
    pub size: u64,
}

fn group_by_md5(data: Vec<CliperInfo>) -> HashMap<String, Vec<Md5SizeInfo>> {
    let mut md5_map: HashMap<String, Vec<Md5SizeInfo>> = HashMap::new();
    // 根据 md5 值获取对应的向量，如果不存在则插入一个新的空向量
    for cliper_info in data {
        let file_names = md5_map.entry(cliper_info.md5.clone()).or_insert_with(Vec::new);   
        let md5_size_info = Md5SizeInfo {
            id: cliper_info.id,
            file_path: cliper_info.file_path,
            name: cliper_info.name,
            size: cliper_info.size,
            download: cliper_info.download,
            file_type: cliper_info.file_type,
            file_ext: cliper_info.file_ext,
            file_folder: cliper_info.file_folder,
            md5: cliper_info.md5.clone(),
        };
        // 将文件名添加到向量中
        file_names.push(md5_size_info);
    }

    md5_map
}

async fn read_same_info(filename: &str, filter: &CommonOpts) {
    read_info(&filename).await;
    match size_reader::read_detail_info_with_md5(filename) {
        Ok(value) => {
            // 对value进行排序，以donwload大小进行排序
            let md5_groups = group_by_md5(value);

            let md5_groups_convert = md5_groups
                .into_iter()
                // 过滤 md5 值只有一个文件的情况
                .filter_map(|(md5, file_names)| {
                    if file_names.len() == 1 {
                        return None;
                    }
                    Some((md5, file_names))
                })
                .map(|(md5, file_names)| {
                    let size = file_names.iter().map(|file_name| file_name.size).sum();
                    let file_names = file_names
                        .iter()
                        .map(|file_name| file_name.file_path.clone())
                        .collect::<Vec<String>>()
                        .join("\n");
                    Md5Group {
                        md5,
                        file_names,
                        size,
                    }
                })
                .collect::<Vec<Md5Group>>();

            // 按照size大小排序
            let mut md5_groups_convert = md5_groups_convert;
            md5_groups_convert.sort_by(|a, b| b.size.cmp(&a.size));

            let mut md5_table = Table::new();
            let mut md5_line_num = 0;
            md5_table.add_row(row!["id", "md5", "files", "size"]);


            // 打印出按 MD5 分组的文件名
            for item_info in md5_groups_convert {
                md5_line_num += 1;
                md5_table.add_row(Row::new(vec![
                    // Cell::new(&cliper_item.id.to_string()),
                    Cell::new(&md5_line_num.to_string()),
                    Cell::new(&item_info.md5),
                    Cell::new(&item_info.file_names),
                    Cell::new(&item_info.size.to_string()),
                ]));
            }
            // 按文件大小排序
            println!("");
            printline();
            println!("Total: {}, Filter: {}", &md5_table.len()-1, md5_line_num);
            md5_table.printstd();
            printline();
            if filter.output_csv {
                let output = build_file("csv/table_same.csv");
                create_csv(&md5_table, &output);
            }
            
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
    // 如果表格为空，直接返回
    if table.is_empty() {
        return;
    }
    // 如果文件存在，删除文件
    if fs::metadata(output).is_ok() {
        fs::remove_file(output).expect("Cannot remove file");
    }
    // 判断文件的父目录是否存在，不存在则创建
    let file_path = Path::new(output);
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Cannot create dir");
        }
    }
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
    return build_path.join(filename).to_str().unwrap().to_string();
}

fn check_input_file(filename: &str) -> Result<(), String> {
    if filename.is_empty() {
        println_message("Error: Please input the apk file: --input ./build/app.apk");
        return Err("Input file path is empty".to_string());
    }
    let file = Path::new(filename);
    if !file.exists() {
        println_message("Error: Please input the apk file: --input ./build/app.apk");
        return Err("Input file path not exists".to_string());
    }
    return Ok(());
}

fn absolute_path(input: &str) -> String {
    let mut file_path = input.to_string();
    if file_path.starts_with(".") {
        file_path = format!("{}/{}", get_current_dir(), file_path);
    }
    return file_path;
}

fn show_debug(debug: bool, apk_path: &str) {
    if !debug {
        return;
    }
    let mut system_message = String::from("");
    system_message.push_str(format!("args          : {:?}", env::args()).as_str());
    system_message.push_str(format!("\nCmd         : {}", "summary").as_str());
    system_message.push_str(format!("\nCurrent Path: {}", get_current_dir()).as_str());
    system_message.push_str(format!("\nBuild Path  : {}", get_build_dir()).as_str());
    system_message.push_str(format!("\ninput Path  : {}", apk_path).as_str());
    println_message(system_message.as_str());
}

fn main() -> Result<(), String> {
    if std::env::args().len() == 1 {
        Args::clap().print_help().expect("Failed to print help");
        println!(); // 打印换行符以更好地格式化输出
        return Ok(());
    }
    let args_from: Args = Args::from_args();
    // 匹配不同的命令来获取参数
    match args_from {
        Args::Summary { common } => {
            let mut opts = common;
            opts.build_path = get_build_dir();
            check_input_file(opts.input.as_str())?;
            let apk_path = absolute_path(&opts.input.clone());
            show_debug(opts.debug, apk_path.as_str());
            task::block_on(read_total(&apk_path, &opts));
        }
        Args::Detail { common, detail } => {
            let mut opts = common;
            opts.build_path = get_build_dir();
            check_input_file(opts.input.as_str())?;
            let apk_path = absolute_path(&opts.input.clone());
            show_debug(opts.debug, apk_path.as_str());
            task::block_on(read_detail_info(&apk_path, &opts, &detail));
        }
        Args::Same { common } => {
            let mut opts = common;
            opts.build_path = get_build_dir();
            check_input_file(opts.input.as_str())?;
            let apk_path = absolute_path(&opts.input.clone());
            show_debug(opts.debug, apk_path.as_str());
            task::block_on(read_same_info(&apk_path, &opts));
        }
    }
    Ok(())
}
