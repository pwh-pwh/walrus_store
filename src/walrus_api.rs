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
        
        Ok("".to_string())
    }
}
