use iced::{
    Application, Command, Element, Settings, Theme,
};
use iced::widget::{
    button, column, container, row, scrollable, text, text_input, Space,
};
use iced::Length;
use std::path::PathBuf;
use async_std::sync::Mutex;
use std::sync::Arc; // 确保导入了 std::sync::Arc
use rfd::AsyncFileDialog;
use directories::UserDirs;

mod data;
mod file_management;
mod mock_api;

use data::FileEntry;
use file_management::{load_file_entries, save_file_entries, get_data_dir};
use mock_api::MockApi;

// 定义应用程序的状态
#[derive(Debug, Default)]
pub struct WalrusStore {
    pub files: Vec<FileEntry>,
    pub upload_progress: f32,
    pub upload_file_path: String,
    pub download_id_input: String,
    pub status_message: String,
    // 用于模拟异步操作
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
    // 移除了 type Renderer = Renderer;

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
        match message {
            Message::FilesLoaded(files) => {
                self.files = files;
                Command::none()
            }
            Message::TriggerFileSelection => {
                Command::perform(
                    async {
                        // 这是一个异步操作，会打开文件选择对话框
                        let initial_directory = UserDirs::new()
                            .and_then(|user_dirs| Some(user_dirs.home_dir().to_path_buf()))
                            .unwrap_or_else(|| PathBuf::from("."));

                        let pick_result = rfd::AsyncFileDialog::new()
                            .set_directory(initial_directory)
                            .pick_file()
                            .await;
                        // 将选择的结果发送回 Message::FileSelected
                        Message::FileSelected(pick_result.map(|handle| handle.path().to_path_buf()))
                    },
                    |msg| msg, // 这里只是简单地将 Message::FileSelected 传回 update
                )
            }
            Message::FileSelected(path_opt) => {
                if let Some(path) = path_opt {
                    self.upload_file_path = path.to_string_lossy().into_owned();
                    self.status_message = format!("已选择文件: {}", self.upload_file_path);
                } else {
                    self.upload_file_path = String::new();
                    self.status_message = "未选择文件。".into();
                }
                Command::none()
            }
            Message::UploadButtonPressed => {
                if self.upload_file_path.is_empty() {
                    self.status_message = "请先选择一个文件。".into();
                    return Command::none();
                }
                let file_path = PathBuf::from(&self.upload_file_path);
                let file_name = file_path.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned();

                self.status_message = format!("正在上传 {}...", file_name);
                self.upload_progress = 0.0;

                // 异步上传文件
                Command::perform(
                    async { MockApi::upload_file(file_path).await },
                    |result| {
                        match result {
                            Ok(id) => Message::UploadComplete(Ok(FileEntry::new(file_name))),
                            Err(e) => Message::UploadComplete(Err(e)),
                        }
                    },
                )
            }
            Message::DownloadButtonPressed(id) => {
                let file_entry = self.files.iter().find(|f| f.id == id).cloned();
                if let Some(entry) = file_entry {
                    self.status_message = format!("正在下载 {}...", entry.name);
                    Command::perform(
                        async move { MockApi::download_file(entry.id.clone(), entry.name.clone()).await },
                        |result| Message::DownloadComplete(result),
                    )
                } else {
                    self.status_message = format!("找不到文件 ID: {}", id);
                    Command::none()
                }
            }
            Message::DeleteButtonPressed(id) => {
                let file_name = self.files.iter().find(|f| f.id == id).map(|f| f.name.clone());
                if let Some(name) = file_name {
                    self.status_message = format!("正在删除 {}...", name);
                    Command::perform(
                        async move { MockApi::delete_file(id.clone()).await },
                        |result| Message::DeleteComplete(result.map(|_id| _id)),
                    )
                } else {
                    self.status_message = format!("找不到文件 ID: {}", id);
                    Command::none()
                }
            }
            Message::DownloadInputChanged(id) => {
                self.download_id_input = id;
                Command::none()
            }
            Message::DownloadFromInputButtonPressed => {
                if self.download_id_input.is_empty() {
                    self.status_message = "请输入要下载的文件 ID。".into();
                    return Command::none();
                }
                let id_to_download = self.download_id_input.clone();
                let file_entry = self.files.iter().find(|f| f.id == id_to_download).cloned();

                if let Some(entry) = file_entry {
                    self.status_message = format!("正在下载 {} (ID: {})...", entry.name, id_to_download);
                    Command::perform(
                        async move { MockApi::download_file(entry.id.clone(), entry.name.clone()).await },
                        |result| Message::DownloadComplete(result),
                    )
                } else {
                    self.status_message = format!("找不到文件 ID: {}", id_to_download);
                    Command::none()
                }
            }
            Message::UploadProgress(progress) => {
                self.upload_progress = progress;
                Command::none()
            }
            Message::UploadComplete(result) => {
                self.upload_progress = 0.0; // 重置进度条
                match result {
                    Ok(entry) => {
                        self.files.push(entry.clone());
                        save_file_entries(&self.files);
                        self.status_message = format!("上传成功，ID: {}", entry.id);
                        self.upload_file_path = String::new(); // 清空选择的文件路径
                    }
                    Err(e) => {
                        self.status_message = format!("上传失败: {}", e);
                    }
                }
                Command::none()
            }
            Message::DownloadComplete(result) => {
                match result {
                    Ok(path) => {
                        self.status_message = format!("下载成功到: {}", path);
                    }
                    Err(e) => {
                        self.status_message = format!("下载失败: {}", e);
                    }
                }
                Command::none()
            }
            Message::DeleteComplete(result) => {
                match result {
                    Ok(deleted_id) => {
                        self.files.retain(|f| f.id != deleted_id);
                        save_file_entries(&self.files);
                        self.status_message = format!("文件已删除，ID: {}", deleted_id);
                    }
                    Err(e) => {
                        self.status_message = format!("删除失败: {}", e);
                    }
                }
                Command::none()
            }
            Message::StatusMessage(msg) => {
                self.status_message = msg;
                Command::none()
            }
            Message::NoOp => Command::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        let title_bar = container(
            text("我的网盘应用")
                .size(30)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .padding(10)
        .center_y();

        // 上传区域
        let upload_area = column![
            row![
                button("选择文件").on_press(Message::TriggerFileSelection),
                text_input("文件路径", &self.upload_file_path)
                    .on_input(|_| Message::NoOp) // 只读
                    .width(Length::FillPortion(2)),
            ]
            .spacing(10),
            button("上传文件").on_press(Message::UploadButtonPressed),
            // 进度条 (占位符)
            text(format!("上传进度: {:.0}%", self.upload_progress * 100.0)),
        ]
        .spacing(10)
        .padding(10)
        .width(Length::Fill);

        // 文件列表区域
        let file_list_header = row![
            text("文件名").width(Length::FillPortion(3)),
            text("文件 ID").width(Length::FillPortion(2)),
            text("上传时间").width(Length::FillPortion(2)),
            text("操作").width(Length::FillPortion(2)),
        ]
        .spacing(10)
        .padding(5);

        let file_list_items: Vec<Element<Message>> = self
            .files
            .iter()
            .map(|file| {
                row![
                    text(&file.name).width(Length::FillPortion(3)),
                    text(&file.id).width(Length::FillPortion(2)),
                    text(&file.uploaded_at).width(Length::FillPortion(2)),
                    row![
                        button("下载").on_press(Message::DownloadButtonPressed(file.id.clone())),
                        button("删除").on_press(Message::DeleteButtonPressed(file.id.clone())),
                    ]
                    .spacing(5)
                    .width(Length::FillPortion(2)),
                ]
                .spacing(10)
                .padding(5)
                .into()
            })
            .collect();

        let file_list_scrollable = scrollable(
            column(file_list_items).spacing(5)
        )
        .width(Length::Fill)
        .height(Length::FillPortion(3));

        let file_list_area = column![
            file_list_header,
            file_list_scrollable,
        ]
        .spacing(10)
        .padding(10)
        .width(Length::Fill);


        // 下载区域
        let download_area = column![
            text_input("输入文件 ID", &self.download_id_input)
                .on_input(Message::DownloadInputChanged)
                .padding(10)
                .width(Length::Fill),
            button("从 ID 下载").on_press(Message::DownloadFromInputButtonPressed),
        ]
        .spacing(10)
        .padding(10)
        .width(Length::Fill);

        // 状态栏
        let status_bar = container(
            text(&self.status_message)
                .size(16)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .padding(5)
        .center_x();


        column![
            title_bar,
            upload_area,
            Space::with_height(Length::Fixed(20.0)),
            file_list_area,
            Space::with_height(Length::Fixed(20.0)),
            download_area,
            status_bar,
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

pub fn main() -> iced::Result {
    // 检查并创建数据目录，确保在应用启动时存在
    get_data_dir();

    WalrusStore::run(Settings::default())
}
