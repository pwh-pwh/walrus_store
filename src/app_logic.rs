use directories::UserDirs;
use iced::Command;
use rfd::AsyncFileDialog;
use std::path::PathBuf;

use crate::Message;
use crate::WalrusStore; // 需要引入 WalrusStore 结构体
use crate::data::FileEntry;
use crate::file_management::save_file_entries;
use crate::walrus_api::WalrusApi; // 引入 WalrusApi

pub fn handle_message(app_state: &mut WalrusStore, message: Message) -> Command<Message> {
    match message {
        Message::FilesLoaded(files) => {
            app_state.files = files;
            Command::none()
        }
        Message::TriggerFileSelection => Command::perform(
            async {
                let initial_directory = UserDirs::new()
                    .and_then(|user_dirs| Some(user_dirs.home_dir().to_path_buf()))
                    .unwrap_or_else(|| PathBuf::from("."));

                let pick_result = AsyncFileDialog::new()
                    .set_directory(initial_directory)
                    .pick_file()
                    .await;
                Message::FileSelected(pick_result.map(|handle| handle.path().to_path_buf()))
            },
            |msg| msg,
        ),
        Message::FileSelected(path_opt) => {
            if let Some(path) = path_opt {
                app_state.upload_file_path = path.to_string_lossy().into_owned();
                app_state.status_message = format!("已选择文件: {}", app_state.upload_file_path);
            } else {
                app_state.upload_file_path = String::new();
                app_state.status_message = "未选择文件。".into();
            }
            Command::none()
        }
        Message::UploadButtonPressed => {
            if app_state.upload_file_path.is_empty() {
                app_state.status_message = "请先选择一个文件。".into();
                return Command::none();
            }
            let file_path = PathBuf::from(&app_state.upload_file_path);
            let file_name = file_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned();

            app_state.status_message = format!("正在上传 {}...", file_name);
            app_state.upload_progress = 0.0;
 
            let walrus_api = WalrusApi::default(); // 创建 WalrusApi 实例
            Command::perform(
                async move { walrus_api.upload_file(file_path).await },
                move |result| match result {
                    Ok(id) => Message::UploadComplete(Ok(FileEntry::new(id, file_name.clone()))),
                    Err(e) => Message::UploadComplete(Err(e)),
                },
            )
        }
        Message::DownloadButtonPressed(id) => {
            Command::perform(
                async move { id },
                |id| Message::TriggerDownloadSelection(id),
            )
        }
        Message::TriggerDownloadSelection(id) => Command::perform(
            async {
                let initial_directory = UserDirs::new()
                    .and_then(|user_dirs| user_dirs.download_dir().map(|path| path.to_path_buf()))
                    .unwrap_or_else(|| PathBuf::from("."));

                let pick_result = AsyncFileDialog::new()
                    .set_directory(initial_directory)
                    .pick_folder()
                    .await;
                Message::DownloadLocationSelected(pick_result.map(|handle| handle.path().to_path_buf()), id)
            },
            |msg| msg,
        ),
        Message::DownloadLocationSelected(path_opt, id) => {
            if let Some(download_path) = path_opt {
                let file_entry = app_state.files.iter().find(|f| f.id == id).cloned();
                if let Some(entry) = file_entry {
                    app_state.status_message = format!("正在下载 {} 到 {}...", entry.name, download_path.to_string_lossy());
                    let walrus_api = WalrusApi::default(); // 创建 WalrusApi 实例
                    Command::perform(
                        async move { walrus_api.download_file(entry.id.clone(), entry.name.clone(), download_path).await },
                        |result| Message::DownloadComplete(result),
                    )
                } else {
                    app_state.status_message = format!("找不到文件 ID: {}", id);
                    Command::none()
                }
            } else {
                app_state.status_message = "未选择下载路径。".into();
                Command::none()
            }
        }
        Message::DeleteButtonPressed(id) => {
            if app_state.files.iter().any(|f| f.id == id) {
                // 用户要求delete file不用处理，只需要把这个配置文件的记录删掉即可
                app_state.files.retain(|f| f.id != id);
                save_file_entries(&app_state.files);
                app_state.status_message = format!("文件已从本地记录中删除，ID: {}", id);
                Command::none()
            } else {
                app_state.status_message = format!("找不到文件 ID: {}", id);
                Command::none()
            }
        }
        Message::DownloadInputChanged(id) => {
            app_state.download_id_input = id;
            Command::none()
        }
        Message::DownloadFromInputButtonPressed => {
            if app_state.download_id_input.is_empty() {
                app_state.status_message = "请输入要下载的文件 ID。".into();
                return Command::none();
            }
            let id_to_download = app_state.download_id_input.clone();
            let file_entry = app_state
                .files
                .iter()
                .find(|f| f.id == id_to_download)
                .cloned();

            if let Some(entry) = file_entry {
                app_state.status_message =
                    format!("正在下载 {} (ID: {})...", entry.name, id_to_download);
                Command::perform(
                    async move {
                        let walrus_api = WalrusApi::default(); // 创建 WalrusApi 实例
                        walrus_api.download_file(entry.id.clone(), entry.name.clone(), UserDirs::new().and_then(|user_dirs| user_dirs.download_dir().map(|path| path.to_path_buf())).unwrap_or_else(|| PathBuf::from("."))).await
                    }, // 使用默认下载目录
                    |result| Message::DownloadComplete(result),
                )
            } else {
                app_state.status_message = format!("找不到文件 ID: {}", id_to_download);
                Command::none()
            }
        }
        Message::UploadProgress(progress) => {
            app_state.upload_progress = progress;
            Command::none()
        }
        Message::UploadComplete(result) => {
            app_state.upload_progress = 0.0;
            match result {
                Ok(entry) => {
                    app_state.files.push(entry.clone());
                    save_file_entries(&app_state.files);
                    app_state.status_message = format!("上传成功，ID: {}", entry.id);
                    app_state.upload_file_path = String::new();
                }
                Err(e) => {
                    app_state.status_message = format!("上传失败: {}", e);
                }
            }
            Command::none()
        }
        Message::DownloadComplete(result) => {
            match result {
                Ok(path) => {
                    app_state.status_message = format!("下载成功到: {}", path);
                }
                Err(e) => {
                    app_state.status_message = format!("下载失败: {}", e);
                }
            }
            Command::none()
        }
        Message::DeleteComplete(result) => {
            match result {
                Ok(deleted_id) => {
                    app_state.files.retain(|f| f.id != deleted_id);
                    save_file_entries(&app_state.files);
                    app_state.status_message = format!("文件已删除，ID: {}", deleted_id);
                }
                Err(e) => {
                    app_state.status_message = format!("删除失败: {}", e);
                }
            }
            Command::none()
        }
        Message::StatusMessage(msg) => {
            app_state.status_message = msg;
            Command::none()
        }
        Message::CopyIdToClipboard(id) => {
            app_state.status_message = format!("文件 ID 已复制到剪贴板: {}", id);
            Command::perform(async move {
                let mut clipboard = arboard::Clipboard::new().unwrap();
                clipboard.set_text(id).unwrap();
                async_std::task::sleep(std::time::Duration::from_millis(100)).await;
            }, |_| Message::NoOp)
        }
        Message::NoOp => Command::none(),
    }
}
