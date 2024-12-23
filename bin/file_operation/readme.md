# 文件操作

该项目演示了一个使用 Rust 的 `iced` GUI 框架的简单文件操作应用程序。该应用程序允许用户打开和保存文本文件。

## 使用的 `iced` 组件

- `button`：用于创建按钮。
- `text`：用于显示文本。
- `column`：用于垂直排列 UI 元素。
- `row`：用于水平排列 UI 元素。
- `container`：用于容纳其他 UI 元素。
- `Scrollable`：用于创建可滚动区域。
- `tooltip`：用于显示工具提示。
- `Text`：用于设置文本样式。


### 导入
- `std::sync::Arc`
- `std::io`
- `std::path::{Path, PathBuf}`
- `iced::widget::{button, column, container, row, Scrollable, text, tooltip, Text}`
- `iced::{Task, Element, Font}`
- `iced::widget::Column`

### 主要功能
- `iced::application`：用于创建和运行应用程序。
- `Task`：用于处理异步任务。
- `Element`：表示 UI 元素。
- `Font`：用于设置字体。

### 消息枚举
- `Message`：定义了应用程序中的各种消息类型，如打开文件、文件已打开、保存文件、文件已保存。

### 错误枚举
- `Error`：定义了可能出现的错误类型，如对话框关闭、无效文件类型、IO 错误。

### 文件操作结构体
- `FileOperation`：包含文件路径和文件内容。

### 异步函数
- `open_file`：用于打开文件并读取内容。
- `save_file`：用于保存文件内容。

### 文件操作实现
- `new`：初始化文件操作结构体。
- `update`：处理消息并更新状态。
- `view`：生成 UI 界面。

### 辅助函数
- `action`：创建带有工具提示的按钮。
- `save_icon` 和 `open_icon`：生成保存和打开文件的图标。
- `icon`：生成图标元素。
