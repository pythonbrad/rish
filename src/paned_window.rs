//! Paned-window widget.
//! 
//! A container widget which contains multiple panes. 
//! Resizable sizers separate each pane.
//!
//! * also see the Tk [manual](http://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_panedwindow.htm)

use super::grid;
use super::widget;
use super::wish;

/// Refers to a paned-window widget
#[derive(Clone)]
pub struct TkPanedWindow {
    pub id: String,
}

/// Creates an instance of a horizontally aligned paned-window, in given 
/// parent. Child panes will be stacked left-to-right.
pub fn make_horizontal_paned_window(parent: &impl widget::TkWidget) -> TkPanedWindow {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::panedwindow {} -orient horizontal", id);
    wish::tell_wish(&msg);

    TkPanedWindow {
        id,
    }
}
/// Creates an instance of a vertically aligned paned-window, in given 
/// parent. Child panes will be stacked top-to-bottom.
pub fn make_vertical_paned_window(parent: &impl widget::TkWidget) -> TkPanedWindow {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::panedwindow {} -orient vertical", id);
    wish::tell_wish(&msg);

    TkPanedWindow {
        id,
    }
}

impl widget::TkWidget for TkPanedWindow {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}
impl grid::TkGridLayout for TkPanedWindow {
}

impl TkPanedWindow {
    /// Adds given widget to the pane.
    pub fn add(&self, pane: &impl widget::TkWidget) {
        let msg = format!("{} add {}", self.id, pane.id());
        wish::tell_wish(&msg);
    }

    /// Adds given widget to the pane with given weight.
    pub fn add_weighted(&self, pane: &impl widget::TkWidget, weight: u32) {
        let msg = format!("{} add {} -weight {}", self.id, pane.id(), weight);
        wish::tell_wish(&msg);
    }

    /// Removes given widget from the pane.
    pub fn forget(&self, pane: &impl widget::TkWidget) {
        let msg = format!("{} forget {}", self.id, pane.id());
        wish::tell_wish(&msg);
    }

    /// Height of paned window, in rows
    pub fn height(&self, height: u32) {
        widget::configure(&self.id, "height", &height.to_string());
    }

    /// Inserts given widget to the pane at given index position.
    pub fn insert(&self, index: u32, pane: &impl widget::TkWidget) {
        let msg = format!("{} insert {} {}", self.id, index, pane.id());
        wish::tell_wish(&msg);
    }

    /// Inserts given widget to the pane at given index position with given weight.
    pub fn insert_weighted(&self, index: u32, pane: &impl widget::TkWidget, weight: u32) {
        let msg = format!("{} insert {} {} -weight {}", self.id, index, pane.id(), weight);
        wish::tell_wish(&msg);
    }

    /// Width of paned window, in columns
    pub fn width(&self, width: u32) {
        widget::configure(&self.id, "width", &width.to_string());
    }
}