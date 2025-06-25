use ::TerimalRtdm::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut app = App::new();
    clear(&mut app);

    raw_mode(true);
    clear(&mut app);

    real_cursor_visibility(false);

    // animation
    let mut last_color = Color::Blue;
    loop {
        let mut cycle_color = Color::Black;
        if last_color == Color::Blue {
            cycle_color = Color::Red
        } else {
            cycle_color = Color::Blue
        }
        last_color = cycle_color;

        Text::new().foreground(cycle_color).style(Style::Bold).show(
            &mut app,
            "   ▄▄▄▄▀ ▄███▄   █▄▄▄▄ ▄█ █▀▄▀█ ██   █     █▄▄▄▄    ▄▄▄▄▀ ██▄   █▀▄▀█ ",
            pos!(0, 0),
        );
        Text::new().foreground(cycle_color).style(Style::Bold).show(
            &mut app,
            "▀▀▀ █    █▀   ▀  █  ▄▀ ██ █ █ █ █ █  █     █  ▄▀ ▀▀▀ █    █  █  █ █ █ ",
            pos!(0, 1),
        );
        Text::new().foreground(cycle_color).style(Style::Bold).show(
            &mut app,
            "    █    ██▄▄    █▀▀▌  ██ █ ▄ █ █▄▄█ █     █▀▀▌      █    █   █ █ ▄ █ ",
            pos!(0, 2),
        );
        Text::new().foreground(cycle_color).style(Style::Bold).show(
            &mut app,
            "   █     █▄   ▄▀ █  █  ▐█ █   █ █  █ ███▄  █  █     █     █  █  █   █ ",
            pos!(0, 3),
        );
        Text::new().foreground(cycle_color).style(Style::Bold).show(
            &mut app,
            "  ▀      ▀███▀     █    ▐    █     █     ▀   █     ▀      ███▀     █  ",
            pos!(0, 4),
        );
        Text::new().foreground(cycle_color).style(Style::Bold).show(
            &mut app,
            "                  ▀         ▀     █         ▀                     ▀   ",
            pos!(0, 5),
        );
        Text::new().foreground(cycle_color).style(Style::Bold).show(
            &mut app,
            "                                 ▀                                   ",
            pos!(0, 6),
        );

        sleep(Duration::from_millis(500));

        render(&app);
    }
}
