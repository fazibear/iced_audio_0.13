use iced::{widget::container, Border, Color, Element, Length, Rectangle, Shadow, Size, Theme};
use iced_audio::Knob;

fn main() -> iced::Result {
    iced::run("Knob Example", KnobExample::update, KnobExample::view)
}

#[derive(Debug, Clone)]
enum Message {}

#[derive(Default)]
struct KnobExample;

impl KnobExample {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<Message> {
        container(Knob)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
