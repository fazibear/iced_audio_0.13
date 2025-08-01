mod colors;
mod info_text;

use iced::{
    application,
    widget::{canvas::LineCap, column, row, text},
    Color, Element, Length, Result, Size, Theme,
};
use iced_audio::{style, FloatRange, FreqRange, IntRange, LogDBRange, Normal, NormalParam};

fn main() -> Result {
    application("None Example", NoneExample::update, NoneExample::view)
        .window_size(Size::new(600.0, 400.0))
        .run()
}

#[derive(Debug, Clone)]
enum Message {}

pub struct NoneExample {}

impl Default for NoneExample {
    fn default() -> Self {
        Self {}
    }
}

impl NoneExample {
    fn update(&mut self, message: Message) {}

    fn view(&self) -> Element<Message> {
        column![].into()
    }
}
