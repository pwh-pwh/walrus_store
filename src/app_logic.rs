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
            let id_to_download = app_state.download_id_input.clone();
            if id_to_download.is_empty() {
                app_state.status_message = "请输入要下载的文件 ID。".into();
                return Command::none();
            }
            // 触发文件选择对话框，并将下载ID传递给后续处理
            Command::perform(
                async move { id_to_download },
                |id| Message::TriggerDownloadSelectionFromInput(id),
            )
        }
        Message::TriggerDownloadSelectionFromInput(id) => Command::perform(
            async {
                let initial_directory = UserDirs::new()
                    .and_then(|user_dirs| user_dirs.download_dir().map(|path| path.to_path_buf()))
                    .unwrap_or_else(|| PathBuf::from("."));

                let pick_result = AsyncFileDialog::new()
                    .set_directory(initial_directory)
                    .pick_folder()
                    .await;
                // 注意：这里需要传递下载的文件名。由于我们只知道 ID，需要从文件条目中获取或在API中处理
                // 暂时使用一个占位符，实际可能需要额外的API调用来获取文件名
                Message::DownloadLocationSelectedFromInput(pick_result.map(|handle| handle.path().to_path_buf()), id)
            },
            |msg| msg,
        ),
        Message::DownloadLocationSelectedFromInput(path_opt, id_to_download) => {
            if let Some(download_path) = path_opt {
                // 现在我们需要从Walrus API获取文件名，或者直接使用一个默认文件名
                // 简化处理：假设API能根据ID返回文件数据，但文件名需要手动指定或从API结果中提取
                // 这里我们暂时使用一个placeholder，或者尝试从本地已存在的文件列表中查找
                let mut file_name = format!("downloaded_file_{}", id_to_download); // 默认文件名

                // 尝试从本地文件列表中查找文件名
                if let Some(entry) = app_state.files.iter().find(|f| f.id == id_to_download) {
                    file_name = entry.name.clone();
                }

                app_state.status_message = format!("正在下载文件 (ID: {}) 到 {}...", id_to_download, download_path.to_string_lossy());
                let walrus_api = WalrusApi::default();
                Command::perform(
                    async move { walrus_api.download_file(id_to_download.clone(), file_name, download_path).await },
                    |result| Message::DownloadComplete(result),
                )
            } else {
                app_state.status_message = "未选择下载路径。".into();
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
