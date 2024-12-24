
use std::sync::Arc;
use std::io;
use std::path::{Path,PathBuf};
use iced::widget::{button,column,container,row,Scrollable,text,tooltip,Text};
use iced::{Task,Element,Font};
use iced::widget::Column;


pub fn main() -> iced::Result {
    iced::application("File Operation",FileOperation::update,FileOperation::view)
    .font(include_bytes!("../fonts/icons.ttf").as_slice())
    .run_with(FileOperation::new)
}

#[derive(Debug, Clone)]
enum Message {
    OpenFile,
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    SaveFile,
    FileSaved(Result<PathBuf, Error>),
}

#[derive(Debug, Clone)]
pub enum Error {
    // 通常在用户交互过程中，使用文件选择对话框时出现。
    // 当用户打开文件选择对话框，但选择关闭对话框（例如点击“取消”按钮或关闭窗口的按钮），而不是选择一个文件时，这个错误会被触发 DialogClosed
    DialogClosed,
    InvalidFileType,
    IoError(io::ErrorKind),
}

struct FileOperation{
    file: Option<PathBuf>,
    content: Vec<String>,
}

async fn open_file() -> Result<(PathBuf, Arc<String>), Error> {
    let picked_file = rfd::AsyncFileDialog::new()
        .set_title("Open a text file...")
        .add_filter("Text file", &["txt"]) // 只显示 txt 文件
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    if !picked_file.path().to_str().map_or(false, |s| s.ends_with(".txt")) {
        return Err(Error::InvalidFileType);
    }

    let contents = tokio::fs::read_to_string(picked_file.path()).await
        .map_err(|error| Error::IoError(error.kind()))?;
    Ok((picked_file.path().to_path_buf(), Arc::new(contents)))
}

async fn save_file(
    path: Option<PathBuf>,
    contents: Vec<String>,
) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .save_file()
            .await
            .as_ref()
            .map(rfd::FileHandle::path)
            .map(Path::to_owned)
            .ok_or(Error::DialogClosed)?
    };
    let content_string = contents.join("\n");
    tokio::fs::write(&path, content_string)
        .await
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok(path)
}

impl FileOperation{
    fn new() ->(Self, Task<Message>)  {
        (
            Self {file: None,content: Vec::new()},
            Task::none()
        )
    }


    fn update(&mut self, message: Message) -> Task<Message>
    {
        match message {
            Message::OpenFile => {
                Task::perform(open_file(), Message::FileOpened)
            }
            Message::FileOpened(result) => {
                if let Ok((path, contents)) = result {
                    self.file = Some(path);
                    let line_content = Arc::try_unwrap(contents)
                    .unwrap_or_else(|arc| (*arc).clone());
                    self.content.push(line_content);
                }
                Task::none()
            }
            Message::SaveFile => {
                Task::perform(
                    save_file(self.file.clone(), std::mem::take(&mut self.content)),
                    Message::FileSaved,
                )
            }
            Message::FileSaved(_result) => {
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message>
    {
        let controls = row![
            action(open_icon(), "Open file", Some(Message::OpenFile)),
            action(save_icon(), "Save file", Some(Message::SaveFile))
        ];
        let content = self.content.iter().fold(Column::new().spacing(5), |column, text| {
            column.push(Text::new(text.clone()))
        });
        let scrollable = Scrollable::new(content);
        column![
            controls,
            scrollable
        ].into()
    }
}

fn action<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let action = button(container(content).center_x(30));

    if let Some(on_press) = on_press {
        tooltip(
            action.on_press(on_press),
            label,
            tooltip::Position::FollowCursor,
        )
        .style(container::rounded_box)
        .into()
    } else {
        action.style(button::secondary).into()
    }
}

fn save_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e801}')
}

fn open_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0f115}')
}

fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("editor-icons");

    text(codepoint).font(ICON_FONT).into()
}