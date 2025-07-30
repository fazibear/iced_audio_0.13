//! Display an interactive rotating knob that controls a [`NormalParam`]
//!
//! [`NormalParam`]: ../core/normal_param/struct.NormalParam.html
use iced::{
    advanced::{
        graphics::core::keyboard,
        layout, mouse,
        renderer::{self, Quad},
        widget::Tree,
        Layout, Widget,
    },
    Border, Color, Element, Length, Rectangle, Shadow, Size, Theme,
};

/// Moved status for the virtual sliders.
///
/// This allows tracking the virtual slider actual movements
/// thus preventing some events from unnecessary being emitted.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub(crate) enum SliderStatus {
    Moved,
    #[default]
    Unchanged,
}

impl SliderStatus {
    /// Sets the slider as moved.
    pub(crate) fn moved(&mut self) {
        *self = SliderStatus::Moved;
    }

    /// Whether the slider was moved.
    pub(crate) fn was_moved(self) -> bool {
        matches!(self, SliderStatus::Moved)
    }
}

use crate::core::{ModulationRange, Normal, NormalParam};
//use crate::native::{text_marks, tick_marks, SliderStatus};
use crate::style::knob::StyleSheet;

static DEFAULT_SIZE: f32 = 30.0;
static DEFAULT_SCALAR: f32 = 0.00385;
static DEFAULT_WHEEL_SCALAR: f32 = 0.01;
static DEFAULT_MODIFIER_SCALAR: f32 = 0.02;

/// A rotating knob GUI widget that controls a [`NormalParam`]
///
/// [`NormalParam`]: ../../core/normal_param/struct.NormalParam.html
#[allow(missing_debug_implementations)]
pub struct Knob<'a, Message, Theme>
where
    Theme: StyleSheet,
{
    normal_param: NormalParam,
    size: Length,
    on_change: Box<dyn 'a + Fn(Normal) -> Message>,
    on_grab: Option<Box<dyn 'a + FnMut() -> Option<Message>>>,
    on_release: Option<Box<dyn 'a + FnMut() -> Option<Message>>>,
    scalar: f32,
    wheel_scalar: f32,
    modifier_scalar: f32,
    modifier_keys: keyboard::Modifiers,
    bipolar_center: Option<Normal>,
    style: <Theme as StyleSheet>::Style,
    // tick_marks: Option<&'a tick_marks::Group>,
    // text_marks: Option<&'a text_marks::Group>,
    mod_range_1: Option<&'a ModulationRange>,
    mod_range_2: Option<&'a ModulationRange>,
}

impl<'a, Message, Theme> Knob<'a, Message, Theme>
where
    Theme: StyleSheet,
{
    /// Creates a new [`Knob`].
    ///
    /// It expects:
    ///   * the [`NormalParam`] of the [`Knob`]
    ///   * a function that will be called when the [`Knob`] is turned.
    ///
    /// [`NormalParam`]: struct.NormalParam.html
    /// [`Knob`]: struct.Knob.html
    pub fn new<F>(normal_param: NormalParam, on_change: F) -> Self
    where
        F: 'a + Fn(Normal) -> Message,
    {
        Knob {
            normal_param,
            size: Length::Fixed(DEFAULT_SIZE),
            on_change: Box::new(on_change),
            on_grab: None,
            on_release: None,
            scalar: DEFAULT_SCALAR,
            wheel_scalar: DEFAULT_WHEEL_SCALAR,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,
            modifier_keys: keyboard::Modifiers::CTRL,
            bipolar_center: None,
            style: Default::default(),
            //            tick_marks: None,
            //          text_marks: None,
            mod_range_1: None,
            mod_range_2: None,
        }
    }

    /// Sets the grab message of the [`Knob`].
    /// This is called when the mouse grabs from the knob.
    ///
    /// Typically, the user's interaction with the knob starts when this message is produced.
    /// This is useful for some environments so that external changes, such as automation,
    /// don't interfer with user's changes.
    pub fn on_grab(mut self, on_grab: impl 'a + FnMut() -> Option<Message>) -> Self {
        self.on_grab = Some(Box::new(on_grab));
        self
    }

    /// Sets the release message of the [`Knob`].
    /// This is called when the mouse is released from the knob.
    ///
    /// Typically, the user's interaction with the knob is finished when this message is produced.
    /// This is useful if you need to spawn a long-running task from the knob's result, where
    /// the default on_change message could create too many events.
    pub fn on_release(mut self, on_release: impl 'a + FnMut() -> Option<Message>) -> Self {
        self.on_release = Some(Box::new(on_release));
        self
    }

    /// Sets the diameter of the [`Knob`]. The default size is
    /// `Length::from(Length::Fixed(31))`.
    ///
    /// [`Knob`]: struct.Knob.html
    pub fn size(mut self, size: Length) -> Self {
        self.size = size;
        self
    }

    /// Sets the style of the [`Knob`].
    ///
    /// [`Knob`]: struct.Knob.html
    pub fn style(mut self, style: impl Into<<Theme as StyleSheet>::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Sets how much the [`Normal`] value will change for the [`Knob`] per `y`
    /// pixel movement of the mouse.
    ///
    /// The default value is `0.00385`
    ///
    /// [`Knob`]: struct.Knob.html
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn scalar(mut self, scalar: f32) -> Self {
        self.scalar = scalar;
        self
    }

    /// Sets how much the [`Normal`] value will change for the [`Knob`] per line scrolled
    /// by the mouse wheel.
    ///
    /// This can be set to `0.0` to disable the scroll wheel from moving the parameter.
    ///
    /// The default value is `0.01`
    ///
    /// [`Knob`]: struct.Knob.html
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn wheel_scalar(mut self, wheel_scalar: f32) -> Self {
        self.wheel_scalar = wheel_scalar;
        self
    }

    /// Sets the modifier keys of the [`Knob`].
    ///
    /// The default modifier key is `Ctrl`.
    ///
    /// [`Knob`]: struct.Knob.html
    pub fn modifier_keys(mut self, modifier_keys: keyboard::Modifiers) -> Self {
        self.modifier_keys = modifier_keys;
        self
    }

    /// Sets the scalar to use when the user drags the knobs while holding down
    /// the modifier key. This is multiplied to the value set by
    /// `Knob::scalar()` (which the default is `0.00385`).
    ///
    /// For example, a `modifier_scalar` of `0.5` will cause the knob to turn
    /// half as fast when the modifier key is down.
    ///
    /// The default `modifier_scalar` is `0.02`, and the default modifier key
    /// is `Ctrl`.
    ///
    /// [`Knob`]: struct.Knob.html
    pub fn modifier_scalar(mut self, scalar: f32) -> Self {
        self.modifier_scalar = scalar;
        self
    }

    /// Sets the tick marks to display. Note your [`StyleSheet`] must
    /// also implement `tick_marks_style(&self) -> Option<tick_marks::Style>` for
    /// them to display (which the default style does).
    ///
    /// [`StyleSheet`]: ../../style/knob/trait.StyleSheet.html
    // pub fn tick_marks(mut self, tick_marks: &'a tick_marks::Group) -> Self {
    //     self.tick_marks = Some(tick_marks);
    //     self
    // }

    /// Sets the text marks to display. Note your [`StyleSheet`] must
    /// also implement `text_marks_style(&self) -> Option<text_marks::Style>` for
    /// them to display (which the default style does).
    ///
    /// [`StyleSheet`]: ../../style/knob/trait.StyleSheet.html
    // pub fn text_marks(mut self, text_marks: &'a text_marks::Group) -> Self {
    //     self.text_marks = Some(text_marks);
    //     self
    // }

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

    /// Sets the value to be considered the center of the [`Knob`]. Only has
    /// an effect when using [`ArcBipolarStyle`].
    ///
    /// [`Knob`]: struct.Knob.html
    /// [`ArcBipolarStyle`]: ../../style/knob/struct.ArcBipolarStyle.html
    pub fn bipolar_center(mut self, bipolar_center: Normal) -> Self {
        self.bipolar_center = Some(bipolar_center);
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

    // fn maybe_fire_on_grab(&mut self, shell: &mut Shell<'_, Message>) {
    //     if let Some(message) = self.on_grab.as_mut().and_then(|on_grab| on_grab()) {
    //         shell.publish(message);
    //     }
    // }
    //
    // fn fire_on_change(&self, shell: &mut Shell<'_, Message>) {
    //     shell.publish((self.on_change)(self.normal_param.value));
    // }
    //
    // fn maybe_fire_on_release(&mut self, shell: &mut Shell<'_, Message>) {
    //     if let Some(message) = self.on_release.as_mut().and_then(|on_release| on_release()) {
    //         shell.publish(message);
    //     }
    // }
}

/// The local state of a [`Knob`].
///
/// [`Knob`]: struct.Knob.html
#[derive(Debug, Clone)]
struct State {
    dragging_status: Option<SliderStatus>,
    prev_drag_y: f32,
    prev_normal: Normal,
    continuous_normal: f32,
    pressed_modifiers: keyboard::Modifiers,
    last_click: Option<mouse::Click>,
    //    tick_marks_cache: crate::graphics::tick_marks::PrimitiveCache,
    //    text_marks_cache: crate::graphics::text_marks::PrimitiveCache,
}

impl State {
    /// Creates a new [`Knob`] state.
    ///
    /// It expects:
    /// * current [`Normal`] value for the [`Knob`]
    ///
    /// [`Normal`]: ../../core/normal/struct.Normal.html
    /// [`Knob`]: struct.Knob.html
    fn new(normal: Normal) -> Self {
        Self {
            dragging_status: None,
            prev_drag_y: 0.0,
            prev_normal: normal,
            continuous_normal: normal.as_f32(),
            pressed_modifiers: Default::default(),
            last_click: None,
            //          tick_marks_cache: Default::default(),
            //        text_marks_cache: Default::default(),
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Knob<'a, Message, Theme>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer,
    Theme: StyleSheet,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(iced::Size {
            width: 100.0,
            height: 100.0,
        })
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                border: Border {
                    color: Color::from_rgb(0.6, 0.8, 1.0),
                    width: 1.0,
                    radius: 10.0.into(),
                },
                shadow: Shadow::default(),
            },
            Color::from_rgb(0.0, 0.2, 0.4),
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Knob<'a, Message, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer,
    Theme: 'a + StyleSheet,
{
    fn from(knob: Knob<'a, Message, Theme>) -> Self {
        Self::new(knob)
    }
}
