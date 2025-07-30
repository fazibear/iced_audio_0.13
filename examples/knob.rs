use iced::{widget::container, Length};
use iced_audio::{Knob, Normal, NormalParam};

fn main() -> iced::Result {
    iced::run("Knob Example", KnobExample::update, KnobExample::view)
}

#[derive(Debug, Clone)]
enum Message {
    KnobChange(Normal),
}

#[derive(Default)]
struct KnobExample;

impl KnobExample {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<Message> {
        let knob = Knob::new(NormalParam::default(), Message::KnobChange);
        container(knob)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
