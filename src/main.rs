use iced::{
    Application, Command, Element, Font, Settings, Theme
};

mod data;
mod file_management;
mod mock_api;
mod ui;
mod app_logic; // Add this line

use file_management::get_data_dir;
use app_logic::handle_message; // Add this line
use crate::data::FileEntry;
use crate::file_management::{load_file_entries, save_file_entries};
use crate::mock_api::MockApi;
use crate::ui::view_application;
use std::path::PathBuf;
use async_std::sync::Mutex;
use std::sync::Arc;

// 定义应用程序的状态
#[derive(Debug, Default)]
pub struct WalrusStore {
    pub files: Vec<FileEntry>,
    pub upload_progress: f32,
    pub upload_file_path: String,
    pub download_id_input: String,
    pub status_message: String,
    #[allow(dead_code)]
    pub api_task: Option<Arc<Mutex<Option<async_std::task::JoinHandle<()>>>>>,
}

// 定义应用程序的消息
#[derive(Debug, Clone)]
pub enum Message {
    FilesLoaded(Vec<FileEntry>),
    TriggerFileSelection,
    FileSelected(Option<PathBuf>),
    UploadButtonPressed,
    DownloadButtonPressed(String),
    DeleteButtonPressed(String),
    DownloadInputChanged(String),
    DownloadFromInputButtonPressed,
    UploadProgress(f32),
    UploadComplete(Result<FileEntry, String>),
    DownloadComplete(Result<String, String>),
    DeleteComplete(Result<String, String>),
    StatusMessage(String),
    NoOp,
}

impl Application for WalrusStore {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (WalrusStore, Command<Message>) {
        (
            WalrusStore {
                files: load_file_entries(),
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("我的网盘应用")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        handle_message(self, message)
    }

    fn view(&self) -> Element<Message> {
        view_application(
            &self.files,
            &self.upload_file_path,
            self.upload_progress,
            &self.download_id_input,
            &self.status_message,
        )
    }
}

pub fn main() -> iced::Result {
    get_data_dir();
    let mut config = Settings::default();
    config.default_font = Font::with_name("微软雅黑");
    WalrusStore::run(config)
}

