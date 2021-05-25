//! Functions and definitions applying to all widgets or specific sub-classes 
//! of widgets.
//!

use super::image;
use super::wish;

/// Struct holding information from a bound event, 
/// returned as a parameter to the bound closure.
pub struct TkEvent {
    /// x-coordinate relative to current widget
    pub x: i32,
    /// y-coordinate relative to current widget
    pub y: i32,
    /// x-coordinate relative to screen
    pub root_x: i32,
    /// y-coordinate relative to screen
    pub root_y: i32,
    /// vertical screen distance, e.g. for a drag event
    pub height: i32,
    /// horizontal screen distance, e.g. for a drag event
    pub width: i32,
    /// Numeric code representing key for current event
    pub key_code: u32,
    /// Symbol representing key for current event, e.g. "space", "e".
    pub key_symbol: String,
    /// Number of mouse button in current event: 1 for left, 3 for right, etc.
    pub mouse_button: u32,
}

/// Common trait for container widgets. Child widgets should implement the `id`
/// method. The remaining methods are standard Tk methods and convenient, 
/// type-save versions of them.
pub trait TkWidget {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str;

    /// Binds a command to this widget to call on given event pattern
    fn bind(&self, pattern: &str, command: impl Fn(TkEvent)->() + Send + 'static) {
        bind_to(&self.id(), pattern, command);
    }

    /// Retrieve the value of a configuration option
    /// as a string. 
    ///
    /// * `option` - the option to read
    ///
    fn cget(&self, option: &str) -> String {
        let msg = format!("{} cget {}", self.id(), option);
        wish::eval_wish(&msg)
    }

    /// Used to change properties of a widget. 
    /// This function can be used to directly configure
    /// the widget using an option-value string pair: 
    ///
    /// * `option` - the option to change
    /// * `value` - the value to change it to
    ///
    fn configure(&self, option: &str, value: &str) {
        configure(&self.id(), option, value);
    }

    /// Destroys a widget and its children.
    fn destroy(&self) {
        let msg = format!("destroy {}", self.id());
        wish::tell_wish(&msg);
    }

    /// winfo retrieves information about widget.
    ///
    fn winfo(&self, option: &str) -> String {
        let msg = format!("winfo {} {}", option, self.id());
        wish::eval_wish(&msg)
    }

    // -- TODO should be here, or more specific?

    /// Makes this widget the focus window (e.g. for key presses)
    fn focus(&self) {
        let msg = format!("focus {}", self.id());
        wish::tell_wish(&msg);
    }

    // -- winfo functions

    /// Returns the widget x position in pixels, within its parent.
    fn position_x(&self) -> u32 {
        let msg = format!("winfo x {}", self.id());
        let result = wish::eval_wish(&msg);
        if let Ok(value) = result.parse::<u32>() {
            value
        } else {
            0
        }
    }

    /// Returns the widget y position in pixels, within its parent.
    fn position_y(&self) -> u32 {
        let msg = format!("winfo y {}", self.id());
        let result = wish::eval_wish(&msg);
        if let Ok(value) = result.parse::<u32>() {
            value
        } else {
            0
        }
    }

    /// Returns the widget height in pixels.
    fn widget_height(&self) -> u32 {
        let msg = format!("winfo height {}", self.id());
        let result = wish::eval_wish(&msg);
        if let Ok(value) = result.parse::<u32>() {
            value
        } else {
            0
        }
    }

    /// Returns the widget width in pixels.
    fn widget_width(&self) -> u32 {
        let msg = format!("winfo width {}", self.id());
        let result = wish::eval_wish(&msg);
        if let Ok(value) = result.parse::<u32>() {
            value
        } else {
            0
        }
    }

    /// Returns the position of the mouse on screen of widget as (x,y).
    fn mouse_position(&self) -> (i32, i32) {
        (self.mouse_x(), self.mouse_y())
    }

    /// Gives the x position of the mouse on screen of widget.
    fn mouse_x(&self) -> i32 {
        let msg = format!("winfo pointerx {}", self.id());
        let result = wish::eval_wish(&msg);
        if let Ok(value) = result.parse::<i32>() {
            value
        } else {
            -1
        }
    }

    /// Gives the y position of the mouse on screen of widget.
    fn mouse_y(&self) -> i32 {
        let msg = format!("winfo pointery {}", self.id());
        let result = wish::eval_wish(&msg);
        if let Ok(value) = result.parse::<i32>() {
            value
        } else {
            -1
        }
    }

    /// Height of screen of widget in pixels.
    fn screen_height(&self) -> u32 {
        let msg = format!("winfo screenheight {}", self.id());
        let result = wish::eval_wish(&msg);
        if let Ok(value) = result.parse::<u32>() {
            value
        } else {
            0
        }
    }

    /// Height of screen of widget in millimetres.
    fn screen_height_mm(&self) -> u32 {
        let msg = format!("winfo screenmmheight {}", self.id());
        let result = wish::eval_wish(&msg);
        if let Ok(value) = result.parse::<u32>() {
            value
        } else {
            0
        }
    }

    /// Width of screen of widget in pixels.
    fn screen_width(&self) -> u32 {
        let msg = format!("winfo screenwidth {}", self.id());
        let result = wish::eval_wish(&msg);
        if let Ok(value) = result.parse::<u32>() {
            value
        } else {
            0
        }
    }

    /// Width of screen of widget in millimetres.
    fn screen_width_mm(&self) -> u32 {
        let msg = format!("winfo screenmmwidth {}", self.id());
        let result = wish::eval_wish(&msg);
        if let Ok(value) = result.parse::<u32>() {
            value
        } else {
            0
        }
    }

    // -- stacking order

    /// Lowers the widget in stacking order.
    fn lower(&self) {
        let msg = format!("lower {}", self.id());
        wish::tell_wish(&msg);
    }

    /// Raises the widget in stacking order.
    fn raise(&self) {
        let msg = format!("raise {}", self.id());
        wish::tell_wish(&msg);
    }

    // -- for widgets that can contain other widgets

    /// Sets property for a given column of the grid layout 
    /// contained within this widget.
    fn grid_configure_column(&self, index: u32, option: &str, value: &str) {
        let msg = format!("grid columnconfigure {} {} -{} {{{}}}", self.id(), index, option, value);
        wish::tell_wish(&msg);
    }

    /// Sets property for a given row of the grid layout 
    /// contained within this widget.
    fn grid_configure_row(&self, index: u32, option: &str, value: &str) {
        let msg = format!("grid rowconfigure {} {} -{} {{{}}}", self.id(), index, option, value);
        wish::tell_wish(&msg);
    }
}


/// A set of common functions used in all label, button and similar widgets.
///
/// * also see the Tk [manual](https://tcl.tk/man/tcl/TkCmd/ttk_widget.htm#M6)
///
pub trait TkLabelOptions: TkWidget {
    /// Specifies how to arrange the text relative to the image.
    fn compound(&self, value: Compound) {
        compound(&self.id(), value);
    }

    /// Specifies the font to use for text.
    fn font(&self, definition: &str) {
        configure(&self.id(), "font", definition);
    }

    /// Specifies the foreground (text) colour.
    fn foreground(&self, colour: &str) {
        configure(&self.id(), "foreground", colour);
    }

    /// Sets an image to display on the widget.
    fn image(&self, image: &image::TkImage) {
        configure(&self.id(), "image", &image.id);
    }

    /// Sets space around the widget. Takes 
    /// an array of up to four values, specifying: 
    ///
    /// * \[all]
    /// * [left-right top-bottom]
    /// * [left top-bottom right]
    /// * [left top right bottom]
    fn padding(&self, values: &[u32]) {
        padding(&self.id(), values);
    }

    /// Sets the text label for the widget.
    fn text(&self, value: &str) {
        configure(&self.id(), "text", value);
    }

    /// Underlines the character at the given index position.
    fn underline(&self, index: u32) {
        configure(&self.id(), "underline", &index.to_string());
    }

    /// Sets the width of the widget, in characters
    fn width(&self, value: i32) {
        let msg = format!("{} configure -width {{{}}}", self.id(), value);
        wish::tell_wish(&msg);
    }
}


// --------------------------------------------------------------------------
// Enums to type-check values

pub enum Anchor {
    N, 
    NE, 
    E, 
    SE, 
    S, 
    SW, 
    W, 
    NW, 
    Center,
    Centre,
}

pub enum Compound {
    Bottom,
    Center,
    Centre,
    Image,
    Left,
    None,
    Right,
    Text,
    Top,
}

#[derive(Clone)]
pub enum DialogType {
    AbortRetryIgnore,
    Ok,
    OkCancel,
    RetryCancel,
    YesNo,
    YesNoCancel,
}

#[derive(Clone)]
pub enum IconImage {
    Error,
    Information,
    Question,
    Warning,
}

pub enum Justify {
    Center,
    Centre,
    Left,
    Right,
}

pub enum Relief {
    Flat,
    Groove,
    Raised,
    Ridge,
    Solid,
    Sunken,
}

/// The kinds of activity state for a widget, e.g. if it is currently
/// available to use or disabled.
pub enum State {
    /// Used, e.g., for buttons, to highlight when a mouse pointer is over them.
    Active,
    /// Used to prevent user-interaction with a widget.
    Disabled,
    /// The usual state of a widget, permitting user interactions.
    Normal,
    /// State cannot be changed, for those widgets with editable state.
    Readonly,
}

// --------------------------------------------------------------------------
// Internal functions for within crate use

pub(super) fn bind_to(tag: &str, pattern: &str, command: impl Fn(TkEvent)->() + Send + 'static) {
    // tag+pattern used as identifier, as multiple commands can be bound to each entity
    let tag_pattern = format!("{}{}", tag, pattern);  // TODO ? remove ':' ?
    wish::add_callback1_event(&tag_pattern, wish::mk_callback1_event(command));
    let msg = format!("bind {} {} {{ puts cb1e:{}:%x:%y:%X:%Y:%h:%w:%k:%K:%b ; flush stdout }}",
                      tag, pattern, tag_pattern);
    wish::tell_wish(&msg);
}

pub(super) fn compound(wid: &str, value: Compound) {
    let value = match value {
        Compound::Bottom => "bottom",
        Compound::Center | Compound::Centre => "center",
        Compound::Image => "image",
        Compound::Left => "left",
        Compound::None => "none",
        Compound::Right => "right",
        Compound::Text => "text",
        Compound::Top => "top",
    };
    configure(wid, "compound", value);
}

pub(super) fn configure(wid: &str, option: &str, value: &str) {
    let msg = format!("{} configure -{} {{{}}}", wid, option, value);
    wish::tell_wish(&msg);
}

pub(super) fn justify(wid: &str, value: Justify) {
    let value = match value {
        Justify::Left => "left",
        Justify::Center | Justify::Centre => "center",
        Justify::Right => "right",
    };
    configure(wid, "justify", value);
}

pub(super) fn padding(wid: &str, values: &[u32]) {
    let mut value_str = String::from("");
    for i in values.iter() {
        value_str.push_str(&i.to_string());
        value_str.push(' ');
    }
    configure(wid, "padding", &value_str);
}

pub(super) fn relief(wid: &str, value: Relief) {
    let value = match value {
        Relief::Flat => "flat",
        Relief::Groove => "groove",
        Relief::Raised => "raised",
        Relief::Ridge => "ridge",
        Relief::Solid => "solid",
        Relief::Sunken => "sunken",
    };
    configure(wid, "relief", value);
}

pub(super) fn state(wid: &str, value: State) {
    let value = match value {
        State::Active => "active",
        State::Disabled => "disabled",
        State::Normal => "normal",
        State::Readonly => "readonly",
    };
    configure(wid, "state", value);
}

// --------------------------------------------------------------------------

/// Binds command for event pattern to _all_ widgets. 
pub fn bind(pattern: &str, command: impl Fn(TkEvent)->() + Send + 'static) {
    bind_to("all", pattern, command);
}