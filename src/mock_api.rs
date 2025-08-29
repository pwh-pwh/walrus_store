use std::{path::PathBuf, fs};
use uuid::Uuid;
use crate::file_management::get_data_dir;

pub struct MockApi;

impl MockApi {
    pub async fn upload_file(file_path: PathBuf) -> Result<String, String> {
        println!("模拟上传文件: {:?}", file_path.display());
        async_std::task::sleep(std::time::Duration::from_secs(2)).await; // 模拟网络延迟
        if file_path.file_name().unwrap_or_default().to_string_lossy().contains("error") {
            Err(format!("上传失败：模拟错误发生在文件: {}", file_path.display()))
        } else {
            Ok(Uuid::new_v4().to_string())
        }
    }

    pub async fn download_file(file_id: String, file_name: String) -> Result<String, String> {
        println!("模拟下载文件，ID: {}", file_id);
        async_std::task::sleep(std::time::Duration::from_secs(1)).await; // 模拟网络延迟
        if file_id.contains("error") {
            Err(format!("下载失败：模拟错误发生，ID: {}", file_id))
        } else {
            let download_dir = get_data_dir().join("downloads");
            fs::create_dir_all(&download_dir).map_err(|e| format!("无法创建下载目录: {}", e))?;
            let download_path = download_dir.join(&file_name);
            fs::write(&download_path, format!("This is content for file ID: {}", file_id))
                .map_err(|e| format!("无法写入下载文件: {}", e))?;
            Ok(download_path.to_string_lossy().into_owned())
        }
    }

    pub async fn delete_file(file_id: String) -> Result<String, String> {
        println!("模拟删除文件，ID: {}", file_id);
        async_std::task::sleep(std::time::Duration::from_secs(0)).await; // 模拟网络延迟
        if file_id.contains("error") {
            Err(format!("删除失败：模拟错误发生，ID: {}", file_id))
        } else {
            Ok(file_id)
        }
    }
}