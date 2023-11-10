use rstk::*;

fn main() {
    let root = rstk::start_wish().unwrap();

    root.title("popup-menu-example.rs");

    let menu = rstk::make_menu(&root);
    for item in ["One", "Two", "Three"].iter() {
        menu.command()
            .label(item)
            .command(move || println!("You clicked {}", item))
            .add();
    }

    root.bind("<3>", move |event| {
        menu.popup(event.root_x, event.root_y);
    });

    rstk::mainloop();
}