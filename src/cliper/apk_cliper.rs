
pub mod size_reader {
    use std::fs::File;
    use zip::read::ZipArchive;

    use crate::cliper::size_data::SizeData;

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

        file_info.all = file_info.asserts + file_info.res + file_info.code + file_info.native + file_info.others;

        Ok(file_info)
    }


}