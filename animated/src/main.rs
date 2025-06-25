use ::TerimalRtdm::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut app = App::new();
    clear(&mut app);

    raw_mode(true);
    clear(&mut app);

    loop {
        collect_presses(&mut app);

        if Key::o()
            .no_clear()
            .case_sen(true)
            .pressed(&mut app, KeyType::Q)
        {
            break;
        }

        // animation
        loop {
            collect_presses(&mut app);

            Text::new()
                .foreground(Color::Black)
                .style(Style::Bold)
                .show(&mut app, "TerimalRtdm", pos!(0, 0));
            sleep(Duration::from_millis(500));

            if Key::o()
                .no_clear()
                .case_sen(true)
                .pressed(&mut app, KeyType::Q)
            {
                break;
            }
        }

        render(&app);
    }

    raw_mode(false);
}
