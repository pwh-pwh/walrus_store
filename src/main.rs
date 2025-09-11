#![windows_subsystem = "windows"]

use iced::{Application, Command, Element, Font, Settings, Theme};
use std::collections::HashSet;

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
    pub selected_files: std::collections::HashSet<String>, // 新增，用于存储选中的文件ID
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
    FileSelectedForBatch(String, bool), // 用于批量操作中选择/取消选择文件 (文件ID, 是否选中)
    BatchDeleteButtonPressed,           // 批量删除按钮
    BatchDownloadButtonPressed,         // 批量下载按钮
    TriggerBatchDownloadSelection,      // 触发批量下载的路径选择
    BatchDownloadLocationSelected(Option<PathBuf>), // 批量下载路径选择完成
    DownloadInputChanged(String),
    DownloadFromInputButtonPressed,
    TriggerDownloadSelectionFromInput(String), // 用于从输入框下载时选择路径
    DownloadLocationSelectedFromInput(Option<PathBuf>, String), // 从输入框下载后选择路径
    TriggerExportConfig,                       // 触发导出配置文件
    ExportConfigSelected(Option<PathBuf>),     // 导出配置文件路径选择完成
    TriggerImportConfig,                       // 触发导入配置文件
    ImportConfigSelected(Option<PathBuf>),     // 导入配置文件路径选择完成
    UploadConfigButtonPressed,                 // 新增：上传当前配置
    UploadConfigSuccess(String),               // 新增：配置上传成功，包含blob ID
    LoadConfigFromIdButtonPressed,             // 新增：加载远程配置按钮被按下
    ConfigLoaded(Result<String, String>),      // 新增：远程配置加载完成，包含配置内容
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
                search_input: String::new(),    // 初始化搜索输入为空
                selected_files: HashSet::new(), // 初始化选中的文件ID为空
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
            &self.search_input,   // 添加 search_input 参数
            &self.selected_files, // 新增，传递 selected_files
        )
    }

    fn theme(&self) -> Theme {
        Theme::Dark // 返回 Dark 主题
    }
}

pub fn main() -> iced::Result {
    get_data_dir();
    let config = Settings {
        default_font: Font::with_name("微软雅黑"),
        ..Default::default()
    };
    WalrusStore::run(config)
}
