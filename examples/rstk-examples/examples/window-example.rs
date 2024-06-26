use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();

    root.title("window-example.rs");
    let open_1 = rish::make_button(&root);
    open_1.text("Open 1");
    let open_2 = rish::make_button(&root);
    open_2.text("Open 2");

    println!("Geometry of root at start: {}", root.geometry_get());

    open_1.grid().row(0).column(0).layout();
    open_2.grid().row(0).column(1).layout();

    println!("Geometry of root when filled: {}", root.geometry_get());

    {
        let root = root.clone();

        open_1.command(move || {
            let new_window = rish::make_toplevel(&root);
            new_window.title("Window 1");
            new_window.iconify();
        });
    }

    {
        let root = root.clone();

        open_2.command(move || {
            let new_window = rish::make_toplevel(&root);
            new_window.title("Window 2");
            new_window.resizable(false, false);
        });
    }

    root.on_close(|| {
        let result = rish::message_box()
            .title("Really close?")
            .message("Still a chance to say no")
            .type_buttons(rish::DialogType::YesNo)
            .show();

        if result == "yes" {
            rish::end_wish();
        }
    });

    rish::mainloop();
}
