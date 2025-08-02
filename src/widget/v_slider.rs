//! Display an interactive vertical slider that controls a [`NormalParam`]
//!
//! [`NormalParam`]: ../core/normal_param/struct.Param.html

mod draw;
mod state;
mod value_markers;

use crate::{
    core::{ModulationRange, Normal, NormalParam, SliderStatus},
    text_marks, tick_marks,
};
use iced::{
    advanced::{
        graphics::core::{event, keyboard, touch},
        layout, mouse,
        renderer::Style,
        widget::{tree, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    Element, Event, Length, Rectangle, Renderer, Size,
};
use state::State;
use value_markers::ValueMarkers;

pub use crate::style::v_slider::{
    Appearance, ClassicAppearance, ClassicHandle, ClassicRail, ModRangeAppearance,
    ModRangePlacement, RectAppearance, RectBipolarAppearance, StyleSheet, TextMarksAppearance,
    TextureAppearance, TickMarksAppearance,
};

static DEFAULT_WIDTH: f32 = 14.0;
static DEFAULT_SCALAR: f32 = 0.9575;
static DEFAULT_WHEEL_SCALAR: f32 = 0.01;
static DEFAULT_MODIFIER_SCALAR: f32 = 0.02;

/// A vertical slider GUI widget that controls a [`NormalParam`]
///
/// a [`VSlider`] will try to fill the vertical space of its container.
///
/// [`NormalParam`]: ../../core/normal_param/struct.NormalParam.html
/// [`VSlider`]: struct.VSlider.html
#[allow(missing_debug_implementations)]
pub struct VSlider<'a, Message, Theme>
where
    Theme: StyleSheet,
{
    normal_param: NormalParam,
    on_change: Box<dyn 'a + Fn(Normal) -> Message>,
    on_grab: Option<Box<dyn 'a + FnMut() -> Option<Message>>>,
    on_release: Option<Box<dyn 'a + FnMut() -> Option<Message>>>,
    scalar: f32,
    wheel_scalar: f32,
    modifier_scalar: f32,
    modifier_keys: keyboard::Modifiers,
    width: Length,
    height: Length,
    style: <Theme as StyleSheet>::Style,
    tick_marks: Option<&'a tick_marks::Group>,
    text_marks: Option<&'a text_marks::Group>,
    mod_range_1: Option<&'a ModulationRange>,
    mod_range_2: Option<&'a ModulationRange>,
}

impl<'a, Message, Theme> VSlider<'a, Message, Theme>
where
    Theme: StyleSheet,
{
    /// Creates a new [`VSlider`].
    ///
    /// It expects:
    ///   * the [`NormalParam`] of the [`VSlider`]
    ///   * a function that will be called when the [`VSlider`] is dragged.
    ///
    /// [`NormalParam`]: struct.NormalParam.html
    /// [`VSlider`]: struct.VSlider.html
    pub fn new<F>(normal_param: NormalParam, on_change: F) -> Self
    where
        F: 'a + Fn(Normal) -> Message,
    {
        VSlider {
            normal_param,
            on_change: Box::new(on_change),
            on_grab: None,
            on_release: None,
            scalar: DEFAULT_SCALAR,
            wheel_scalar: DEFAULT_WHEEL_SCALAR,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,
            modifier_keys: keyboard::Modifiers::CTRL,
            width: Length::Fixed(DEFAULT_WIDTH),
            height: Length::Fill,
            style: Default::default(),
            tick_marks: None,
            text_marks: None,
            mod_range_1: None,
            mod_range_2: None,
        }
    }

    /// Sets the grab message of the [`VSlider`].
    /// This is called when the mouse grabs from the slider.
    ///
    /// Typically, the user's interaction with the slider starts when this message is produced.
    /// This is useful for some environments so that external changes, such as automation,
    /// don't interfer with user's changes.
    pub fn on_grab(mut self, on_grab: impl 'a + FnMut() -> Option<Message>) -> Self {
        self.on_grab = Some(Box::new(on_grab));
        self
    }

    /// Sets the release message of the [`VSlider`].
    /// This is called when the mouse is released from the slider.
    ///
    /// Typically, the user's interaction with the slider is finished when this message is produced.
    /// This is useful if you need to spawn a long-running task from the slider's result, where
    /// the default on_change message could create too many events.
    pub fn on_release(mut self, on_release: impl 'a + FnMut() -> Option<Message>) -> Self {
        self.on_release = Some(Box::new(on_release));
        self
    }

    /// Sets the width of the [`VSlider`].
    /// The default width is `Length::Fixed(14)`.
    ///
    /// [`VSlider`]: struct.VSlider.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`VSlider`].
    /// The default height is `Length::Fill`.
    ///
    /// [`VSlider`]: struct.VSlider.html
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the style of the [`VSlider`].
    ///
    /// [`VSlider`]: struct.VSlider.html
    pub fn style(mut self, style: impl Into<<Theme as StyleSheet>::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the modifier keys of the [`VSlider`].
    ///
    /// The default modifier key is `Ctrl`.
    ///
    /// [`VSlider`]: struct.VSlider.html
    pub fn modifier_keys(mut self, modifier_keys: keyboard::Modifiers) -> Self {
        self.modifier_keys = modifier_keys;
        self
    }

    /// Sets the scalar to use when the user drags the slider per pixel.
    ///
    /// For example, a scalar of `0.5` will cause the slider to move half a
    /// pixel for every pixel the mouse moves.
    ///
    /// The default scalar is `0.9575`.
    ///
    /// [`VSlider`]: struct.VSlider.html
    pub fn scalar(mut self, scalar: f32) -> Self {
        self.scalar = scalar;
        self
    }

    /// Sets how much the [`Normal`] value will change for the [`VSlider`] per line scrolled
    /// by the mouse wheel.
    ///
    /// This can be set to `0.0` to disable the scroll wheel from moving the parameter.
    ///
    /// The default value is `0.01`
    ///
    /// [`VSlider`]: struct.VSlider.html
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn wheel_scalar(mut self, wheel_scalar: f32) -> Self {
        self.wheel_scalar = wheel_scalar;
        self
    }

    /// Sets the scalar to use when the user drags the slider while holding down
    /// the modifier key.
    ///
    /// For example, a scalar of `0.5` will cause the slider to move half a
    /// pixel for every pixel the mouse moves.
    ///
    /// The default scalar is `0.02`, and the default modifier key is `Ctrl`.
    ///
    /// [`VSlider`]: struct.VSlider.html
    pub fn modifier_scalar(mut self, scalar: f32) -> Self {
        self.modifier_scalar = scalar;
        self
    }

    /// Sets the tick marks to display. Note your [`StyleSheet`] must
    /// also implement `tick_marks_style(&self) -> Option<tick_marks::Style>` for
    /// them to display (which the default style does).
    ///
    /// [`StyleSheet`]: ../../style/v_slider/trait.StyleSheet.html
    pub fn tick_marks(mut self, tick_marks: &'a tick_marks::Group) -> Self {
        self.tick_marks = Some(tick_marks);
        self
    }

    /// Sets the text marks to display. Note your [`StyleSheet`] must
    /// also implement `text_marks_style(&self) -> Option<text_marks::Style>` for
    /// them to display (which the default style does).
    ///
    /// [`StyleSheet`]: ../../style/v_slider/trait.StyleSheet.html
    pub fn text_marks(mut self, text_marks: &'a text_marks::Group) -> Self {
        self.text_marks = Some(text_marks);
        self
    }

    /// Sets a [`ModulationRange`] to display. Note your [`StyleSheet`] must
    /// also implement `mod_range_style(&self) -> Option<ModRangeStyle>` for
    /// them to display.
    ///
    /// [`ModulationRange`]: ../../core/struct.ModulationRange.html
    /// [`StyleSheet`]: ../../style/v_slider/trait.StyleSheet.html
    pub fn mod_range(mut self, mod_range: &'a ModulationRange) -> Self {
        self.mod_range_1 = Some(mod_range);
        self
    }

    /// Sets a second [`ModulationRange`] to display. Note your [`StyleSheet`] must
    /// also implement `mod_range_style_2(&self) -> Option<ModRangeStyle>` for
    /// them to display.
    ///
    /// [`ModulationRange`]: ../../core/struct.ModulationRange.html
    /// [`StyleSheet`]: ../../style/v_slider/trait.StyleSheet.html
    pub fn mod_range_2(mut self, mod_range: &'a ModulationRange) -> Self {
        self.mod_range_1 = Some(mod_range);
        self
    }

    fn move_virtual_slider(&mut self, state: &mut State, mut normal_delta: f32) -> SliderStatus {
        if normal_delta.abs() < f32::EPSILON {
            return SliderStatus::Unchanged;
        }

        if state.pressed_modifiers.contains(self.modifier_keys) {
            normal_delta *= self.modifier_scalar;
        }

        self.normal_param
            .value
            .set_clipped(state.continuous_normal - normal_delta);
        state.continuous_normal = self.normal_param.value.as_f32();

        SliderStatus::Moved
    }

    fn maybe_fire_on_grab(&mut self, shell: &mut Shell<'_, Message>) {
        if let Some(message) = self.on_grab.as_mut().and_then(|on_grab| on_grab()) {
            shell.publish(message);
        }
    }

    fn fire_on_change(&self, shell: &mut Shell<'_, Message>) {
        shell.publish((self.on_change)(self.normal_param.value));
    }

    fn maybe_fire_on_release(&mut self, shell: &mut Shell<'_, Message>) {
        if let Some(message) = self.on_release.as_mut().and_then(|on_release| on_release()) {
            shell.publish(message);
        }
    }
}

impl<'a, Message, Theme> Widget<Message, Theme, Renderer> for VSlider<'a, Message, Theme>
where
    Theme: StyleSheet,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new(self.normal_param.value))
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: self.height,
        }
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(limits.resolve(self.width, self.width, Size::ZERO))
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        let state = state.state.downcast_mut::<State>();

        let is_over = cursor.is_over(layout.bounds());

        // Update state after a discontinuity
        if state.dragging_status.is_none() && state.prev_normal != self.normal_param.value {
            state.prev_normal = self.normal_param.value;
            state.continuous_normal = self.normal_param.value.as_f32();
        }

        match event {
            Event::Mouse(mouse::Event::CursorMoved { position })
            | Event::Touch(touch::Event::FingerMoved { position, .. }) => {
                if state.dragging_status.is_some() {
                    let bounds = layout.bounds();
                    if bounds.height > 0.0 {
                        let normal_delta =
                            (position.y - state.prev_drag_y) / bounds.height * self.scalar;

                        state.prev_drag_y = if position.y <= bounds.y {
                            bounds.y
                        } else {
                            position.y.min(bounds.y + bounds.height)
                        };

                        if self.move_virtual_slider(state, normal_delta).was_moved() {
                            self.fire_on_change(shell);

                            state
                                .dragging_status
                                .as_mut()
                                .expect("dragging_status taken")
                                .moved();
                        }

                        return event::Status::Captured;
                    }
                }
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if self.wheel_scalar == 0.0 {
                    return event::Status::Ignored;
                }

                if is_over {
                    let lines = match delta {
                        mouse::ScrollDelta::Lines { y, .. } => y,
                        mouse::ScrollDelta::Pixels { y, .. } => {
                            if y > 0.0 {
                                1.0
                            } else if y < 0.0 {
                                -1.0
                            } else {
                                0.0
                            }
                        }
                    };

                    if lines != 0.0 {
                        let normal_delta = -lines * self.wheel_scalar;

                        if self.move_virtual_slider(state, normal_delta).was_moved() {
                            if state.dragging_status.is_none() {
                                self.maybe_fire_on_grab(shell);
                            }

                            self.fire_on_change(shell);

                            if let Some(slider_status) = state.dragging_status.as_mut() {
                                // Widget was grabbed => keep it grabbed
                                slider_status.moved();
                            } else {
                                self.maybe_fire_on_release(shell);
                            }
                        }

                        return event::Status::Captured;
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if is_over {
                    let click = mouse::Click::new(
                        cursor.position().unwrap(),
                        mouse::Button::Left,
                        state.last_click,
                    );

                    match click.kind() {
                        mouse::click::Kind::Single => {
                            self.maybe_fire_on_grab(shell);

                            state.dragging_status = Some(Default::default());
                            state.prev_drag_y = cursor.position().unwrap().y;
                        }
                        _ => {
                            // Reset to default

                            let prev_dragging_status = state.dragging_status.take();

                            if self.normal_param.value != self.normal_param.default {
                                if prev_dragging_status.is_none() {
                                    self.maybe_fire_on_grab(shell);
                                }

                                self.normal_param.value = self.normal_param.default;

                                self.fire_on_change(shell);

                                self.maybe_fire_on_release(shell);
                            } else if prev_dragging_status.is_some() {
                                self.maybe_fire_on_release(shell);
                            }
                        }
                    }

                    state.last_click = Some(click);

                    return event::Status::Captured;
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                if let Some(slider_status) = state.dragging_status.take() {
                    if self.on_grab.is_some() || slider_status.was_moved() {
                        // maybe fire on release if `on_grab` is defined
                        // so as to terminate the action, regardless of the actual user movement.
                        self.maybe_fire_on_release(shell);
                    }

                    return event::Status::Captured;
                }
            }
            Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::KeyPressed { modifiers, .. } => {
                    state.pressed_modifiers = modifiers;

                    return event::Status::Captured;
                }
                keyboard::Event::KeyReleased { modifiers, .. } => {
                    state.pressed_modifiers = modifiers;

                    return event::Status::Captured;
                }
                keyboard::Event::ModifiersChanged(modifiers) => {
                    state.pressed_modifiers = modifiers;

                    return event::Status::Captured;
                }
            },
            _ => {}
        }

        event::Status::Ignored
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = state.state.downcast_ref::<State>();
        let bounds = layout.bounds();
        let is_over = cursor.is_over(bounds);

        let appearance = if state.dragging_status.is_some() {
            theme.dragging(&self.style)
        } else if is_over {
            theme.hovered(&self.style)
        } else {
            theme.active(&self.style)
        };

        let bounds = Rectangle {
            x: bounds.x.round(),
            y: bounds.y.round(),
            width: bounds.width.round(),
            height: bounds.height.round(),
        };

        let value_markers = ValueMarkers {
            tick_marks: self.tick_marks,
            text_marks: self.text_marks,
            mod_range_1: self.mod_range_1,
            mod_range_2: self.mod_range_2,
            tick_marks_style: theme.tick_marks_appearance(&self.style),
            text_marks_style: theme.text_marks_appearance(&self.style),
            mod_range_style_1: theme.mod_range_appearance(&self.style),
            mod_range_style_2: theme.mod_range_appearance_2(&self.style),
        };

        let normal = self.normal_param.value;

        match appearance {
            Appearance::Texture(style) => draw::texture_style(
                renderer,
                normal,
                &bounds,
                style,
                &value_markers,
                //tick_marks_cache,
                //text_marks_cache,
            ),
            Appearance::Classic(style) => draw::classic_style(
                renderer,
                normal,
                &bounds,
                &style,
                &value_markers,
                //tick_marks_cache,
                //text_marks_cache,
            ),
            Appearance::Rect(style) => draw::rect_style(
                renderer,
                normal,
                &bounds,
                &style,
                &value_markers,
                //tick_marks_cache,
                //text_marks_cache,
            ),
            Appearance::RectBipolar(style) => draw::rect_bipolar_style(
                renderer,
                normal,
                &bounds,
                &style,
                &value_markers,
                //tick_marks_cache,
                //text_marks_cache,
            ),
        }
    }
}

impl<'a, Message, Theme> From<VSlider<'a, Message, Theme>> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + StyleSheet,
{
    fn from(v_slider: VSlider<'a, Message, Theme>) -> Self {
        Self::new(v_slider)
    }
}
