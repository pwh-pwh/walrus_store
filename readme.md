# Walrus 云盘客户端

这是一个使用 Rust 语言和 Iced 框架构建的桌面应用程序，旨在提供一个简单的文件上传、下载和管理界面，与 Walrus API 进行交互。

## 功能特性

*   **文件上传**: 用户可以选择本地文件并上传到 Walrus 云盘。
*   **文件下载**: 用户可以根据文件 ID 下载云盘中的文件到本地指定目录。
*   **文件删除**: 用户可以从云盘中删除文件。
*   **文件搜索**: 用户可以通过文件名搜索已上传的文件。
*   **选择文件下载**: 用户可以从文件列表中选择多个文件进行下载。
*   **根据输入 ID 下载**: 用户可以直接输入文件 ID 进行下载。
*   **配置加载**: 应用程序可以加载用户配置。
*   **本地文件管理**: 显示已上传文件的列表，包括文件 ID、文件名和上传时间。

## 技术栈

*   **Rust**: 核心编程语言。
*   **Iced**: 用于构建跨平台 GUI 的 Rust 框架。
*   **walrus_rs**: 用于与 Walrus API 交互的客户端库。
*   **rfd (native-dialog)**: 用于本地文件选择对话框。
*   **arboard**: 用于跨平台剪贴板操作。
*   **chrono**: 用于处理日期和时间。
*   **serde**: 用于序列化和反序列化数据。

## 安装与运行

### 1. 克隆仓库

```bash
git clone https://github.com/pwh-pwh/walrus_store.git
cd walrus_store
```

### 2. 构建与运行

确保您已安装 Rust 和 Cargo。

```bash
cargo run
```

### 3. 注意事项

*   本应用程序需要连接到 Walrus API。请确保您的网络环境允许访问 `https://aggregator.testnet.walrus.atalma.io` 和 `https://publisher.walrus-01.tududes.com`。
*   文件列表数据存储在本地。

## 项目结构

```
.
├── Cargo.toml
├── src/
│   ├── main.rs         # 应用程序入口，Iced 应用的初始化和主循环
│   ├── app_logic.rs    # 核心业务逻辑处理，包括文件上传、下载、删除和 UI 消息处理
│   ├── data.rs         # 数据结构定义，如 FileEntry
│   ├── file_management.rs # 本地文件和数据存储管理
│   ├── mock_api.rs     # (已弃用) 模拟 API，已替换为 walrus_api
│   ├── ui.rs           # 用户界面布局和组件
│   └── walrus_api.rs   # 与 Walrus API 的实际交互逻辑
└── readme.md
```

## 贡献

欢迎提交 Issue 和 Pull Request。