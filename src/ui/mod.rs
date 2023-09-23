use iced::widget::{button, column};
use iced::{Alignment, Element, Sandbox};

use crate::export::{export_to_mp3, export_to_mp4};

pub struct App {}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ExportToMp3,
    ExportToMp4,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {}
    }

    fn title(&self) -> String {
        String::from("Vioux - Video Editor")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ExportToMp3 => export_to_mp3().unwrap(),
            Message::ExportToMp4 => export_to_mp4(),
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            button("Export to Mp3").on_press(Message::ExportToMp3),
            button("Export to Mp4").on_press(Message::ExportToMp4)
        ]
        .spacing(20)
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
