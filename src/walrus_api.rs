use std::{default, fs, path::PathBuf};

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
    fn new(agg_url: String, pub_url: String) -> Self {
        Self {
            client: WalrusClient::new(&agg_url, &pub_url).unwrap(),
        }
    }

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

    
}