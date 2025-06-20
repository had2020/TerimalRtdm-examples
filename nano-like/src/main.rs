use ::TerimalRtdm::*;

fn main() {
    let mut app = App::new();
    clear(&mut app);

    raw_mode(true);
    clear(&mut app);

    loop {
        collect_presses(&mut app);

        if Key::o().pressed(&mut app, "Esc") {
            Text::new()
                .foreground(Color::Black)
                .background(Color::Green)
                .style(Style::Bold)
                .show(&mut app, "Normal", pos!(0, 0));
        }

        if Key::o().pressed(&mut app, "Esc") {
            Text::new()
                .foreground(Color::Black)
                .background(Color::Green)
                .style(Style::Bold)
                .show(&mut app, "Normal", pos!(0, 0));
        }

        if Key::o().no_clear().pressed(&mut app, "w") {
            Mov::cur().wrap().dir(&mut app, Dir::Up, 1);
        }
        if Key::o().no_clear().pressed(&mut app, "s") {
            Mov::cur().wrap().dir(&mut app, Dir::Down, 1);
        }
        if Key::o().no_clear().pressed(&mut app, "a") {
            Mov::cur().wrap().dir(&mut app, Dir::Left, 1);
        }
        if Key::o().no_clear().pressed(&mut app, "d") {
            Mov::cur().wrap().dir(&mut app, Dir::Right, 1);
        }

        render(&app);
    }

    raw_mode(false);
}
