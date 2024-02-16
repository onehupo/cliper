pub mod size_reader {
    use std::{fs::File, option};
    use zip::read::ZipArchive;
    use std::path::Path;

    use crate::cliper::size_data::SizeData;
    use crate::cliper::cliper_info::CliperInfo;

    pub fn read_size(filename: &str) -> zip::result::ZipResult<SizeData> {
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

        file_info.all = file_info.asserts
            + file_info.res
            + file_info.code
            + file_info.native
            + file_info.others;

        Ok(file_info)
    }

    /**
     * 读取文件详细信息
     * 路径，名称，压缩大小，原始大小，分类，文件类型，文件夹的路径
     */
    pub fn read_detail_info(
        filepath: &str,
    ) -> zip::result::ZipResult<Vec<CliperInfo>> {
        let file = File::open(filepath)?;

        // 读取apk文件,zip格式
        let mut archive = ZipArchive::new(file)?;
        // 存放文件信息 
        let mut cliper_info_list: Vec<CliperInfo> = Vec::new();

        for i in 0..archive.len() {
            let file = archive.by_index(i)?;
            let name = file.name().to_string();
            let size = file.size();
            let download = file.compressed_size();
            let file_type = read_type(&name);
            let file_ext = read_file_ext(&name);
            let path = Path::new(&name);
            let file_folder = path.parent().unwrap().to_str().unwrap().to_string();
            let file_name = read_file_name(&name);
            let cliper_info = create_cliper_item(
                i as u64,
                name,
                file_name,
                size,
                download,
                file_type,
                file_ext,
                file_folder,
            );
            cliper_info_list.push(cliper_info);
        }

        Ok(cliper_info_list)
    }

    /**
     * 读取文件类型
     */
    fn read_type(name: &str) -> String {
        let mut file_type = String::new();
        if name.starts_with("assets/") || name.starts_with("base/assets/") {
            file_type = "Asserts".to_string();
        } else if name.starts_with("res/") || name.starts_with("base/res/") {
            file_type = "Res".to_string();
        } else if name.starts_with("resources.arsc") || name.ends_with("resources.arsc") {
            file_type = "Res".to_string();
        } else if name.starts_with("classes") || name.starts_with("base/dex/classes") {
            file_type = "Code".to_string();
        } else if name.starts_with("lib/") || name.starts_with("base/lib") {
            file_type = "Native".to_string();
        } else {
            file_type = "Others".to_string();
        }
        return file_type;
    }

    /**
     * 读取文件扩展名
     */
    fn read_file_ext(name: &str) -> String {
        let mut file_ext = String::new();
        // 获取最后一个.的位置
        let option = name.rfind('.');
        if option.is_none() {
            return file_ext;
        }
        let index = option.unwrap();
        if index == 0 {
            return file_ext;
        }
        file_ext = name[index..].to_string();
        return file_ext;
    }

    /**
     * 读取文件名称
     */
    fn read_file_name(name: &str) -> String {
        let option = name.rfind('/');
        if let Some(index) = option {
            // Add 1 to index to skip the '/'
            let file_name = &name[index + 1..];
            file_name.to_string()
        } else {
            String::new()
        }
    }

    /**
     * 创建一个文件信息
     */
    fn create_cliper_item(
        id: u64,
        file_path: String,
        name: String,
        size: u64,
        download: u64,
        file_type: String,
        file_ext: String,
        file_folder: String,
    ) -> CliperInfo {
        let mut cliper_info = CliperInfo::new();
        cliper_info.id = id;
        cliper_info.file_path = file_path;
        cliper_info.name = name;
        cliper_info.size = size;
        cliper_info.download = download;
        cliper_info.file_type = file_type;
        cliper_info.file_ext = file_ext;
        cliper_info.file_folder = file_folder;
        return cliper_info;
    }
}
