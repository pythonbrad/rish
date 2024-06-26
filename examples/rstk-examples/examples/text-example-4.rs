use rish::*;

/// Adds 'msg' to given 'log_text', but ensures only 24-lines are shown.
fn write_to_log(msg: &str, log_text: &rish::TkText) {
    let num_lines = log_text.get_to_end((1,0)).lines().count();

    log_text.state(rish::State::Normal);
    if num_lines == 24 { // delete first line, if there are 24 already
        log_text.delete((1,0), (2,0));
    }
    log_text.insert_end(msg);
    log_text.insert_end("\n");
    log_text.state(rish::State::Disabled);
}

fn write_messages(count: u32, log_text: rish::TkText) {
    if count <= 100 {
        rish::after(100, move || {
            write_to_log(&format!("Log message {} of 100", count), &log_text);
            write_messages(count + 1, log_text.clone());
        });
    }
}

fn main() {
    let root = rish::start_wish().unwrap();
    root.title("text-example-4.rs");

    let log_text = rish::make_text(&root);
    log_text.height(24);
    log_text.width(80);
    log_text.wrap(rish::Wrapping::None);
    log_text.state(rish::State::Disabled);

    log_text.grid().sticky(rish::Sticky::NESW).layout();
    root.grid_configure_column(0, "weight", "1");
    root.grid_configure_row(0, "weight", "1");

    write_messages(1, log_text.clone());

    rish::mainloop();
}
