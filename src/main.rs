// #![windows_subsystem = "windows"]

use iced::{Application, Command, Element, Font, Settings, Theme};

mod app_logic; // Add this line
mod data;
mod file_management;
// mod mock_api; // 注释掉或删除，因为我们将使用 walrus_api
mod ui;
mod walrus_api;

use crate::data::FileEntry;
use crate::file_management::load_file_entries;
// use crate::mock_api::MockApi; // 注释掉或删除
use crate::ui::view_application;
use app_logic::handle_message; // Add this line
use file_management::get_data_dir;
use std::path::PathBuf;

// 定义应用程序的状态
#[derive(Debug, Default)]
pub struct WalrusStore {
    pub files: Vec<FileEntry>,
    pub upload_progress: f32,
    pub upload_file_path: String,
    pub download_id_input: String,
    pub status_message: String,
    pub search_input: String, // 用于文件搜索的输入
}

// 定义应用程序的消息
#[derive(Debug, Clone)]
pub enum Message {
    FilesLoaded(Vec<FileEntry>),
    TriggerFileSelection,
    FileSelected(Option<PathBuf>),
    UploadButtonPressed,
    DownloadButtonPressed(String),
    TriggerDownloadSelection(String),
    DownloadLocationSelected(Option<PathBuf>, String),
    CopyIdToClipboard(String),
    DeleteButtonPressed(String),
    DownloadInputChanged(String),
    DownloadFromInputButtonPressed,
    TriggerDownloadSelectionFromInput(String), // 用于从输入框下载时选择路径
    DownloadLocationSelectedFromInput(Option<PathBuf>, String), // 从输入框下载后选择路径
    UploadProgress(f32),
    UploadComplete(Result<FileEntry, String>),
    DownloadComplete(Result<String, String>),
    DeleteComplete(Result<String, String>),
    StatusMessage(String),
    SearchInputChanged(String), // 用于文件搜索输入框变化的事件
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
                search_input: String::new(), // 初始化搜索输入为空
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("walrus云盘")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        handle_message(self, message)
    }

    fn view(&self) -> Element<'_, Message> {
        view_application(
            &self.files,
            &self.upload_file_path,
            self.upload_progress,
            &self.download_id_input,
            &self.status_message,
            &self.search_input, // 添加 search_input 参数
        )
    }
}

pub fn main() -> iced::Result {
    get_data_dir();
    let mut config = Settings::default();
    config.default_font = Font::with_name("微软雅黑");
    WalrusStore::run(config)
}
