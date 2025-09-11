use crate::Message;
use crate::data::FileEntry;
use iced::widget::{button, column, container, row, scrollable, text, text_input, checkbox};
use iced::{Element, Length, Color};
 
const SPACING: u16 = 10;
const PADDING: u16 = 10;
 
// Cyberpunk theme colors
const CYBER_BACKGROUND: Color = Color::from_rgb(
    0x08 as f32 / 255.0,
    0x08 as f32 / 255.0,
    0x1A as f32 / 255.0,
); // Dark Blue/Near Black
const CYBER_FOREGROUND: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0xFF as f32 / 255.0,
    0x00 as f32 / 255.0,
); // Bright Green (Neon)
const CYBER_ACCENT_BLUE: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0xC8 as f32 / 255.0,
    0xFF as f32 / 255.0,
); // Bright Blue (Neon)
const CYBER_ACCENT_PURPLE: Color = Color::from_rgb(
    0xBF as f32 / 255.0,
    0x00 as f32 / 255.0,
    0xFF as f32 / 255.0,
); // Bright Purple (Neon)
const CYBER_ERROR: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0x33 as f32 / 255.0,
    0x33 as f32 / 255.0,
); // Bright Red
const CYBER_GREY: Color = Color::from_rgb(
    0x20 as f32 / 255.0,
    0x20 as f32 / 255.0,
    0x30 as f32 / 255.0,
); // Dark Grey for secondary elements


pub fn view_application<'a>(
    files: &'a [FileEntry],
    upload_file_path: &'a str,
    upload_progress: f32,
    download_id_input: &'a str,
    status_message: &'a str,
    search_input: &'a str, // 添加搜索输入参数
    selected_files: &'a std::collections::HashSet<String>, // 新增
) -> Element<'a, Message> {
    let config_buttons = row![
        button("导入配置")
            .on_press(Message::TriggerImportConfig)
            .style(iced::theme::Button::Custom(Box::new(CyberButtonStyle))),
        button("导出配置")
            .on_press(Message::TriggerExportConfig)
            .style(iced::theme::Button::Custom(Box::new(CyberButtonStyle))),
        button("上传配置")
            .on_press(Message::UploadConfigButtonPressed)
            .style(iced::theme::Button::Custom(Box::new(CyberButtonStyle))),
    ]
    .spacing(SPACING)
    .padding(PADDING);

    let title_bar = container(
        row![
            text("Walrus云盘")
                .size(30)
                .style(iced::theme::Text::Color(CYBER_FOREGROUND)),
            iced::widget::Space::with_width(Length::Fill), // 填充空间，将按钮推到右边
            config_buttons,
        ]
        .align_items(iced::alignment::Alignment::Center)
        .spacing(SPACING)
    )
    .width(Length::Fill)
    .padding(PADDING)
    .center_y()
    .style(iced::theme::Container::Custom(Box::new(
        CyberContainerStyle {
            background: Some(CYBER_BACKGROUND.into()),
            border: iced::Border {
                radius: 5.0.into(),
                width: 2.0,
                color: CYBER_ACCENT_BLUE,
            },
        },
    )));

    // 上传区域
    let upload_area = container(
        column![
            row![
                button("选择文件")
                    .on_press(Message::TriggerFileSelection)
                    .style(iced::theme::Button::Custom(Box::new(CyberButtonStyle))),
                text_input("文件路径", upload_file_path)
                    .on_input(|_| Message::NoOp) // 只读
                    .width(Length::FillPortion(2))
                    .style(iced::theme::TextInput::Custom(Box::new(CyberTextInputStyle))),
            ]
            .spacing(SPACING),
            button("上传文件")
                .on_press(Message::UploadButtonPressed)
                .style(iced::theme::Button::Custom(Box::new(CyberButtonStyle))),
            // 进度条 (占位符)
            text(format!("上传进度: {:.0}%", upload_progress * 100.0))
                .style(iced::theme::Text::Color(CYBER_FOREGROUND)),
        ]
        .spacing(SPACING)
        .padding(PADDING)
        .width(Length::Fill),
    )
    .style(iced::theme::Container::Custom(Box::new(
        CyberContainerStyle {
            background: Some(CYBER_GREY.into()),
            border: iced::Border {
                radius: 5.0.into(),
                width: 1.0,
                color: CYBER_ACCENT_BLUE,
            },
        },
    )))
    .padding(PADDING)
    .width(Length::Fill);

    // 文件搜索输入框
    let search_input_widget = text_input("搜索文件...", search_input)
        .on_input(Message::SearchInputChanged)
        .padding(PADDING)
        .width(Length::Fill)
        .style(iced::theme::TextInput::Custom(Box::new(CyberTextInputStyle)));

    // 文件列表区域
    let file_list_header = container(
        row![
            text("").width(Length::Fixed(20.0)), // 复选框的占位
            text("文件名").width(Length::FillPortion(3)).style(iced::theme::Text::Color(CYBER_ACCENT_BLUE)),
            text("文件 ID").width(Length::FillPortion(2)).style(iced::theme::Text::Color(CYBER_ACCENT_BLUE)),
            text("上传时间").width(Length::FillPortion(2)).style(iced::theme::Text::Color(CYBER_ACCENT_BLUE)),
            text("操作").width(Length::FillPortion(2)).style(iced::theme::Text::Color(CYBER_ACCENT_BLUE)),
        ]
        .spacing(SPACING)
        .padding(PADDING)
    )
    .style(iced::theme::Container::Custom(Box::new(
        CyberContainerStyle {
            background: Some(CYBER_GREY.into()),
            border: iced::Border {
                radius: 5.0.into(),
                width: 1.0,
                color: CYBER_ACCENT_PURPLE,
            },
        },
    )));

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

            let checkbox = checkbox(
                "", // label
                selected_files.contains(&file_id_clone), // is_checked
            )
            .on_toggle(move |is_checked| Message::FileSelectedForBatch(file_id_clone.clone(), is_checked))
            .width(Length::Fixed(20.0))
            .style(iced::theme::Checkbox::Custom(Box::new(
                CyberCheckboxStyle,
            )));

            container(
                row![
                    checkbox,
                    text(file_name_clone).width(Length::FillPortion(3)).style(iced::theme::Text::Color(CYBER_FOREGROUND)),
                    text(display_id_clone).width(Length::FillPortion(2)).style(iced::theme::Text::Color(CYBER_FOREGROUND)),
                    text(uploaded_at_clone).width(Length::FillPortion(2)).style(iced::theme::Text::Color(CYBER_FOREGROUND)),
                    row![
                        button("复制 ID")
                            .on_press(Message::CopyIdToClipboard(file_ref.id.clone()))
                            .style(iced::theme::Button::Custom(Box::new(CyberButtonStyle))),
                        button("下载")
                            .on_press(Message::DownloadButtonPressed(file_ref.id.clone()))
                            .style(iced::theme::Button::Custom(Box::new(CyberButtonStyle))),
                        button("删除")
                            .on_press(Message::DeleteButtonPressed(file_ref.id.clone()))
                            .style(iced::theme::Button::Custom(Box::new(CyberDestructiveButtonStyle))),
                    ]
                    .spacing(SPACING)
                    .width(Length::FillPortion(2)),
                ]
                .spacing(SPACING)
                .padding(PADDING),
            )
            .style(iced::theme::Container::Custom(Box::new(
                CyberContainerStyle {
                    background: Some(CYBER_BACKGROUND.into()),
                    border: iced::Border {
                        radius: 5.0.into(),
                        width: 1.0,
                        color: CYBER_ACCENT_PURPLE,
                    },
                },
            )))
            .into()
        })
        .collect();

    let file_list_scrollable = scrollable(column(file_list_items).spacing(SPACING))
        .width(Length::Fill)
        .height(Length::FillPortion(6));

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
                .style(iced::theme::Button::Custom(Box::new(CyberDestructiveButtonStyle))),
            button("批量下载")
                .on_press(Message::BatchDownloadButtonPressed)
                .style(iced::theme::Button::Custom(Box::new(CyberButtonStyle))),
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
                .style(iced::theme::TextInput::Custom(Box::new(CyberTextInputStyle))),
            row![
                button("从 ID 下载")
                    .on_press(Message::DownloadFromInputButtonPressed)
                    .style(iced::theme::Button::Custom(Box::new(CyberButtonStyle))),
                button("加载配置")
                    .on_press(Message::LoadConfigFromIdButtonPressed)
                    .style(iced::theme::Button::Custom(Box::new(CyberButtonStyle))),
            ]
            .spacing(SPACING)
            .width(Length::Fill),
        ]
        .spacing(SPACING)
        .padding(PADDING)
        .width(Length::Fill),
    )
    .style(iced::theme::Container::Custom(Box::new(
        CyberContainerStyle {
            background: Some(CYBER_GREY.into()),
            border: iced::Border {
                radius: 5.0.into(),
                width: 1.0,
                color: CYBER_ACCENT_BLUE,
            },
        },
    )))
    .padding(PADDING)
    .width(Length::Fill);


    let status_bar = container(
        text(status_message)
            .size(16)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .vertical_alignment(iced::alignment::Vertical::Center)
            .style(iced::theme::Text::Color(CYBER_FOREGROUND)),
    )
    .width(Length::Fill)
    .padding(PADDING)
    .center_x()
    .style(iced::theme::Container::Custom(Box::new(
        CyberContainerStyle {
            background: Some(CYBER_BACKGROUND.into()),
            border: iced::Border {
                radius: 5.0.into(),
                width: 2.0,
                color: CYBER_ACCENT_BLUE,
            },
        },
    )));

    column![
        title_bar,
        upload_area,
        search_input_widget, // 添加搜索输入框
        file_list_area,
        batch_actions_area, // 添加批量操作区域
        download_area,
        status_bar,
    ]
    .spacing(SPACING)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
 
struct CyberContainerStyle {
    background: Option<iced::Background>,
    border: iced::Border,
}
 
impl iced::widget::container::StyleSheet for CyberContainerStyle {
    type Style = iced::Theme;
 
    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: self.background,
            border: self.border,
            text_color: Some(CYBER_FOREGROUND),
            shadow: iced::Shadow::default(), // 添加 shadow 字段
        }
    }
}
 
struct CyberButtonStyle;
 
impl iced::widget::button::StyleSheet for CyberButtonStyle {
    type Style = iced::Theme;
 
    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(CYBER_ACCENT_BLUE.into()),
            border: iced::Border {
                radius: 5.0.into(),
                width: 1.0,
                color: CYBER_ACCENT_BLUE,
            },
            text_color: Color::BLACK,
            ..Default::default()
        }
    }
 
    fn hovered(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(CYBER_ACCENT_PURPLE.into()),
            text_color: Color::WHITE,
            ..self.active(_style)
        }
    }
 
    fn pressed(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            border: iced::Border {
                width: 2.0,
                ..self.active(_style).border
            },
            ..self.active(_style)
        }
    }
 
    fn disabled(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(CYBER_GREY.into()),
            text_color: Color::from_rgb(0.5, 0.5, 0.5),
            ..self.active(_style)
        }
    }
}
 
struct CyberDestructiveButtonStyle;
 
impl iced::widget::button::StyleSheet for CyberDestructiveButtonStyle {
    type Style = iced::Theme;
 
    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(CYBER_ERROR.into()),
            border: iced::Border {
                radius: 5.0.into(),
                width: 1.0,
                color: CYBER_ERROR,
            },
            text_color: Color::BLACK,
            ..Default::default()
        }
    }
 
    fn hovered(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(Color::from_rgb(1.0, 0.2, 0.2).into()), // Lighter red on hover
            text_color: Color::WHITE,
            ..self.active(_style)
        }
    }
 
    fn pressed(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            border: iced::Border {
                width: 2.0,
                ..self.active(_style).border
            },
            ..self.active(_style)
        }
    }
 
    fn disabled(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(CYBER_GREY.into()),
            text_color: Color::from_rgb(0.5, 0.5, 0.5),
            ..self.active(_style)
        }
    }
}
 
struct CyberTextInputStyle;
 
impl iced::widget::text_input::StyleSheet for CyberTextInputStyle {
    type Style = iced::Theme;
 
    fn active(&self, _style: &Self::Style) -> iced::widget::text_input::Appearance {
        iced::widget::text_input::Appearance {
            background: CYBER_GREY.into(),
            border: iced::Border {
                radius: 5.0.into(),
                width: 1.0,
                color: CYBER_ACCENT_BLUE,
            },
            icon_color: CYBER_FOREGROUND,
        }
    }
 
    fn focused(&self, _style: &Self::Style) -> iced::widget::text_input::Appearance {
        iced::widget::text_input::Appearance {
            border: iced::Border {
                color: CYBER_ACCENT_PURPLE,
                ..self.active(_style).border
            },
            ..self.active(_style)
        }
    }
 
    fn hovered(&self, _style: &Self::Style) -> iced::widget::text_input::Appearance {
        iced::widget::text_input::Appearance {
            border: iced::Border {
                color: CYBER_ACCENT_PURPLE,
                ..self.active(_style).border
            },
            ..self.focused(_style)
        }
    }
 
    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb(0.5, 0.5, 0.5)
    }
 
    fn value_color(&self, _style: &Self::Style) -> Color {
        CYBER_FOREGROUND
    }
 
    fn disabled_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb(0.5, 0.5, 0.5)
    }
 
    fn selection_color(&self, _style: &Self::Style) -> Color {
        CYBER_ACCENT_BLUE
    }
 
    fn disabled(&self, _style: &Self::Style) -> iced::widget::text_input::Appearance {
        iced::widget::text_input::Appearance {
            background: Color::from_rgb(0.1, 0.1, 0.1).into(),
            border: iced::Border {
                color: Color::from_rgb(0.2, 0.2, 0.2),
                ..self.active(_style).border
            },
            icon_color: Color::from_rgb(0.3, 0.3, 0.3),
        }
    }
}
 
struct CyberCheckboxStyle;
 
impl iced::widget::checkbox::StyleSheet for CyberCheckboxStyle {
    type Style = iced::Theme;
 
    fn active(&self, _style: &Self::Style, is_checked: bool) -> iced::widget::checkbox::Appearance {
        iced::widget::checkbox::Appearance {
            background: if is_checked {
                CYBER_ACCENT_BLUE.into()
            } else {
                CYBER_GREY.into()
            },
            border: iced::Border {
                radius: 3.0.into(),
                width: 1.0,
                color: CYBER_ACCENT_BLUE,
            },
            icon_color: Color::BLACK,
            text_color: Some(CYBER_FOREGROUND),
        }
    }
 
    fn hovered(&self, _style: &Self::Style, is_checked: bool) -> iced::widget::checkbox::Appearance {
        iced::widget::checkbox::Appearance {
            border: iced::Border {
                color: CYBER_ACCENT_PURPLE,
                ..self.active(_style, is_checked).border
            },
            ..self.active(_style, is_checked)
        }
    }
}
