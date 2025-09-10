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

    // 文件列表区域
    let file_list_header = row![
        text("文件名").width(Length::FillPortion(3)),
        text("文件 ID").width(Length::FillPortion(2)),
        text("上传时间").width(Length::FillPortion(2)),
        text("操作").width(Length::FillPortion(2)),
    ]
    .spacing(10)
    .padding(5);

    let file_list_items: Vec<Element<Message>> = files
        .iter()
        .map(|file| {
            row![
                text(&file.name).width(Length::FillPortion(3)),
                text(&file.id).width(Length::FillPortion(2)),
                text(&file.uploaded_at).width(Length::FillPortion(2)),
                row![
                    button("复制 ID").on_press(Message::CopyIdToClipboard(file.id.clone())),
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

    let file_list_scrollable = scrollable(column(file_list_items).spacing(5))
        .width(Length::Fill)
        .height(Length::FillPortion(3));

    let file_list_area = column![file_list_header, file_list_scrollable,]
        .spacing(10)
        .padding(10)
        .width(Length::Fill);

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
        file_list_area,
        Space::with_height(Length::Fixed(20.0)),
        download_area,
        status_bar,
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
