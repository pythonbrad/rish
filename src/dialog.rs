//! Dialogs
//!
//! All of the dialogs are built using a "builder" style. 
//! The named function creates a relevant struct value, and 
//! then functions on the relevant TkWIDGET struct alter the 
//! default values in that struct, until finally calling `show`
//! will set up and display the dialog.
//!
//! # Message boxes
//!
//! * also see the Tk [manual](http://www.tcl-lang.org/man/tcl8.6/TkCmd/messageBox.htm)
//!
//! The message-box is used to set up a simple alert, confirmation or 
//! information dialog:
//!
//! ```
//! rstk::message_box()
//!   .OPTION(VALUE) // 0 or more
//!   .show();
//! ```
//!
//! 1. `message_box` is called first, to get the TkMessageBox instance.
//! 2. `show` must be called last, to set up and display the dialog.
//! 3. zero or more options are added to the message box.
//!
//! # Chooser dialogs
//!
//! For colours, directories, files and fonts!
//!
//! Each dialog returns an Option type, with value None if cancelled.
//!
//! Tk manual pages:
//!
//! * [chooseColor](http://www.tcl-lang.org/man/tcl8.6/TkCmd/chooseColor.htm)
//! * [chooseDirectory](http://www.tcl-lang.org/man/tcl8.6/TkCmd/chooseDirectory.htm)
//! * [getOpenFile](http://www.tcl-lang.org/man/tcl8.6/TkCmd/getOpenFile.htm) (and getSaveFile)
//! * [fontchooser](http://www.tcl-lang.org/man/tcl8.6/TkCmd/fontchooser.htm)
//!

use super::toplevel;
use super::widgets;
use super::wish;

/// Refers to the settings for TkMessageBox.
#[derive(Clone)]
pub struct TkMessageBox {
    default: Option<String>,
    detail: Option<String>,
    icon: widgets::IconImage,
    message: Option<String>,
    parent: Option<String>,
    title: Option<String>,
    type_buttons: widgets::DialogType,
}

/// Creates a message box to complete in builder style.
pub fn message_box() -> TkMessageBox {
    TkMessageBox {
        default: None,
        detail: None,
        icon: widgets::IconImage::Error,
        message: None,
        parent: None,
        title: None,
        type_buttons: widgets::DialogType::Ok,
    }
}

impl TkMessageBox {

    /// Sets name used for default button.
    pub fn default(&mut self, name: &str) -> &mut Self {
        self.default = Some(String::from(name));
        self
    }

    /// Sets submessage to display, below message.
    pub fn detail(&mut self, text: &str) -> &mut Self {
        self.detail = Some(String::from(text));
        self
    }

    /// Sets icon type.
    pub fn icon(&mut self, value: widgets::IconImage) -> &mut Self {
        self.icon = value;
        self
    }

    /// Sets message to display.
    pub fn message(&mut self, text: &str) -> &mut Self {
        self.message = Some(String::from(text));
        self
    }

    /// Sets parent widget - dialog is usually shown relative to parent.
    pub fn parent(&mut self, value: &toplevel::TkTopLevel) -> &mut Self {
        self.parent = Some(String::from(&value.id));
        self
    }

    /// Sets title of the dialog window.
    pub fn title(&mut self, text: &str) -> &mut Self {
        self.title = Some(String::from(text));
        self
    }

    /// Sets type of dialog, which specifies its buttons.
    pub fn type_buttons(&mut self, value: widgets::DialogType) -> &mut Self {
        self.type_buttons = value;
        self
    }

    /// Once message box is defined, this function will finally show it.
    ///
    /// Returns a string for the name of the button pressed.
    ///
    pub fn show(&self) -> String {
        let mut msg = format!("puts [tk_messageBox ");

        if let Some(default) = &self.default {
            msg.push_str(&format!("-default {{{}}} ", default));
        }
        
        if let Some(detail) = &self.detail {
            msg.push_str(&format!("-detail {{{}}} ", detail));
        }

        let icon = match self.icon {
            widgets::IconImage::Error => "error",
            widgets::IconImage::Information => "info",
            widgets::IconImage::Question => "question",
            widgets::IconImage::Warning => "warning",
        };
        msg.push_str(&format!("-icon {} ", icon));

        if let Some(message) = &self.message {
            msg.push_str(&format!("-message {{{}}} ", message));
        }

        if let Some(parent) = &self.parent {
            msg.push_str(&format!("-parent {} ", parent));
        }

        if let Some(title) = &self.title {
            msg.push_str(&format!("-title {{{}}} ", title));
        }

        let buttons = match self.type_buttons {
            widgets::DialogType::AbortRetryIgnore => "abortretryignore",
            widgets::DialogType::Ok => "ok",
            widgets::DialogType::OkCancel => "okcancel",
            widgets::DialogType::RetryCancel => "retrycancel",
            widgets::DialogType::YesNo => "yesno",
            widgets::DialogType::YesNoCancel => "yesnocancel",
        };
        msg.push_str(&format!("-type {} ", buttons));
        msg.push_str("] ; flush stdout");

        wish::eval_wish(&msg)
    }
}

/// Refers to the settings for TkColourChooser.
#[derive(Clone)]
pub struct TkColourChooser {
    parent: Option<String>,
    title: Option<String>,
    initial: Option<String>,
}

/// Creates a colour-chooser to complete in builder style.
pub fn colour_chooser() -> TkColourChooser {
    TkColourChooser {
        parent: None,
        title: None,
        initial: None,
    }
}

/// Creates a colour-chooser to complete in builder style.
pub fn color_chooser() -> TkColourChooser {
    colour_chooser()
}

impl TkColourChooser {

    /// Sets parent widget - dialog is usually shown relative to parent.
    pub fn parent(&mut self, value: &toplevel::TkTopLevel) -> &mut Self {
        self.parent = Some(String::from(&value.id));
        self
    }

    /// Sets title of the dialog window.
    pub fn title(&mut self, text: &str) -> &mut Self {
        self.title = Some(String::from(text));
        self
    }

    /// Sets initial color of chooser.
    pub fn initial_color(&mut self, value: &str) -> &mut Self {
        self.initial_colour(value)
    }

    /// Sets initial colour of chooser.
    pub fn initial_colour(&mut self, value: &str) -> &mut Self {
        self.initial = Some(String::from(value));
        self
    }

    /// Once dialog is defined, this function will finally show it.
    ///
    /// Returns an option:
    ///
    /// * `Some(string)` - for the chosen colour, or
    /// * `None` - if cancel pressed.
    ///
    pub fn show(&self) -> Option<String> {
        let mut msg = format!("puts [tk_chooseColor ");

        if let Some(parent) = &self.parent {
            msg.push_str(&format!("-parent {} ", parent));
        }

        if let Some(title) = &self.title {
            msg.push_str(&format!("-title {{{}}} ", title));
        }

        if let Some(initial) = &self.initial {
            msg.push_str(&format!("-initialcolor {{{}}} ", initial));
        }

        msg.push_str("] ; flush stdout");

        let result = wish::eval_wish(&msg);
        if result == "" {
            None
        } else {
            Some(result)
        }
    }
}

/// Refers to the settings for TkDirectoryChooser.
#[derive(Clone)]
pub struct TkDirectoryChooser {
    parent: Option<String>,
    title: Option<String>,
    initial: Option<String>,
    must_exist: bool,
}

/// Creates a directory-chooser to complete in builder style.
pub fn directory_chooser() -> TkDirectoryChooser {
    TkDirectoryChooser {
        parent: None,
        title: None,
        initial: None,
        must_exist: false,
    }
}

impl TkDirectoryChooser {

    /// Sets parent widget - dialog is usually shown relative to parent.
    pub fn parent(&mut self, value: &toplevel::TkTopLevel) -> &mut Self {
        self.parent = Some(String::from(&value.id));
        self
    }

    /// Sets title of the dialog window.
    pub fn title(&mut self, text: &str) -> &mut Self {
        self.title = Some(String::from(text));
        self
    }

    /// Sets initial directory of chooser.
    pub fn initial_directory(&mut self, value: &str) -> &mut Self {
        self.initial = Some(String::from(value));
        self
    }

    /// Specify if directory must exist.
    pub fn must_exist(&mut self, value: bool) -> &mut Self {
        self.must_exist = value;
        self
    }

    /// Once dialog is defined, this function will finally show it.
    ///
    /// Returns an option:
    ///
    /// * `Some(string)` - for the chosen directory, or
    /// * `None` - if cancel pressed.
    ///
    pub fn show(&self) -> Option<String> {
        let mut msg = format!("puts [tk_chooseDirectory ");

        if let Some(parent) = &self.parent {
            msg.push_str(&format!("-parent {} ", parent));
        }

        if let Some(title) = &self.title {
            msg.push_str(&format!("-title {{{}}} ", title));
        }

        if let Some(initial) = &self.initial {
            msg.push_str(&format!("-initialdir {{{}}} ", initial));
        }

        if self.must_exist { // default is false, so only change for true
            msg.push_str(&format!("-mustexist 1 "));
        }

        msg.push_str("] ; flush stdout");

        let result = wish::eval_wish(&msg);
        if result == "" {
            None
        } else {
            Some(result)
        }
    }
}