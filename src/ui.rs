use crate::Message;
use crate::data::FileEntry;
use iced::widget::{button, column, container, row, scrollable, text, text_input}; // 移除 checkbox 导入
use iced::{Element, Length, Color};

const SPACING: u16 = 10;
const PADDING: u16 = 10;

// Hacker theme colors (using a slightly desaturated green for better contrast on dark)
// const HACKER_BACKGROUND: Color = Color::from_rgb( // 移除 HACKER_BACKGROUND
//     0x0A as f32 / 255.0,
//     0x0A as f32 / 255.0,
//     0x0A as f32 / 255.0,
// ); // Darkest grey
const HACKER_FOREGROUND: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0xE6 as f32 / 255.0,
    0x00 as f32 / 255.0,
); // Green
const HACKER_ACCENT: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0xA0 as f32 / 255.0,
    0xA0 as f32 / 255.0,
); // Cyan
// const HACKER_RED: Color = Color::from_rgb(0xFF as f32 / 255.0, 0x33 as f32 / 255.0, 0x33 as f32 / 255.0); // Red for destructive actions // 移除 HACKER_RED
// const HACKER_GREY: Color = Color::from_rgb( // 移除 HACKER_GREY
//     0x33 as f32 / 255.0,
//     0x33 as f32 / 255.0,
//     0x33 as f32 / 255.0,
// ); // Dark grey for secondary elements


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
        text("Walrus云盘")
            .size(30)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .vertical_alignment(iced::alignment::Vertical::Center)
            .style(iced::theme::Text::Color(HACKER_FOREGROUND)), // 设置文本颜色为绿色
    )
    .width(Length::Fill)
    .padding(PADDING)
    .center_y()
    .style(iced::theme::Container::Box); // 使用 Box 样式，它在 Dark 主题下会有深色背景

    // 上传区域
    let upload_area = container(
        column![
            row![
                button("选择文件")
                    .on_press(Message::TriggerFileSelection)
                    .style(iced::theme::Button::Primary), // 使用 Primary 样式，它在 Dark 主题下有深色背景
                text_input("文件路径", upload_file_path)
                    .on_input(|_| Message::NoOp) // 只读
                    .width(Length::FillPortion(2))
                    .style(iced::theme::TextInput::Default), // 使用默认样式
            ]
            .spacing(SPACING),
            button("上传文件")
                .on_press(Message::UploadButtonPressed)
                .style(iced::theme::Button::Primary), // 使用 Primary 样式
            // 进度条 (占位符)
            text(format!("上传进度: {:.0}%", upload_progress * 100.0))
                .style(iced::theme::Text::Color(HACKER_FOREGROUND)), // 进度文本颜色
        ]
        .spacing(SPACING)
        .padding(PADDING)
        .width(Length::Fill),
    )
    .style(iced::theme::Container::Box) // 使用 Box 样式，适应 Dark 主题
    .padding(PADDING)
    .width(Length::Fill);

    // 文件搜索输入框
    let search_input_widget = text_input("搜索文件...", search_input)
        .on_input(Message::SearchInputChanged)
        .padding(PADDING)
        .width(Length::Fill)
        .style(iced::theme::TextInput::Default); // 使用默认样式，文本颜色将由全局主题控制

    // 文件列表区域
    let file_list_header = container(
        row![
            text("").width(Length::Fixed(20.0)), // 复选框的占位
            text("文件名").width(Length::FillPortion(3)).style(iced::theme::Text::Color(HACKER_ACCENT)),
            text("文件 ID").width(Length::FillPortion(2)).style(iced::theme::Text::Color(HACKER_ACCENT)),
            text("上传时间").width(Length::FillPortion(2)).style(iced::theme::Text::Color(HACKER_ACCENT)),
            text("操作").width(Length::FillPortion(2)).style(iced::theme::Text::Color(HACKER_ACCENT)),
        ]
        .spacing(SPACING)
        .padding(PADDING)
    )
    .style(iced::theme::Container::Box); // 添加背景和边框

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
            .width(Length::Fixed(20.0)); // 不设置自定义样式，让其继承全局 Dark 主题的默认外观

            container(
                row![
                    checkbox,
                    text(file_name_clone).width(Length::FillPortion(3)).style(iced::theme::Text::Color(HACKER_FOREGROUND)),
                    text(display_id_clone).width(Length::FillPortion(2)).style(iced::theme::Text::Color(HACKER_FOREGROUND)),
                    text(uploaded_at_clone).width(Length::FillPortion(2)).style(iced::theme::Text::Color(HACKER_FOREGROUND)),
                    row![
                        button("复制 ID")
                            .on_press(Message::CopyIdToClipboard(file_ref.id.clone()))
                            .style(iced::theme::Button::Text), // 使用 Text 样式
                        button("下载")
                            .on_press(Message::DownloadButtonPressed(file_ref.id.clone()))
                            .style(iced::theme::Button::Primary), // 使用 Primary 样式
                        button("删除")
                            .on_press(Message::DeleteButtonPressed(file_ref.id.clone()))
                            .style(iced::theme::Button::Destructive), // 使用 Destructive 样式
                    ]
                    .spacing(SPACING)
                    .width(Length::FillPortion(2)),
                ]
                .spacing(SPACING)
                .padding(PADDING),
            )
            .style(iced::theme::Container::Box) // 使用 Box 样式，适应 Dark 主题
            .into()
        })
        .collect();

    let file_list_scrollable = scrollable(column(file_list_items).spacing(SPACING))
        .width(Length::Fill)
        .height(Length::FillPortion(3));

    let file_list_area = column![file_list_header, file_list_scrollable,]
        .spacing(SPACING)
        .padding(PADDING)
        .width(Length::Fill);

    // 批量操作区域
    let batch_actions_area = if selected_files.is_empty() {
        row![]
    } else {
        row![
            button("批量删除")
                .on_press(Message::BatchDeleteButtonPressed)
                .style(iced::theme::Button::Destructive), // 使用 Destructive 样式
            button("批量下载")
                .on_press(Message::BatchDownloadButtonPressed)
                .style(iced::theme::Button::Primary), // 使用 Primary 样式
        ]
        .spacing(SPACING)
        .padding(PADDING)
        .width(Length::Fill)
    };

    // 下载区域
    let download_area = container(
        column![
            text_input("输入文件 ID", download_id_input)
                .on_input(Message::DownloadInputChanged)
                .padding(PADDING)
                .width(Length::Fill)
                .style(iced::theme::TextInput::Default), // 使用默认样式
            button("从 ID 下载")
                .on_press(Message::DownloadFromInputButtonPressed)
                .style(iced::theme::Button::Primary), // 使用 Primary 样式
        ]
        .spacing(SPACING)
        .padding(PADDING)
        .width(Length::Fill),
    )
    .style(iced::theme::Container::Box) // 添加边框和背景
    .padding(PADDING)
    .width(Length::Fill);

    // 导入/导出配置区域
    let config_management_area = container(
        row![
            button("导入配置")
                .on_press(Message::TriggerImportConfig)
                .style(iced::theme::Button::Secondary), // 使用 Secondary 样式
            button("导出配置")
                .on_press(Message::TriggerExportConfig)
                .style(iced::theme::Button::Secondary), // 使用 Secondary 样式
        ]
        .spacing(SPACING)
        .padding(PADDING)
        .width(Length::Fill),
    )
    .style(iced::theme::Container::Box) // 添加边框和背景
    .padding(PADDING)
    .width(Length::Fill);

    let status_bar = container(
        text(status_message)
            .size(16)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .vertical_alignment(iced::alignment::Vertical::Center)
            .style(iced::theme::Text::Color(HACKER_FOREGROUND)), // 设置文本颜色
    )
    .width(Length::Fill)
    .padding(PADDING)
    .center_x()
    .style(iced::theme::Container::Box); // 添加背景和边框

    column![
        title_bar,
        upload_area,
        search_input_widget, // 添加搜索输入框
        file_list_area,
        batch_actions_area, // 添加批量操作区域
        download_area,
        config_management_area, // 添加配置管理区域
        status_bar,
    ]
    .spacing(SPACING)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
