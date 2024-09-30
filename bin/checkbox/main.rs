use iced::widget::{center, checkbox, column,scrollable, text};
use iced::Element;
use iced::Center;


pub fn main() -> iced::Result {
    iced::application("Checkbox - Iced", Example::update, Example::view)
        .window_size((250.0, 400.0)) 
        .run()
}

#[derive(Default)]
struct Example {
    default: bool,
    history: Vec<String>
}

#[derive(Debug, Clone, Copy)]
enum Message {
    DefaultToggled(bool),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::DefaultToggled(default) => {
                self.default = default;
                // 每次复选框状态改变时，记录新的状态
                let new_status = if default {
                    "Checkbox is checked".to_string()
                } else {
                    "Checkbox is unchecked".to_string()
                };
                self.history.push(new_status); // 追加记录到历史中
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // 创建复选框
        let default_checkbox = checkbox("Default", self.default)
            .on_toggle(Message::DefaultToggled);

        // 将历史记录中的每一条状态生成 `text` 元素
        let history_texts: Vec<Element<Message>> = self
            .history
            .iter()
            .map(|entry| text(entry.clone()).into())
            .collect();

        // 使用 scrollable 让历史记录可以滚动显示
        let scrollable_history = scrollable(column(history_texts).spacing(10))
            .height(200).width(500); // 设置滚动区域的高度

        // 将复选框和滚动显示框垂直排列
        let content = column![
            default_checkbox, 
            scrollable_history
        ]
        .spacing(20)
        .align_x(Center);// 居中对齐 这边的话 其实是两个元素了，如果想单独设置checkBox的话需要使用row! 或者 column!

        center(content).into()
    }
}
