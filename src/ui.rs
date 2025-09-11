use crate::Message;
use crate::data::FileEntry;
use iced::widget::{Space, button, column, container, row, scrollable, text, text_input};
use iced::{Element, Length};

pub fn view_application<'a>(
    files: &'a Vec<FileEntry>,
    upload_file_path: &'a str,
    upload_progress: f32,
    download_id_input: &'a str,
    status_message: &'a str,
    search_input: &'a str, // 添加搜索输入参数
    selected_files: &'a std::collections::HashSet<String>, // 新增
) -> Element<'a, Message> {
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
            text_input("文件路径", upload_file_path)
                .on_input(|_| Message::NoOp) // 只读
                .width(Length::FillPortion(2)),
        ]
        .spacing(10),
        button("上传文件").on_press(Message::UploadButtonPressed),
        // 进度条 (占位符)
        text(format!("上传进度: {:.0}%", upload_progress * 100.0)),
    ]
    .spacing(10)
    .padding(10)
    .width(Length::Fill);

    // 文件搜索输入框
    let search_input_widget = text_input("搜索文件...", search_input)
        .on_input(Message::SearchInputChanged)
        .padding(10)
        .width(Length::Fill);

    // 文件列表区域
    let file_list_header = row![
        text("").width(Length::Fixed(20.0)), // 复选框的占位
        text("文件名").width(Length::FillPortion(3)),
        text("文件 ID").width(Length::FillPortion(2)),
        text("上传时间").width(Length::FillPortion(2)),
        text("操作").width(Length::FillPortion(2)),
    ]
    .spacing(10)
    .padding(5);

    let filtered_files = files
        .iter()
        .filter(|file| {
            file.name.to_lowercase().contains(&search_input.to_lowercase()) ||
            file.id.to_lowercase().contains(&search_input.to_lowercase())
        })
        .collect::<Vec<_>>();

    let file_list_items: Vec<Element<'a, Message>> = filtered_files
        .iter()
        .map(|file_ref| {
            let file_id_clone = file_ref.id.clone();
            let file_name_clone = file_ref.name.clone();
            let uploaded_at_clone = file_ref.uploaded_at.clone();
            let display_id_clone = if file_id_clone.len() > 10 {
                format!("{}...", &file_id_clone[0..10])
            } else {
                file_id_clone.clone()
            };

            let checkbox = iced::widget::checkbox(
                "", // label
                selected_files.contains(&file_id_clone), // is_checked
            )
            .on_toggle(move |is_checked| Message::FileSelectedForBatch(file_id_clone.clone(), is_checked)) // 链式调用 on_toggle
            .width(Length::Fixed(20.0));

            row![
                checkbox,
                text(file_name_clone).width(Length::FillPortion(3)),
                text(display_id_clone).width(Length::FillPortion(2)),
                text(uploaded_at_clone).width(Length::FillPortion(2)),
                row![
                    button("复制 ID").on_press(Message::CopyIdToClipboard(file_ref.id.clone())),
                    button("下载").on_press(Message::DownloadButtonPressed(file_ref.id.clone())),
                    button("删除").on_press(Message::DeleteButtonPressed(file_ref.id.clone())),
                ]
                .spacing(5)
                .width(Length::FillPortion(2)),
            ]
            .spacing(10)
            .padding(5)
            .into()
        })
        .collect();

    let file_list_scrollable = scrollable(column(file_list_items).spacing(5))
        .width(Length::Fill)
        .height(Length::FillPortion(3));

    let file_list_area = column![file_list_header, file_list_scrollable,]
        .spacing(10)
        .padding(10)
        .width(Length::Fill);

    // 批量操作区域
    let batch_actions_area = if selected_files.is_empty() {
        row![]
    } else {
        row![
            button("批量删除").on_press(Message::BatchDeleteButtonPressed),
            button("批量下载").on_press(Message::BatchDownloadButtonPressed),
        ]
        .spacing(10)
        .padding(10)
        .width(Length::Fill)
    };

    // 下载区域
    let download_area = column![
        text_input("输入文件 ID", download_id_input)
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
        text(status_message)
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
        search_input_widget, // 添加搜索输入框
        file_list_area,
        Space::with_height(Length::Fixed(20.0)),
        batch_actions_area, // 添加批量操作区域
        download_area,
        status_bar,
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
