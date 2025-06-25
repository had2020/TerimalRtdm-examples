use ::TerimalRtdm::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut app = App::new();
    clear(&mut app);

    raw_mode(true);
    clear(&mut app);

    real_cursor_visibility(false);

    Text::new()
        .foreground(Color::BrightBlue)
        .style(Style::Bold)
        .show(
            &mut app,
            "A new crate to setup quick and sweet command line interfaces.",
            pos!(4, 8),
        );

    // animation
    let mut progress = 0;
    let mut last_color = Color::BrightMagenta;
    loop {
        let mut cycle_color = Color::Black;
        if last_color == Color::BrightMagenta {
            cycle_color = Color::BrightYellow
        } else {
            cycle_color = Color::BrightMagenta
        }
        last_color = cycle_color;

        Text::new().foreground(cycle_color).style(Style::Bold).show(
            &mut app,
            "   ▄▄▄▄▀ ▄███▄   █▄▄▄▄ ▄█ █▀▄▀█ ██   █     █▄▄▄▄    ▄▄▄▄▀ ██▄   █▀▄▀█ ",
            pos!(0, 1),
        );
        Text::new().foreground(cycle_color).style(Style::Bold).show(
            &mut app,
            "▀▀▀ █    █▀   ▀  █  ▄▀ ██ █ █ █ █ █  █     █  ▄▀ ▀▀▀ █    █  █  █ █ █ ",
            pos!(0, 2),
        );
        Text::new().foreground(cycle_color).style(Style::Bold).show(
            &mut app,
            "    █    ██▄▄    █▀▀▌  ██ █ ▄ █ █▄▄█ █     █▀▀▌      █    █   █ █ ▄ █ ",
            pos!(0, 3),
        );
        Text::new().foreground(cycle_color).style(Style::Bold).show(
            &mut app,
            "   █     █▄   ▄▀ █  █  ▐█ █   █ █  █ ███▄  █  █     █     █  █  █   █ ",
            pos!(0, 4),
        );
        Text::new().foreground(cycle_color).style(Style::Bold).show(
            &mut app,
            "  ▀      ▀███▀     █    ▐    █     █     ▀   █     ▀      ███▀     █  ",
            pos!(0, 5),
        );
        Text::new().foreground(cycle_color).style(Style::Bold).show(
            &mut app,
            "                  ▀         ▀     █         ▀                     ▀   ",
            pos!(0, 6),
        );
        Text::new().foreground(cycle_color).style(Style::Bold).show(
            &mut app,
            "                                 ▀                                   ",
            pos!(0, 7),
        );

        progress += 1;
        Text::new().foreground(cycle_color).style(Style::Bold).show(
            &mut app,
            "#",
            pos!(progress, 0),
        );

        sleep(Duration::from_millis(400));

        render(&app);
    }
}
