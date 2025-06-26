use TerimalRtdm::*;

fn main() {
    let mut app = App::new(); // Holds all interface state variables.
    clear(&mut app); // Clear the screen competely.
    raw_mode(true); // Enabled for correct showing of elements at specific positions.
    show_cursor(false); // By default it is set to show. The cursor is off, since we don't need to move it.

    loop {
        Text::new().show(&mut app, "Hello world", pos!(0, 0));

        // Example exit key
        if Key::o().pressed(&mut app, KeyType::Esc) {
            break;
        }

        render(&app);
        collect_presses(&mut app);
    }

    restore_terminal(); // Should be done at any exit of your program to restore the terminal defaults.
}
