use std::{fs, path::PathBuf};

use walrus_rs::WalrusClient;

const AGG_URL: &'static str = "https://aggregator.testnet.walrus.atalma.io";
const PUB_URL: &'static str = "https://publisher.walrus-01.tududes.com";

pub struct WalrusApi {
    client: WalrusClient,
}

impl Default for WalrusApi {
    fn default() -> Self {
        Self {
            client: WalrusClient::new(AGG_URL, PUB_URL).unwrap(),
        }
    }
}

impl WalrusApi {

    pub async fn upload_file(&self,file_path: PathBuf) -> Result<String, String> {
        println!("上传文件路径: {:?}", file_path.display());
        let data = fs::read(&file_path).map_err(|e|e.to_string())?;
        let result = self.client.store_blob(data, Some(1), None, None, None).await.map_err(|e|e.to_string())?;
        if result.newly_created.is_none() {
            Ok(result.already_certified.unwrap().blob_id)
        } else {
            Ok(result.newly_created.unwrap().blob_object.blob_id)
        }
    }

    pub async fn download_file(&self,blob_id:String,file_name: String, download_dir: PathBuf) -> Result<String,String> {
        let data = self.client.read_blob_by_id(&blob_id).await.map_err(|e|e.to_string())?;
        fs::create_dir_all(&download_dir).map_err(|e| format!("无法创建下载目录: {}", e))?;
        let download_path = download_dir.join(&file_name);
        fs::write(download_path, data).map_err(|e|e.to_string())?;
        Ok("Ok".to_string())
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[tokio::test]
    async fn test_upload() {
        let pb = PathBuf::from_str("E:\\dev\\walrus_store\\Cargo.toml").unwrap();
        let walrus_api = WalrusApi::default();
        let result = walrus_api.upload_file(pb).await;
        println!("result: {:?}",result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_download() {
        let file_name = "Test.toml".to_string();
        let download_dir = PathBuf::from_str("E:\\dev\\walrus_store").unwrap(); // 指定一个下载目录
        let walrus_api = WalrusApi::default();
        let result = walrus_api.download_file("Gt72sjsONf_6ySL1Mzrxbjl5_WgEWDRjTWhxN8fBeus".to_string(), file_name, download_dir).await;
        println!("result: {:?}",result);
        assert!(result.is_ok());
    }

    
}