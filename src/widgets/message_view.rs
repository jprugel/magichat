use iced::Element;
use iced::widget::text_editor;
use iced::widget::text_editor::Content;

pub struct MessageView {
    content: text_editor::Content,
}

#[derive(Clone, Debug)]
enum Message {}

impl MessageView {
    
    pub fn new(content: &str) -> Self {
        Self {
            content: Content::with_text(content),
        }
    }
    pub fn view(&self, content: &str) -> Element<'_, Message> {
        text_editor(&self.content).into()
    }
}