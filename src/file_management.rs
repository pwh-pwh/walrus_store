use std::{
    fs,
    path::PathBuf,
};
use directories::ProjectDirs;
use crate::data::FileEntry;
use std::env;

pub fn get_data_dir() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("com", "kilocode", "WalrusStore") {
        let data_dir = proj_dirs.data_dir().to_path_buf();
        fs::create_dir_all(&data_dir).expect("无法创建数据目录");
        data_dir
    } else {
        // 如果无法获取用户数据目录，则使用当前工作目录
        eprintln!("无法获取用户数据目录，使用当前工作工作目录作为替代。");
        // 尝试获取当前可执行文件的路径，然后获取其父目录作为基础路径
        let current_exe_path = env::current_exe().expect("无法获取当前可执行文件路径");
        let base_dir = current_exe_path.parent().expect("无法获取父目录");
        let data_dir = base_dir.join("data");
        fs::create_dir_all(&data_dir).expect("无法创建数据目录");
        data_dir
    }
}

fn get_files_json_path() -> PathBuf {
    get_data_dir().join("files.json")
}

pub fn load_file_entries() -> Vec<FileEntry> {
    let path = get_files_json_path();
    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|e| {
                eprintln!("解析 files.json 失败: {}", e);
                Vec::new()
            }),
            Err(e) => {
                eprintln!("读取 files.json 失败: {}", e);
                Vec::new()
            }
        }
    } else {
        Vec::new()
    }
}

pub fn save_file_entries(entries: &[FileEntry]) {
    let path = get_files_json_path();
    match serde_json::to_string_pretty(entries) {
        Ok(json) => {
            fs::write(&path, json).expect("无法写入 files.json");
        },
        Err(e) => {
            eprintln!("序列化文件列表失败: {}", e);
        }
    }
}