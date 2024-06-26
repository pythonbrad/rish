// Translation of plotdemos9.tcl
//
// Windrose not done as does not work in tklib 0.7!

use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();
    root.title("plotdemos9.rs");

    // bands in two directions
    let canvas = rish::make_canvas(&root);
    canvas.background("white");
    canvas.pack().layout();

    let chart = rish::make_x_y(&canvas, (0.0, 10.0, 2.0), (0.0, 40.0, 10.0))
        .plot();
    chart.plot("data", (1.0, 10.0));
    chart.plot("data", (6.0, 20.0));
    chart.plot("data", (9.0, 10.0));

    chart.draw_x_band(15.0, 25.0);
    chart.draw_y_band(3.0, 5.0);

    // label dots and vertical text
    let canvas = rish::make_canvas(&root);
    canvas.background("white");
    canvas.pack().layout();

    let chart = rish::make_x_y(&canvas, (0.0, 10.0, 2.0), (0.0, 40.0, 10.0))
        .plot();
    chart.draw_labelled_dot((3.0, 10.0), "Point 1", rish::Location::West);
    chart.draw_labelled_dot((6.0, 20.0), "Point 2", rish::Location::East);
    chart.draw_labelled_dot((9.0, 10.0), "Point 3", rish::Location::North);
    chart.draw_labelled_dot((9.0, 30.0), "Point 4", rish::Location::South);

    chart.v_title("Vertical axis label");

    rish::mainloop();
}
