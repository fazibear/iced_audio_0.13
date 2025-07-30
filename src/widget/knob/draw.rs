use std::cmp::Ordering;

use iced::{
    advanced::renderer::{self, Quad},
    border::Radius,
    widget::canvas::{self, path::Arc, Frame, Path, Stroke},
    Border, Point, Radians, Rectangle, Shadow, Size, Vector,
};

use crate::{
    style::knob::{
        ArcAppearance, ArcBipolarAppearance, CircleAppearance, CircleNotch, LineNotch,
        ModRangeArcAppearance, NotchShape, ValueArcAppearance,
    },
    widget::knob::{KnobInfo, ValueMarkers},
    ModulationRange, Normal,
};

enum BipolarState {
    Left,
    Right,
    Center,
}

impl BipolarState {
    pub fn from_knob_info(knob_info: &KnobInfo) -> Self {
        if let Some(center) = knob_info.bipolar_center {
            match knob_info.value.partial_cmp(&center) {
                Some(Ordering::Less) => BipolarState::Left,
                Some(Ordering::Equal) => BipolarState::Center,
                Some(Ordering::Greater) => BipolarState::Right,
                None => BipolarState::Center,
            }
        } else if knob_info.value.as_f32() < 0.499 {
            BipolarState::Left
        } else if knob_info.value.as_f32() > 0.501 {
            BipolarState::Right
        } else {
            BipolarState::Center
        }
    }
}

pub fn draw_circle_style<Renderer>(
    renderer: &mut Renderer,
    knob_info: &KnobInfo,
    style: CircleAppearance,
    value_markers: &ValueMarkers<'_>,
    //    tick_marks_cache: &tick_marks::PrimitiveCache,
    //    text_marks_cache: &text_marks::PrimitiveCache,
) where
    Renderer: iced::advanced::graphics::geometry::Renderer,
{
    // draw_tick_marks(
    //     knob_info,
    //     value_markers.tick_marks,
    //     &value_markers.tick_marks_style,
    //     tick_marks_cache,
    // );
    // draw_text_marks(
    //     knob_info,
    //     value_markers.text_marks,
    //     &value_markers.text_marks_style,
    //     text_marks_cache,
    // );

    draw_value_arc(renderer, knob_info, &value_markers.value_arc_style);

    draw_mod_range_arc(
        renderer,
        knob_info,
        &value_markers.mod_range_style_1,
        value_markers.mod_range_1,
    );

    draw_mod_range_arc(
        renderer,
        knob_info,
        &value_markers.mod_range_style_2,
        value_markers.mod_range_2,
    );

    renderer.fill_quad(
        Quad {
            bounds: knob_info.bounds,
            border: Border {
                color: style.border_color,
                width: style.border_width,
                radius: Radius::new(knob_info.radius),
            },
            shadow: Shadow::default(),
        },
        style.color,
    );

    draw_notch(renderer, knob_info, &style.notch);
}

pub fn draw_arc_style<Renderer>(
    renderer: &mut Renderer,
    knob_info: &KnobInfo,
    style: ArcAppearance,
    value_markers: &ValueMarkers<'_>,
    //    tick_marks_cache: &tick_marks::PrimitiveCache,
    //    text_marks_cache: &text_marks::PrimitiveCache,
) where
    Renderer: iced::advanced::graphics::geometry::Renderer,
{
    // let (tick_marks, text_marks, value_arc, mod_range_arc_1, mod_range_arc_2) =
    //draw_value_markers(knob_info, value_markers, tick_marks_cache, text_marks_cache);

    let width = style.width.from_knob_diameter(knob_info.bounds.width);

    let center_point = Point::new(knob_info.radius, knob_info.radius);
    let arc_radius = knob_info.radius - (width / 2.0);

    let mut frame = Frame::new(
        renderer,
        Size::new(knob_info.bounds.width, knob_info.bounds.width),
    );

    let empty_stroke = Stroke {
        width,
        style: canvas::Style::Solid(style.empty_color),
        line_cap: style.cap,
        ..Stroke::default()
    };

    let empty_arc = Arc {
        center: center_point,
        radius: arc_radius,
        start_angle: Radians(knob_info.start_angle),
        end_angle: Radians(knob_info.start_angle + knob_info.angle_span),
    };

    let empty_path = Path::new(|path| path.arc(empty_arc));

    frame.stroke(&empty_path, empty_stroke);

    let filled_stroke = Stroke {
        width,
        style: canvas::Style::Solid(style.filled_color),
        line_cap: style.cap,
        ..Stroke::default()
    };

    let filled_arc = Arc {
        center: center_point,
        radius: arc_radius,
        start_angle: Radians(knob_info.start_angle),
        end_angle: Radians(knob_info.value_angle),
    };

    let filled_path = Path::new(|path| path.arc(filled_arc));

    frame.stroke(&filled_path, filled_stroke);

    renderer.with_translation(
        Vector::new(knob_info.bounds.x, knob_info.bounds.y),
        |renderer| {
            renderer.draw_geometry(frame.into_geometry());
        },
    );

    draw_notch(renderer, knob_info, &style.notch);
}

pub fn draw_arc_bipolar_style<Renderer>(
    renderer: &mut Renderer,
    knob_info: &KnobInfo,
    style: ArcBipolarAppearance,
    value_markers: &ValueMarkers<'_>,
    // tick_marks_cache: &tick_marks::PrimitiveCache,
    // text_marks_cache: &text_marks::PrimitiveCache,
) where
    Renderer: iced::advanced::graphics::geometry::Renderer,
{
    // let (tick_marks, text_marks, value_arc, mod_range_arc_1, mod_range_arc_2) =
    //     draw_value_markers(knob_info, value_markers, tick_marks_cache, text_marks_cache);

    let bipolar_state = BipolarState::from_knob_info(knob_info);

    let width = style.width.from_knob_diameter(knob_info.bounds.width);

    let center_point = Point::new(knob_info.radius, knob_info.radius);
    let arc_radius = knob_info.radius - (width / 2.0);

    let mut frame = Frame::new(
        renderer,
        Size::new(knob_info.bounds.width, knob_info.bounds.width),
    );

    let empty_stroke = Stroke {
        width,
        style: canvas::Style::Solid(style.empty_color),
        line_cap: style.cap,
        ..Stroke::default()
    };

    let empty_arc = Arc {
        center: center_point,
        radius: arc_radius,
        start_angle: Radians(knob_info.start_angle),
        end_angle: Radians(knob_info.start_angle + knob_info.angle_span),
    };

    let empty_path = Path::new(|path| path.arc(empty_arc));

    frame.stroke(&empty_path, empty_stroke);

    let center_angle = knob_info.start_angle
        + knob_info
            .bipolar_center
            .unwrap_or_else(|| Normal::from_clipped(0.5))
            .scale(knob_info.angle_span);

    match bipolar_state {
        BipolarState::Left => {
            let filled_stroke = Stroke {
                width,
                style: canvas::Style::Solid(style.left_filled_color),
                line_cap: style.cap,
                ..Stroke::default()
            };

            let filled_arc = Arc {
                center: center_point,
                radius: arc_radius,
                start_angle: Radians(knob_info.value_angle),
                end_angle: Radians(center_angle),
            };

            let filled_path = Path::new(|path| path.arc(filled_arc));

            frame.stroke(&filled_path, filled_stroke);
        }
        BipolarState::Right => {
            let filled_stroke = Stroke {
                width,
                style: canvas::Style::Solid(style.right_filled_color),
                line_cap: style.cap,
                ..Stroke::default()
            };

            let filled_arc = Arc {
                center: center_point,
                radius: arc_radius,
                start_angle: Radians(center_angle),
                end_angle: Radians(knob_info.value_angle),
            };

            let filled_path = Path::new(|path| path.arc(filled_arc));

            frame.stroke(&filled_path, filled_stroke);
        }
        _ => {}
    }

    renderer.with_translation(
        Vector::new(knob_info.bounds.x, knob_info.bounds.y),
        |renderer| {
            renderer.draw_geometry(frame.into_geometry());
        },
    );

    if let Some((notch_left, notch_right)) = style.notch_left_right {
        match bipolar_state {
            BipolarState::Left => draw_notch(renderer, knob_info, &notch_left),
            BipolarState::Right => draw_notch(renderer, knob_info, &notch_right),
            BipolarState::Center => draw_notch(renderer, knob_info, &style.notch_center),
        }
    } else {
        draw_notch(renderer, knob_info, &style.notch_center)
    };
}

fn draw_circle_notch<Renderer: renderer::Renderer>(
    renderer: &mut Renderer,
    knob_info: &KnobInfo,
    style: &CircleNotch,
) {
    let value_angle = knob_info.value_angle + std::f32::consts::FRAC_PI_2;

    let (dx, dy) = if !(-0.001..=0.001).contains(&value_angle) {
        value_angle.sin_cos()
    } else {
        (0.0, -1.0)
    };

    let notch_diameter = style.diameter.from_knob_diameter(knob_info.bounds.width);
    let notch_radius = notch_diameter / 2.0;

    let offset_radius = knob_info.radius - style.offset.from_knob_diameter(knob_info.bounds.width);

    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x: knob_info.bounds.center_x() + (dx * offset_radius) - notch_radius,
                y: knob_info.bounds.center_y() - (dy * offset_radius) - notch_radius,
                width: notch_diameter,
                height: notch_diameter,
            },
            border: Border {
                color: style.border_color,
                width: style.border_width,
                radius: Radius::new(notch_radius),
            },
            shadow: Shadow::default(),
        },
        style.color,
    );
}

fn draw_line_notch<Renderer>(renderer: &mut Renderer, knob_info: &KnobInfo, style: &LineNotch)
where
    Renderer: iced::advanced::graphics::geometry::Renderer,
{
    let value_angle = knob_info.value_angle + std::f32::consts::FRAC_PI_2;

    let stroke = Stroke {
        width: style.width.from_knob_diameter(knob_info.bounds.width),
        style: canvas::Style::Solid(style.color),
        line_cap: style.cap,
        ..Stroke::default()
    };

    let stroke_begin_y =
        -(knob_info.radius - style.offset.from_knob_diameter(knob_info.bounds.width));
    let notch_height = style.length.from_knob_diameter(knob_info.bounds.width);

    let path = Path::line(
        Point::new(0.0, stroke_begin_y),
        Point::new(0.0, stroke_begin_y + notch_height),
    );

    let mut frame = Frame::new(
        renderer,
        Size::new(knob_info.bounds.width, knob_info.bounds.width),
    );

    frame.translate(Vector::new(knob_info.radius, knob_info.radius));

    if !(-0.001..=0.001).contains(&value_angle) {
        frame.rotate(value_angle);
    }

    frame.stroke(&path, stroke);

    renderer.with_translation(
        Vector::new(knob_info.bounds.x, knob_info.bounds.y),
        |renderer| {
            renderer.draw_geometry(frame.into_geometry());
        },
    );
}

fn draw_notch<Renderer>(renderer: &mut Renderer, knob_info: &KnobInfo, notch: &NotchShape)
where
    Renderer: iced::advanced::graphics::geometry::Renderer,
{
    match notch {
        NotchShape::Circle(style) => draw_circle_notch(renderer, knob_info, style),
        NotchShape::Line(style) => draw_line_notch(renderer, knob_info, style),
        NotchShape::None => {}
    }
}

fn draw_value_arc<Renderer>(
    renderer: &mut Renderer,
    knob_info: &KnobInfo,
    style: &Option<ValueArcAppearance>,
) where
    Renderer: iced::advanced::graphics::geometry::Renderer,
{
    if let Some(style) = style {
        let half_width = style.width / 2.0;

        let end_angle = knob_info.start_angle + knob_info.angle_span;
        let arc_radius = knob_info.radius + style.offset + half_width;

        let half_frame_size = (arc_radius + half_width).ceil();
        let frame_size = half_frame_size * 2.0;
        let frame_offset = half_frame_size - knob_info.radius;
        let center_point = Point::new(half_frame_size, half_frame_size);

        let mut frame = Frame::new(renderer, Size::new(frame_size, frame_size));

        if let Some(empty_color) = style.empty_color {
            let empty_stroke = Stroke {
                width: style.width,
                style: canvas::Style::Solid(empty_color),
                line_cap: style.cap,
                ..Stroke::default()
            };

            let empty_arc = Arc {
                center: center_point,
                radius: arc_radius,
                start_angle: Radians(knob_info.start_angle),
                end_angle: Radians(end_angle),
            };

            let empty_path = Path::new(|path| path.arc(empty_arc));

            frame.stroke(&empty_path, empty_stroke);
        }

        if let Some(right_filled_color) = style.right_filled_color {
            if knob_info.value.as_f32() < 0.499 || knob_info.value.as_f32() > 0.501 {
                let half_angle = knob_info.start_angle + (knob_info.angle_span / 2.0);

                if knob_info.value < Normal::CENTER {
                    let filled_stroke = Stroke {
                        width: style.width,
                        style: canvas::Style::Solid(style.left_filled_color),
                        line_cap: style.cap,
                        ..Stroke::default()
                    };

                    let filled_arc = Arc {
                        center: center_point,
                        radius: arc_radius,
                        start_angle: Radians(knob_info.value_angle),
                        end_angle: Radians(half_angle),
                    };

                    let filled_path = Path::new(|path| path.arc(filled_arc));

                    frame.stroke(&filled_path, filled_stroke);
                } else if knob_info.value > Normal::CENTER {
                    let filled_stroke = Stroke {
                        width: style.width,
                        style: canvas::Style::Solid(right_filled_color),
                        line_cap: style.cap,
                        ..Stroke::default()
                    };

                    let filled_arc = Arc {
                        center: center_point,
                        radius: arc_radius,
                        start_angle: Radians(half_angle),
                        end_angle: Radians(knob_info.value_angle),
                    };

                    let filled_path = Path::new(|path| path.arc(filled_arc));

                    frame.stroke(&filled_path, filled_stroke);
                }
            }
        } else if knob_info.value != Normal::MIN {
            let filled_stroke = Stroke {
                width: style.width,
                style: canvas::Style::Solid(style.left_filled_color),
                line_cap: style.cap,
                ..Stroke::default()
            };

            let filled_arc = Arc {
                center: center_point,
                radius: arc_radius,
                start_angle: Radians(knob_info.start_angle),
                end_angle: Radians(knob_info.value_angle),
            };

            let filled_path = Path::new(|path| path.arc(filled_arc));

            frame.stroke(&filled_path, filled_stroke);
        }

        renderer.with_translation(
            Vector::new(
                knob_info.bounds.x - frame_offset,
                knob_info.bounds.y - frame_offset,
            ),
            |renderer| {
                renderer.draw_geometry(frame.into_geometry());
            },
        );
    }
}

fn draw_mod_range_arc<Renderer>(
    renderer: &mut Renderer,
    knob_info: &KnobInfo,
    style: &Option<ModRangeArcAppearance>,
    mod_range: Option<&ModulationRange>,
) where
    Renderer: iced::advanced::graphics::geometry::Renderer,
{
    if let Some(mod_range) = mod_range {
        if let Some(style) = style {
            let half_width = style.width / 2.0;
            let arc_radius = knob_info.radius + style.offset + half_width;

            let half_frame_size = (arc_radius + half_width).ceil();
            let frame_size = half_frame_size * 2.0;
            let frame_offset = half_frame_size - knob_info.radius;
            let center_point = Point::new(half_frame_size, half_frame_size);

            let mut frame = Frame::new(renderer, Size::new(frame_size, frame_size));

            if let Some(empty_color) = style.empty_color {
                let empty_stroke = Stroke {
                    width: style.width,
                    style: canvas::Style::Solid(empty_color),
                    line_cap: style.cap,
                    ..Stroke::default()
                };

                let empty_arc = Arc {
                    center: center_point,
                    radius: arc_radius,
                    start_angle: Radians(knob_info.start_angle),
                    end_angle: Radians(knob_info.start_angle + knob_info.angle_span),
                };

                let empty_path = Path::new(|path| path.arc(empty_arc));

                frame.stroke(&empty_path, empty_stroke);
            }

            if mod_range.filled_visible && (mod_range.start != mod_range.end) {
                let (start, end, color) = if mod_range.start.as_f32() < mod_range.end.as_f32() {
                    (
                        mod_range.start.as_f32(),
                        mod_range.end.as_f32(),
                        style.filled_color,
                    )
                } else {
                    (
                        mod_range.end.as_f32(),
                        mod_range.start.as_f32(),
                        style.filled_inverse_color,
                    )
                };

                let filled_stroke = Stroke {
                    width: style.width,
                    style: canvas::Style::Solid(color),
                    line_cap: style.cap,
                    ..Stroke::default()
                };

                let filled_arc = Arc {
                    center: center_point,
                    radius: arc_radius,
                    start_angle: Radians(knob_info.start_angle + (knob_info.angle_span * start)),
                    end_angle: Radians(knob_info.start_angle + (knob_info.angle_span * end)),
                };

                let filled_path = Path::new(|path| path.arc(filled_arc));

                frame.stroke(&filled_path, filled_stroke);
            }

            renderer.with_translation(
                Vector::new(
                    knob_info.bounds.x - frame_offset,
                    knob_info.bounds.y - frame_offset,
                ),
                |renderer| {
                    renderer.draw_geometry(frame.into_geometry());
                },
            );
        }
    }
}
