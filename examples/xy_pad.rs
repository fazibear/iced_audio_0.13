mod colors;
mod info_text;

use iced::{
    application,
    widget::{column, row, text},
    Color, Element, Length, Result, Size,
};
use iced_audio::{style::xy_pad, FloatRange, Normal, NormalParam, XYPad};

fn main() -> Result {
    application("XYPad Example", XYPadExample::update, XYPadExample::view)
        .window_size(Size::new(600.0, 400.0))
        .run()
}

// Custom style for the XYPad widget

pub struct CustomStyle;
impl CustomStyle {
    const ACTIVE_HANDLE: xy_pad::HandleSquare = xy_pad::HandleSquare {
        color: colors::FILLED,
        size: 10,
        border_width: 1.0,
        border_radius: 2.0,
        border_color: colors::HANDLE,
    };
    const ACTIVE_STYLE: xy_pad::Appearance = xy_pad::Appearance {
        rail_width: 1.0,
        h_rail_color: colors::HANDLE,
        v_rail_color: colors::HANDLE,
        handle: xy_pad::HandleShape::Square(Self::ACTIVE_HANDLE),
        back_color: colors::EMPTY,
        border_width: 2.0,
        border_color: Color::BLACK,
        center_line_width: 1.0,
        center_line_color: Color {
            r: 0.2,
            g: 0.2,
            b: 0.2,
            a: 0.7,
        },
    };
}
impl xy_pad::StyleSheet for CustomStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> xy_pad::Appearance {
        Self::ACTIVE_STYLE
    }

    fn hovered(&self, _style: &Self::Style) -> xy_pad::Appearance {
        xy_pad::Appearance {
            handle: xy_pad::HandleShape::Square(xy_pad::HandleSquare {
                color: colors::FILLED_HOVER,
                size: 12,
                ..Self::ACTIVE_HANDLE
            }),
            ..Self::ACTIVE_STYLE
        }
    }

    fn dragging(&self, _style: &Self::Style) -> xy_pad::Appearance {
        xy_pad::Appearance {
            handle: xy_pad::HandleShape::Square(xy_pad::HandleSquare {
                color: colors::FILLED_HOVER,
                ..Self::ACTIVE_HANDLE
            }),
            ..Self::ACTIVE_STYLE
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Default(Normal, Normal),
    Custom(Normal, Normal),
    Knob(Normal),
}

pub struct XYPadExample {
    float_range: FloatRange,

    xy_pad_default_x_param: NormalParam,
    xy_pad_default_y_param: NormalParam,
    xy_pad_custom_x_param: NormalParam,
    xy_pad_custom_y_param: NormalParam,

    output_text_x: String,
    output_text_y: String,
}

impl Default for XYPadExample {
    fn default() -> Self {
        // initalize parameters

        let float_range = FloatRange::default_bipolar();

        // create application

        Self {
            float_range,

            // initialize the state of the xy_pad widget
            xy_pad_default_x_param: float_range.default_normal_param(),
            xy_pad_default_y_param: float_range.default_normal_param(),

            xy_pad_custom_x_param: float_range.default_normal_param(),
            xy_pad_custom_y_param: float_range.default_normal_param(),

            output_text_x: String::from("Move a widget"),
            output_text_y: String::from(" "),
        }
    }
}

impl XYPadExample {
    fn update(&mut self, message: Message) {
        match message {
            Message::Default(normal_x, normal_y) => {
                self.xy_pad_default_x_param.update(normal_x);
                self.xy_pad_default_y_param.update(normal_y);

                self.output_text_x = info_text::info_text_f32(
                    "XYPadDefaultX",
                    self.float_range.unmap_to_value(normal_x),
                );
                self.output_text_y = info_text::info_text_f32(
                    "XYPadDefaultY",
                    self.float_range.unmap_to_value(normal_y),
                );
            }
            Message::Custom(normal_x, normal_y) => {
                self.xy_pad_custom_x_param.update(normal_x);
                self.xy_pad_custom_y_param.update(normal_y);

                self.output_text_x = info_text::info_text_f32(
                    "XYPadCustomX",
                    self.float_range.unmap_to_value(normal_x),
                );
                self.output_text_y = info_text::info_text_f32(
                    "XYPadCustomY",
                    self.float_range.unmap_to_value(normal_y),
                );
            }
            _ => {}
        }
    }

    fn view(&self) -> Element<Message> {
        // create each of the XYPad widgets, passing in the value of
        // the corresponding parameter

        let xy_pad_default = XYPad::new(
            self.xy_pad_default_x_param,
            self.xy_pad_default_y_param,
            Message::Default,
        );

        let xy_pad_custom = XYPad::new(
            self.xy_pad_custom_x_param,
            self.xy_pad_custom_y_param,
            Message::Custom,
        )
        .style(CustomStyle);

        // push the widgets into rows
        let xy_pad_row = row![
            column![text("Default Style"), xy_pad_default,]
                .width(Length::Fill)
                .spacing(10),
            column![text("Custom Style"), xy_pad_custom,]
                .width(Length::Fill)
                .spacing(10),
        ]
        .spacing(20);

        column![
            xy_pad_row,
            text(&self.output_text_x).size(16),
            text(&self.output_text_y).size(16),
        ]
        .spacing(20)
        .padding(20)
        .into()
    }
}
